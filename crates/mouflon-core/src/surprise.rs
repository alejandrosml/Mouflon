//! Surprises : de temps en temps, un coup de vent traverse l'écran, ou une
//! soucoupe volante vient emprunter un animal pour le déposer ailleurs.
//!
//! - Le vent ([`Gust`]) se lève, souffle par rafales, puis retombe : les
//!   animaux en l'air dérivent, ceux au sol se penchent contre lui, des
//!   feuilles passent, et certains lèvent la tête (réflexe `wind`).
//! - La soucoupe ([`Saucer`]) arrive au-dessus d'un animal posé, l'aspire
//!   dans son faisceau (elle le tient comme la main de l'utilisateur : il
//!   pend et se balance), l'emporte un peu plus loin, le lâche, puis s'en
//!   va. On peut le lui reprendre : elle renonce et repart.
//!
//! Tout reste déterministe : les surprises ne tirent au sort qu'avec la
//! graine qu'on leur donne.

use crate::math::{Rect, Rng, Vec2, lerp};
use crate::pet::Pet;
use crate::world::World;

/// Accélération d'un animal en l'air au plus fort du vent (pt/s²) : il
/// dérive nettement, sans être emporté comme une feuille.
pub const GUST_PEAK: f32 = 900.0;

/// Un coup de vent qui traverse l'écran.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gust {
    /// Sens : 1 vers la droite, -1 vers la gauche.
    pub dir: f32,
    t: f32,
    duration: f32,
    /// Les animaux ont déjà réagi à son arrivée.
    pub(crate) noticed: bool,
}

impl Gust {
    #[must_use]
    pub fn new(dir: f32, duration: f32) -> Self {
        Self {
            dir: if dir < 0.0 { -1.0 } else { 1.0 },
            t: 0.0,
            duration: duration.max(1.0),
            noticed: false,
        }
    }

    /// Force du moment, de 0 à 1 : il se lève, souffle par rafales, retombe.
    #[must_use]
    pub fn strength(&self) -> f32 {
        let u = self.t / self.duration;
        let envelope = smooth(0.0, 0.18, u) * (1.0 - smooth(0.65, 1.0, u));
        let gusts = 0.72 + 0.28 * (self.t * 5.3).sin() * (self.t * 1.7 + 0.4).cos();
        (envelope * gusts).clamp(0.0, 1.0)
    }

    /// Accélération qu'il donne à un animal en l'air (pt/s², signée).
    #[must_use]
    pub fn accel(&self) -> f32 {
        self.dir * GUST_PEAK * self.strength()
    }

    /// Avance de `dt` ; `false` une fois retombé.
    pub(crate) fn step(&mut self, dt: f32) -> bool {
        self.t += dt;
        self.t < self.duration
    }
}

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Taille de la soucoupe (points) : son corps, et la plus grande longueur
/// de son faisceau, sous elle.
pub const SAUCER_WIDTH: f32 = 116.0;
pub const SAUCER_HEIGHT: f32 = 52.0;
pub const BEAM_MAX: f32 = 260.0;

/// Ce que fait la soucoupe.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    /// Elle arrive au-dessus de l'animal.
    Arrive,
    /// Elle se place, les lumières s'affolent.
    Hover,
    /// Le faisceau l'aspire.
    Lift,
    /// Elle l'emporte.
    Carry,
    /// Le faisceau s'éteint : il tombe.
    Drop,
    /// Elle s'en va.
    Leave,
    Gone,
}

/// Ce qu'il faut pour dessiner la soucoupe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaucerView {
    /// Centre de la soucoupe (repère du monde).
    pub pos: Vec2,
    /// Penchée selon sa vitesse (radians, sens horaire).
    pub tilt: f32,
    /// Faisceau : de 0 (éteint) à 1, et jusqu'où il descend sous elle.
    pub beam: f32,
    pub beam_len: f32,
    /// Phase du clignotement de ses lumières (tours).
    pub lights: f32,
}

/// Une soucoupe volante en visite.
#[derive(Clone, Debug)]
pub struct Saucer {
    id: u32,
    phase: Phase,
    t: f32,
    pos: Vec2,
    prev: Vec2,
    vel: Vec2,
    from: Vec2,
    to: Vec2,
    target: u32,
    /// Là où le faisceau tient l'animal (ce qu'on lui a donné au dernier pas).
    grip: Option<Vec2>,
    /// D'où il a été pris, sous la soucoupe (le faisceau le monte de là).
    lift_from: Vec2,
    beam: f32,
    beam_len: f32,
    lights: f32,
    rng: Rng,
}

/// Durées des phases (secondes).
const ARRIVE: f32 = 2.6;
const HOVER: f32 = 0.9;
const LIFT: f32 = 2.0;
const CARRY: f32 = 2.8;
const DROP: f32 = 0.6;
const LEAVE: f32 = 1.7;

/// Hauteur de la soucoupe au-dessus du sol de l'animal qu'elle vient prendre.
const ABOVE: f32 = 190.0;
/// L'animal pend dans le faisceau, ce peu sous la soucoupe (points).
const HANG: f32 = 16.0;

impl Saucer {
    /// Une soucoupe qui vient chercher l'animal `target`, posé en `ground`.
    #[must_use]
    pub fn new(id: u32, target: u32, ground: Vec2, arena: Rect, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        // Elle arrive du côté de l'écran le plus loin de lui, par le haut.
        let from_left = ground.x > arena.x + arena.w * 0.5;
        let x = if from_left {
            arena.left() - SAUCER_WIDTH
        } else {
            arena.right() + SAUCER_WIDTH
        };
        let start = Vec2::new(x, arena.top() + rng.range(30.0, 120.0));
        let hover = Self::above(ground, arena);
        Self {
            id,
            phase: Phase::Arrive,
            t: 0.0,
            pos: start,
            prev: start,
            vel: Vec2::ZERO,
            from: start,
            to: hover,
            target,
            grip: None,
            lift_from: ground,
            beam: 0.0,
            beam_len: 0.0,
            lights: 0.0,
            rng,
        }
    }

    /// Là où elle se tient au-dessus d'un animal posé en `ground`.
    fn above(ground: Vec2, arena: Rect) -> Vec2 {
        let top = arena.top() + SAUCER_HEIGHT * 0.5 + 8.0;
        Vec2::new(
            ground
                .x
                .clamp(arena.left() + SAUCER_WIDTH, arena.right() - SAUCER_WIDTH),
            (ground.y - ABOVE).max(top),
        )
    }

    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// L'animal qu'elle est venue chercher.
    #[must_use]
    pub fn target(&self) -> u32 {
        self.target
    }

    /// Partie, plus rien à dessiner.
    #[must_use]
    pub fn is_gone(&self) -> bool {
        self.phase == Phase::Gone
    }

    #[must_use]
    pub fn view(&self) -> SaucerView {
        SaucerView {
            pos: self.pos,
            tilt: (self.vel.x * 0.0005).clamp(-0.32, 0.32),
            beam: self.beam,
            beam_len: self.beam_len,
            lights: self.lights,
        }
    }

    /// Sa vitesse (pt/s).
    #[must_use]
    pub fn velocity(&self) -> Vec2 {
        self.vel
    }

    fn go(&mut self, phase: Phase, to: Vec2) {
        self.phase = phase;
        self.t = 0.0;
        self.from = self.pos;
        self.to = to;
    }

    /// Elle renonce (l'animal lui a été repris, ou il est parti) : elle s'en va.
    fn give_up(&mut self, arena: Rect) {
        self.grip = None;
        let side = if self.pos.x > arena.x + arena.w * 0.5 {
            1.0
        } else {
            -1.0
        };
        let away = Vec2::new(self.pos.x + side * 300.0, arena.top() - SAUCER_HEIGHT * 3.0);
        self.go(Phase::Leave, away);
    }

    /// Un pas de sa visite ; elle tient l'animal `pet` (s'il est encore là)
    /// comme une main.
    pub(crate) fn step(&mut self, dt: f32, pet: Option<&mut Pet>, world: &World) {
        let arena = world.arena();
        self.t += dt;
        self.lights += dt * if self.beam > 0.05 { 3.2 } else { 1.1 };
        let ease = |t: f32, d: f32| smooth(0.0, 1.0, t / d);
        // L'animal lui a-t-il été repris (la main de l'utilisateur, un autre
        // geste) ? Elle le laisse et s'en va.
        if let (Some(grip), Some(pet)) = (self.grip, pet.as_deref())
            && pet.hold_target() != Some(grip)
        {
            self.give_up(arena);
        }
        let mut pet = pet;
        let gone = pet.is_none()
            || pet
                .as_deref()
                .is_some_and(|p| p.is_leaving() || p.is_finished());
        if gone && !matches!(self.phase, Phase::Leave | Phase::Gone) {
            // Il part (au revoir) : s'il était dans le faisceau, il en tombe.
            if let (Some(_), Some(pet)) = (self.grip, pet.as_deref_mut()) {
                pet.release(world);
            }
            self.give_up(arena);
        }
        match self.phase {
            Phase::Arrive => {
                // Elle suit l'animal s'il bouge encore.
                if let Some(pet) = pet.as_deref() {
                    self.to = Self::above(pet.position(), arena);
                }
                let u = ease(self.t, ARRIVE);
                let arc = (u * std::f32::consts::PI).sin() * -60.0;
                self.pos = self.from.lerp(self.to, u) + Vec2::new(0.0, arc);
                if self.t >= ARRIVE {
                    self.go(Phase::Hover, self.to);
                }
            }
            Phase::Hover => {
                let wobble = Vec2::new((self.t * 9.0).sin() * 3.0, (self.t * 6.0).sin() * 2.0);
                self.pos = self.to + wobble;
                self.beam = (self.t / HOVER).min(1.0) * 0.35;
                if self.t >= HOVER {
                    self.go(Phase::Lift, self.to);
                }
            }
            Phase::Lift => {
                self.pos = self.to + Vec2::new(0.0, (self.t * 6.0).sin() * 1.5);
                self.beam = lerp(0.35, 1.0, (self.t / 0.3).min(1.0));
                if let Some(pet) = pet {
                    let bottom = self.pos + Vec2::new(0.0, SAUCER_HEIGHT * 0.45 + HANG);
                    if self.grip.is_none() {
                        // Pris par le haut du dos.
                        let body = pet.body();
                        let top = body.center - Vec2::new(0.0, body.half.y * 0.85);
                        pet.grab(top, world);
                        self.lift_from = top;
                    }
                    // Il monte doucement, puis de plus en plus vite.
                    let u = (self.t / LIFT).clamp(0.0, 1.0);
                    let rise = u * u * (3.0 - 2.0 * u);
                    let grip = self.lift_from.lerp(bottom, rise)
                        + Vec2::new((self.t * 4.0).sin() * 4.0 * (1.0 - rise), 0.0);
                    pet.drag_to(grip);
                    self.grip = Some(grip);
                    self.beam_len = (self.lift_from.y - bottom.y + pet_height(pet)).max(40.0);
                }
                if self.t >= LIFT {
                    // Un peu plus loin, au moins à 280 points, sans sortir de l'écran.
                    let span = (arena.w - 2.0 * SAUCER_WIDTH).max(1.0);
                    let mut x = arena.left() + SAUCER_WIDTH + self.rng.range(0.0, span);
                    if (x - self.pos.x).abs() < 280.0 {
                        x = if self.pos.x > arena.x + arena.w * 0.5 {
                            (self.pos.x - 320.0).max(arena.left() + SAUCER_WIDTH)
                        } else {
                            (self.pos.x + 320.0).min(arena.right() - SAUCER_WIDTH)
                        };
                    }
                    let y = arena.top() + self.rng.range(90.0, 220.0);
                    self.go(Phase::Carry, Vec2::new(x, y));
                }
            }
            Phase::Carry => {
                let u = ease(self.t, CARRY);
                self.pos = self.from.lerp(self.to, u)
                    + Vec2::new(0.0, (u * std::f32::consts::PI).sin() * -40.0);
                if let Some(pet) = pet {
                    // Il pend dans le faisceau, juste sous elle.
                    let grip = self.pos + Vec2::new(0.0, SAUCER_HEIGHT * 0.45 + HANG);
                    pet.drag_to(grip);
                    self.grip = Some(grip);
                    self.beam_len = pet_height(pet) * 1.4;
                }
                if self.t >= CARRY {
                    self.go(Phase::Drop, self.to);
                }
            }
            Phase::Drop => {
                if let Some(pet) = pet
                    && self.grip.take().is_some()
                {
                    pet.release(world);
                }
                self.pos = self.to + Vec2::new(0.0, (self.t * 7.0).sin() * 2.0);
                self.beam = (1.0 - self.t / DROP).max(0.0);
                if self.t >= DROP {
                    let side = if self.to.x > arena.x + arena.w * 0.5 {
                        1.0
                    } else {
                        -1.0
                    };
                    self.go(
                        Phase::Leave,
                        Vec2::new(self.pos.x + side * 420.0, arena.top() - SAUCER_HEIGHT * 3.0),
                    );
                }
            }
            Phase::Leave => {
                self.beam = (self.beam - dt * 3.0).max(0.0);
                // De plus en plus vite.
                let u = (self.t / LEAVE).clamp(0.0, 1.0);
                self.pos = self.from.lerp(self.to, u * u);
                if self.t >= LEAVE {
                    self.phase = Phase::Gone;
                }
            }
            Phase::Gone => {}
        }
        self.vel = (self.pos - self.prev) * (1.0 / dt.max(1e-4));
        self.prev = self.pos;
    }
}

fn pet_height(pet: &Pet) -> f32 {
    pet.body().half.y * 2.0
}
