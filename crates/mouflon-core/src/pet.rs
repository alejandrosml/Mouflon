//! Simulation d'un animal : physique à pas fixe, comportements et signaux
//! d'animation procédurale (écrasement, démarche, regard, respiration…).
//!
//! Tout est déterministe pour une graine donnée : la même scène rejoue
//! exactement le même mouvement, ce qui rend les tests et l'aperçu fiables.

use std::f32::consts::{PI, TAU};
use std::sync::Arc;

use crate::def::{
    Appearance, Condition, Duration, Motion, PetDef, Pose, Rules, SpawnPoint, SpriteClip,
    Transition,
};
use crate::expr::Expr;
use crate::math::{Rect, Rng, Spring, Vec2, approach, damp, lerp, smoothstep};
use crate::world::{Surface, SurfaceId, SurfaceKind, World};

mod script;

/// Pas de simulation fixe : 120 Hz, la fréquence des écrans ProMotion.
pub const TICK: f32 = 1.0 / 120.0;

const BLINK_TIME: f32 = 0.13;
const MAX_PARTICLES: usize = 64;

/// Ce que l'animal perçoit de l'extérieur à chaque pas.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Input {
    /// Curseur dans le repère du monde.
    pub cursor: Option<Vec2>,
    /// Heure locale décimale (0–24).
    pub local_hour: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    Ground { surface: SurfaceId },
    Air,
    Held,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PetEvent {
    Landed {
        impact: f32,
    },
    Poked,
    Thrown {
        speed: f32,
    },
    /// Heurté par un autre animal (mode collisions).
    Bumped {
        speed: f32,
    },
    /// A rencontré un autre animal et réagit.
    Met {
        other: u32,
    },
}

/// Pause entre deux collisions d'un même animal (secondes) : le temps de se
/// relever avant qu'un autre ne puisse le renverser de nouveau.
const BUMP_COOLDOWN: f32 = 3.5;

/// Pause entre deux rencontres d'un même animal (secondes).
const MEET_COOLDOWN: f32 = 2.5;

/// Durée d'affichage d'une image intermédiaire, au plus (secondes).
const INBETWEEN_TIME: f32 = 0.08;
/// Lissage de la vitesse d'un animal tenu (secondes) : l'élan qu'il garde au
/// lâcher est celui des derniers centièmes de seconde du geste.
const THROW_SMOOTHING: f32 = 0.06;
/// Au revoir : durée du fondu, et durée au-delà de laquelle l'animation
/// d'adieu est écourtée.
const FADE_TIME: f32 = 1.2;
const LEAVE_MAX: f32 = 6.0;

/// Un animal qui tombe se pose sur une surface dont le bord est à moins de
/// cette fraction de sa largeur (ses pattes la touchent encore).
const LANDING_REACH: f32 = 0.15;

/// Besoin de rafraîchissement, pour adapter la cadence d'affichage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Activity {
    Asleep,
    Idle,
    Moving,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleKind {
    /// Poussière soulevée à l'atterrissage.
    Dust,
    /// Bulle de sommeil.
    Bubble,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub kind: ParticleKind,
    /// Position dans le repère du monde.
    pub pos: Vec2,
    pub vel: Vec2,
    pub age: f32,
    pub life: f32,
    pub size: f32,
    pub phase: f32,
}

impl Particle {
    #[must_use]
    pub fn progress(&self) -> f32 {
        (self.age / self.life).clamp(0.0, 1.0)
    }
}

/// Signaux d'animation, tous lissés : le rendu les traduit en transformations.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PoseSignals {
    /// Phase du cycle de marche (radians).
    pub gait_phase: f32,
    /// Poids de la marche et de la course (0–1).
    pub walk: f32,
    pub run: f32,
    /// Poids des poses (0–1).
    pub sit: f32,
    pub sleep: f32,
    pub dangle: f32,
    pub air: f32,
    /// > 0 : écrasé verticalement ; < 0 : étiré.
    pub squash: f32,
    /// Inclinaison du corps (radians, positif vers l'avant).
    pub lean: f32,
    /// Battement des oreilles (radians).
    pub ear: f32,
    /// Queue (radians).
    pub tail: f32,
    /// Paupières : 0 ouvertes, 1 fermées.
    pub blink: f32,
    /// Respiration (-1 à 1).
    pub breath: f32,
    /// Regard dans le repère de l'animal (x vers l'avant, y vers le bas), norme ≤ 1.
    pub look: Vec2,
    /// Balancement quand on le tient (radians, repère de l'écran : positif,
    /// le corps part vers la gauche).
    pub swing: f32,
    /// Culbute (radians, repère de l'écran, sens horaire) : l'animal heurté
    /// roule sur lui-même autour du milieu de son corps.
    pub roll: f32,
}

impl PoseSignals {
    /// Le balancement est donné dans le repère de l'écran ; dans celui d'un
    /// dessin retourné, il change de sens.
    #[must_use]
    pub fn local_swing(&self, facing: f32) -> f32 {
        if facing < 0.0 {
            -self.swing
        } else {
            self.swing
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    /// Hauteur de la surface sous l'animal.
    pub y: f32,
    /// Distance entre les pattes et cette surface.
    pub distance: f32,
}

/// Ce que le rendu a besoin de savoir pour dessiner une image.
#[derive(Clone, Debug)]
pub struct PetView<'a> {
    pub id: u32,
    /// Point d'appui (milieu des pattes), interpolé entre deux pas.
    pub ground: Vec2,
    /// Orientation lissée : 1 vers la droite, -1 vers la gauche, entre les deux pendant un demi-tour.
    pub facing: f32,
    pub pose: PoseSignals,
    pub shadow: Option<Shadow>,
    pub sprite_frame: Option<u32>,
    pub sprite_offset_y: f32,
    pub opacity: f32,
    pub particles: &'a [Particle],
    pub support: Support,
    pub behavior: &'a str,
    /// Tenu : le point par lequel le curseur le tient (il se balance autour).
    pub grip: Option<Vec2>,
}

#[derive(Clone, Copy, Debug)]
struct Hold {
    target: Vec2,
    offset: Vec2,
}

#[derive(Clone, Copy, Debug)]
struct GroundRef {
    surface: SurfaceId,
    x0: f32,
    x1: f32,
}

#[derive(Clone, Debug)]
struct Anim {
    flip: f32,
    gait_phase: f32,
    walk: f32,
    run: f32,
    sit: f32,
    sleep: f32,
    dangle: f32,
    air: f32,
    squash: Spring,
    lean: Spring,
    ear: Spring,
    swing: Spring,
    tail_phase: f32,
    blink: f32,
    blink_timer: f32,
    blink_left: f32,
    double_blink: bool,
    breath_phase: f32,
    look: Vec2,
    glance: Vec2,
    glance_timer: f32,
    bubble_timer: f32,
    /// Prochaine bouffée de poussière quand il court.
    dust_timer: f32,
    /// Culbute : angle courant, angle qu'il reste à parcourir (signé) et vitesse (rad/s).
    roll: f32,
    roll_left: f32,
    roll_rate: f32,
}

impl Anim {
    fn new(facing: f32, rng: &mut Rng) -> Self {
        Self {
            flip: facing,
            gait_phase: 0.0,
            walk: 0.0,
            run: 0.0,
            sit: 0.0,
            sleep: 0.0,
            dangle: 0.0,
            air: 1.0,
            squash: Spring::new(2.6, 0.32),
            lean: Spring::new(2.2, 0.55),
            ear: Spring::new(2.4, 0.22),
            swing: Spring::new(1.3, 0.18),
            tail_phase: rng.range(0.0, TAU),
            blink: 0.0,
            blink_timer: rng.range(1.0, 4.0),
            blink_left: 0.0,
            double_blink: false,
            breath_phase: rng.range(0.0, TAU),
            look: Vec2::new(0.3, 0.0),
            glance: Vec2::new(0.3, 0.0),
            glance_timer: 0.0,
            bubble_timer: 1.0,
            dust_timer: 0.0,
            roll: 0.0,
            roll_left: 0.0,
            roll_rate: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pet {
    id: u32,
    def: Arc<PetDef>,
    rng: Rng,
    pos: Vec2,
    prev_pos: Vec2,
    vel: Vec2,
    prev_vel_x: f32,
    facing: f32,
    support: Support,
    ground_ref: Option<GroundRef>,
    walking_off: bool,
    hold: Option<Hold>,
    behavior: usize,
    behavior_time: f32,
    behavior_length: f32,
    clip_step: usize,
    clip_timer: f32,
    /// Nombre d'images de la séquence en cours (les répétitions peuvent être tirées au hasard).
    clip_total: usize,
    /// Nombre aléatoire fixe pour toute la vie de l'animal (`randS` de desktopPet).
    rand_s: f32,
    /// Écran sous l'animal, pour les expressions desktopPet.
    screen: Option<(Rect, Rect)>,
    /// Déplacement par pas `[premier, dernier]` de l'animation scriptée en cours
    /// (x vers l'avant, y vers le bas), en points.
    script: [Vec2; 2],
    /// Variante de couleur de la planche de sprites.
    skin: u32,
    /// Personnage secondaire : sa profondeur (0 pour un animal principal).
    child_depth: u8,
    /// Personnage secondaire arrivé au bout de son rôle, à retirer.
    finished: bool,
    /// Personnages secondaires à faire apparaître, récupérés par le troupeau.
    children: Vec<ChildRequest>,
    /// Envoyé en arrière par une collision : il se pose sans se retourner.
    knocked: bool,
    /// Temps avant de pouvoir être heurté de nouveau.
    bump_cooldown: f32,
    /// Temps avant de pouvoir rencontrer un autre animal.
    meet_cooldown: f32,
    /// En train de dire au revoir.
    leaving: Option<Leaving>,
    anim: Anim,
    particles: Vec<Particle>,
    events: Vec<PetEvent>,
    shadow: Option<Shadow>,
}

/// Au revoir : l'animal joue son animation d'adieu, puis s'efface.
#[derive(Clone, Copy, Debug)]
struct Leaving {
    /// Temps écoulé depuis l'au revoir.
    time: f32,
    /// Opacité restante, une fois le fondu commencé.
    fade: Option<f32>,
}

/// Demande d'apparition d'un personnage secondaire.
#[derive(Clone, Copy, Debug)]
struct ChildRequest {
    behavior: usize,
    /// Point d'appui (milieu du bas de l'image).
    pos: Vec2,
    facing: f32,
    depth: u8,
    skin: u32,
}

impl Pet {
    /// Fait apparaître un animal en haut de l'écran (comme le mouton d'origine),
    /// à l'abscisse donnée ou au hasard.
    #[must_use]
    pub fn spawn(id: u32, def: Arc<PetDef>, seed: u64, world: &World, x: Option<f32>) -> Self {
        let arena = world.arena();
        let mut rng = Rng::new(seed ^ (u64::from(id) << 32));
        if x.is_none()
            && let Some(i) = rng.pick_weighted(def.spawns.iter().map(|s| s.weight))
        {
            let spawn = def.spawns[i].clone();
            return Self::from_spawn_point(id, def, rng, world, &spawn);
        }
        let margin = def.physics.width;
        let x = x.unwrap_or_else(|| {
            rng.range(
                arena.left() + margin,
                (arena.right() - margin).max(arena.left() + margin),
            )
        });
        let pos = Vec2::new(x, arena.top() + def.physics.height + 2.0);
        Self::at(id, def, rng, pos, world)
    }

    /// Fait apparaître un animal à une position précise (en l'air).
    #[must_use]
    pub fn spawn_at(id: u32, def: Arc<PetDef>, seed: u64, world: &World, pos: Vec2) -> Self {
        let rng = Rng::new(seed ^ (u64::from(id) << 32));
        Self::at(id, def, rng, pos, world)
    }

    /// Apparition à un point desktopPet.
    fn from_spawn_point(
        id: u32,
        def: Arc<PetDef>,
        rng: Rng,
        world: &World,
        spawn: &SpawnPoint,
    ) -> Self {
        let mut pet = Self::bare(id, def, rng, Vec2::ZERO);
        pet.place_at(world, spawn);
        pet
    }

    /// Place l'animal à un point d'apparition desktopPet et lance sa première
    /// animation. Le point peut être hors de l'écran : le mouton entre alors en
    /// marchant par le bord, ou tombe du haut. Un animal retourné apparaît en
    /// miroir, comme dans desktopPet.
    fn place_at(&mut self, world: &World, spawn: &SpawnPoint) {
        self.update_screen(world);
        let p = self.def.physics;
        let (frame, _) = self.screen.unwrap_or_default();
        // Les scènes à plusieurs personnages (bain, mouton noir…) ne se placent
        // correctement que dans le sens d'origine du dessin.
        if !self.def.behaviors[spawn.next].children.is_empty() {
            self.facing = -1.0;
        }
        let mut left = self.eval(&spawn.x).trunc() as f32;
        let top = self.eval(&spawn.y).trunc() as f32;
        if self.facing > 0.0 {
            left = frame.w - left - p.width;
        }
        self.pos = Vec2::new(frame.x + left + p.width * 0.5, frame.y + top + p.height);
        self.prev_pos = self.pos;
        self.vel = Vec2::ZERO;
        self.hold = None;
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
        self.anim.flip = self.facing;
        // Déjà posé sur le sol (même hors de l'écran) ou sur une fenêtre ?
        let y = self.pos.y;
        let x = self.pos.x;
        let ground = world
            .floor_under(x)
            .filter(|f| (f.y - y).abs() < 1.5)
            .or_else(|| {
                world.surfaces().iter().find(|s| {
                    s.kind == SurfaceKind::Window && (s.y - y).abs() < 1.5 && s.contains_x(x, 0.0)
                })
            })
            .copied();
        if let Some(surface) = ground {
            self.pos.y = surface.y;
            self.prev_pos = self.pos;
            self.set_ground(&surface);
            self.anim.air = 0.0;
        }
        self.enter(spawn.next);
    }

    /// Nouvelle apparition (desktopPet : animal sorti de l'écran, ou sans suite).
    fn respawn(&mut self, world: &World) {
        let def = Arc::clone(&self.def);
        if let Some(i) = self.rng.pick_weighted(def.spawns.iter().map(|s| s.weight)) {
            self.place_at(world, &def.spawns[i]);
            return;
        }
        let arena = world.arena();
        let margin = def.physics.width;
        let x = self.rng.range(
            arena.left() + margin,
            (arena.right() - margin).max(arena.left() + margin),
        );
        self.pos = Vec2::new(x, arena.top() + def.physics.height + 2.0);
        self.prev_pos = self.pos;
        self.vel = Vec2::ZERO;
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
        let first = self
            .choose(&def.reflexes.spawn, world, &Input::default())
            .or(def.default_air())
            .unwrap_or(def.default_ground());
        self.enter(first);
    }

    /// Personnage secondaire (desktopPet `child`), né d'un autre animal.
    fn child(id: u32, def: Arc<PetDef>, seed: u64, world: &World, request: ChildRequest) -> Self {
        let rng = Rng::new(seed ^ (u64::from(id) << 32) ^ 0xC41D);
        let mut pet = Self::bare(id, def, rng, request.pos);
        pet.child_depth = request.depth;
        pet.skin = request.skin;
        pet.facing = request.facing;
        pet.anim.flip = request.facing;
        pet.update_screen(world);
        let (x, y) = (pet.pos.x, pet.pos.y);
        let ground = world
            .floor_under(x)
            .filter(|f| (f.y - y).abs() < 1.5)
            .or_else(|| {
                world.surfaces().iter().find(|s| {
                    s.kind == SurfaceKind::Window && (s.y - y).abs() < 1.5 && s.contains_x(x, 0.0)
                })
            })
            .copied();
        if let Some(surface) = ground {
            pet.pos.y = surface.y;
            pet.prev_pos = pet.pos;
            pet.set_ground(&surface);
            pet.anim.air = 0.0;
        }
        pet.enter(request.behavior);
        pet
    }

    fn set_ground(&mut self, surface: &Surface) {
        self.support = Support::Ground {
            surface: surface.id,
        };
        self.ground_ref = Some(GroundRef {
            surface: surface.id,
            x0: surface.x0,
            x1: surface.x1,
        });
        self.walking_off = false;
    }

    fn detach(&mut self) {
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
    }

    /// La fenêtre sous l'animal a bougé (sans changer de largeur) : il voyage avec elle.
    fn ride(&mut self, surface: &Surface) {
        if let Some(r) = self.ground_ref
            && r.surface == surface.id
        {
            let dx0 = surface.x0 - r.x0;
            let dx1 = surface.x1 - r.x1;
            if (dx0 - dx1).abs() < 0.5 && dx0 != 0.0 {
                self.pos.x += dx0;
            }
        }
        self.ground_ref = Some(GroundRef {
            surface: surface.id,
            x0: surface.x0,
            x1: surface.x1,
        });
    }

    fn bare(id: u32, def: Arc<PetDef>, mut rng: Rng, pos: Vec2) -> Self {
        let facing = if rng.chance(0.5) { 1.0 } else { -1.0 };
        let anim = Anim::new(facing, &mut rng);
        let rand_s = (10 + rng.next_u32() % 80) as f32;
        let skin = match &def.appearance {
            Appearance::Sprites(sheet) if sheet.skins > 1 => rng.next_u32() % sheet.skins,
            _ => 0,
        };
        Self {
            id,
            def,
            rng,
            pos,
            prev_pos: pos,
            vel: Vec2::ZERO,
            prev_vel_x: 0.0,
            facing,
            support: Support::Air,
            ground_ref: None,
            walking_off: false,
            hold: None,
            behavior: 0,
            behavior_time: 0.0,
            behavior_length: f32::INFINITY,
            clip_step: 0,
            clip_timer: 0.0,
            clip_total: 1,
            rand_s,
            screen: None,
            script: [Vec2::ZERO; 2],
            skin,
            child_depth: 0,
            finished: false,
            children: Vec::new(),
            knocked: false,
            bump_cooldown: 0.0,
            meet_cooldown: 0.0,
            leaving: None,
            anim,
            particles: Vec::new(),
            events: Vec::new(),
            shadow: None,
        }
    }

    fn at(id: u32, def: Arc<PetDef>, rng: Rng, pos: Vec2, world: &World) -> Self {
        let mut pet = Self::bare(id, def, rng, pos);
        pet.update_screen(world);
        let def = Arc::clone(&pet.def);
        let first = pet
            .choose(&def.reflexes.spawn, world, &Input::default())
            .or(def.default_air())
            .unwrap_or(def.default_ground());
        pet.enter(first);
        pet
    }

    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub fn def(&self) -> &Arc<PetDef> {
        &self.def
    }

    #[must_use]
    pub fn position(&self) -> Vec2 {
        self.pos
    }

    #[must_use]
    pub fn velocity(&self) -> Vec2 {
        self.vel
    }

    #[must_use]
    pub fn support(&self) -> Support {
        self.support
    }

    #[must_use]
    pub fn behavior_name(&self) -> &str {
        &self.def.behaviors[self.behavior].name
    }

    #[must_use]
    pub fn is_held(&self) -> bool {
        self.support == Support::Held
    }

    /// Personnage secondaire d'une scène desktopPet : il ne s'attrape pas.
    #[must_use]
    pub fn is_child(&self) -> bool {
        self.child_depth > 0
    }

    /// Le personnage secondaire a fini son rôle.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn take_events(&mut self) -> Vec<PetEvent> {
        std::mem::take(&mut self.events)
    }

    /// Donne de quoi choisir la cadence d'affichage.
    #[must_use]
    pub fn activity(&self) -> Activity {
        let grounded = matches!(self.support, Support::Ground { .. });
        let settling = self.anim.squash.velocity.abs() > 0.02
            || (self.anim.flip - self.facing).abs() > 0.01
            || self.anim.swing.value.abs() > 0.01
            || self.anim.walk > 0.02
            || self.anim.roll_left != 0.0;
        let sprites = self.def.behaviors[self.behavior].clip.is_some();
        // Les bulles de sommeil montent lentement : elles n'exigent pas la pleine cadence.
        let dust = self.particles.iter().any(|p| p.kind == ParticleKind::Dust);
        if !grounded || self.vel.x.abs() > 0.5 || settling || dust {
            Activity::Moving
        } else if sprites {
            // Les images changent à leur rythme (30 ms au plus court) : inutile de courir.
            Activity::Idle
        } else if self.anim.sleep > 0.9 {
            Activity::Asleep
        } else {
            Activity::Idle
        }
    }

    // ----------------------------------------------------------------- entrées

    /// Le curseur attrape l'animal.
    pub fn grab(&mut self, cursor: Vec2, world: &World) {
        if self.is_child() || self.leaving.is_some() {
            return;
        }
        self.hold = Some(Hold {
            target: cursor,
            offset: cursor - self.pos,
        });
        self.support = Support::Held;
        self.vel = Vec2::ZERO;
        self.ground_ref = None;
        self.walking_off = false;
        self.knocked = false;
        self.squash_kick(-0.9);
        let def = Arc::clone(&self.def);
        let next = self
            .choose(&def.reflexes.held, world, &Input::default())
            .or_else(|| def.behaviors.iter().position(|b| b.motion == Motion::Held));
        if let Some(next) = next {
            self.enter(next);
        }
    }

    pub fn drag_to(&mut self, cursor: Vec2) {
        if let Some(hold) = &mut self.hold {
            hold.target = cursor;
        }
    }

    /// Le curseur relâche l'animal, qui garde son élan.
    pub fn release(&mut self, world: &World) {
        if self.support != Support::Held {
            return;
        }
        self.hold = None;
        self.vel = self.vel.clamp_length(self.def.physics.max_throw_speed);
        self.support = Support::Air;
        self.events.push(PetEvent::Thrown {
            speed: self.vel.length(),
        });
        let def = Arc::clone(&self.def);
        // Lâché sans élan, il tombe simplement (desktopPet : `fall` plutôt que `toss`).
        let gentle = self.vel.length() < 0.25 * def.physics.hard_landing_speed;
        let first = if gentle {
            &def.reflexes.fall
        } else {
            &def.reflexes.thrown
        };
        let next = self
            .choose(first, world, &Input::default())
            .or_else(|| self.choose(&def.reflexes.thrown, world, &Input::default()))
            .or_else(|| self.choose(&def.reflexes.fall, world, &Input::default()))
            .or(def.default_air());
        if let Some(next) = next {
            self.enter(next);
        }
    }

    /// Impose un comportement par son nom (outils, tests, menu de débogage).
    /// Renvoie `false` si le nom est inconnu.
    pub fn force(&mut self, name: &str) -> bool {
        match self.def.behavior_index(name) {
            Some(index) => {
                self.enter(index);
                true
            }
            None => false,
        }
    }

    /// Change la taille de l'animal sans l'interrompre. `def` est la même
    /// définition à une autre échelle (voir [`PetDef::scaled`]) et `ratio`
    /// le rapport de la nouvelle taille à l'ancienne ; ses pattes restent
    /// où elles sont. Renvoie `false` (et ne change rien) si `def` n'a pas
    /// les mêmes comportements.
    pub fn rescale(&mut self, def: Arc<PetDef>, ratio: f32) -> bool {
        let same = def.behaviors.len() == self.def.behaviors.len()
            && def
                .behaviors
                .iter()
                .zip(&self.def.behaviors)
                .all(|(a, b)| a.name == b.name);
        if !same || ratio.is_nan() || ratio <= 0.0 {
            return false;
        }
        self.def = def;
        self.vel *= ratio;
        self.script = [self.script[0] * ratio, self.script[1] * ratio];
        if let Some(hold) = &mut self.hold {
            hold.offset *= ratio;
        }
        true
    }

    /// Tourne l'animal d'un coup vers la droite (1) ou la gauche (-1) : outils et tests.
    pub fn face(&mut self, facing: f32) {
        self.facing = if facing < 0.0 { -1.0 } else { 1.0 };
        self.anim.flip = self.facing;
    }

    /// Peut-il être heurté par un autre animal (mode collisions) ?
    #[must_use]
    pub fn can_bump(&self) -> bool {
        !self.is_child()
            && !self.finished
            && !self.is_held()
            && self.leaving.is_none()
            && self.bump_cooldown <= 0.0
    }

    /// En train de dire au revoir (il s'efface ensuite).
    #[must_use]
    pub fn is_leaving(&self) -> bool {
        self.leaving.is_some()
    }

    /// Dit au revoir : l'animal joue son animation d'adieu s'il en a une et
    /// qu'il est posé, puis s'efface et quitte le troupeau.
    pub fn leave(&mut self, world: &World, input: &Input) {
        if self.leaving.is_some() || self.finished {
            return;
        }
        let mut leaving = Leaving {
            time: 0.0,
            fade: Some(1.0),
        };
        if self.hold.is_some() {
            self.release(world);
        } else if matches!(self.support, Support::Ground { .. }) && !self.is_child() {
            let def = Arc::clone(&self.def);
            if let Some(next) = self.choose(&def.reflexes.leave, world, input) {
                self.enter(next);
                leaving.fade = None;
            }
        }
        self.leaving = Some(leaving);
    }

    /// Tous ensemble : l'animal joue son comportement `together` s'il en a
    /// un et qu'il est libre (posé, ni tenu ni en partance). Renvoie `true`
    /// s'il le joue.
    pub fn together(&mut self, world: &World, input: &Input) -> bool {
        if self.is_child()
            || self.leaving.is_some()
            || !matches!(self.support, Support::Ground { .. })
        {
            return false;
        }
        let def = Arc::clone(&self.def);
        match self.choose(&def.reflexes.together, world, input) {
            Some(next) => {
                self.enter(next);
                true
            }
            None => false,
        }
    }

    /// Peut-il en rencontrer un autre (hors mode collisions) ?
    fn can_meet(&self) -> bool {
        !self.is_child()
            && !self.finished
            && self.hold.is_none()
            && self.leaving.is_none()
            && self.meet_cooldown <= 0.0
            && matches!(self.support, Support::Ground { .. })
    }

    /// Réactions possibles à une rencontre pendant le comportement en cours.
    /// Un animal desktopPet suit son animation ; un animal Mouflon prend le
    /// réflexe `meet` quand il est en chemin, et ne réagit au repos que si
    /// son comportement le prévoit.
    fn meet_reactions<'d>(&self, def: &'d PetDef) -> &'d [Transition] {
        let behavior = &def.behaviors[self.behavior];
        if !behavior.on_meet.is_empty() || def.rules == Rules::DesktopPet {
            &behavior.on_meet
        } else if self.vel.x.abs() > 1.0 {
            &def.reflexes.meet
        } else {
            &[]
        }
    }

    /// Le côté d'une fenêtre barre le chemin de l'animal qui avance de `dx` :
    /// renvoie sa réaction et l'abscisse où il s'arrête, ou rien (il passe
    /// alors devant la fenêtre). Un animal Mouflon en l'air ne réagit que si
    /// son comportement le prévoit.
    fn window_side(
        &mut self,
        dx: f32,
        world: &World,
        input: &Input,
        grounded: bool,
    ) -> Option<(usize, f32)> {
        let def = Arc::clone(&self.def);
        let p = def.physics;
        let behavior = &def.behaviors[self.behavior];
        let list: &[Transition] = if !behavior.on_window_side.is_empty() {
            &behavior.on_window_side
        } else if grounded {
            &def.reflexes.window_side
        } else {
            &[]
        };
        if list.is_empty() || dx == 0.0 {
            return None;
        }
        let dir = dx.signum();
        let half = p.width * 0.5;
        let front = self.pos.x + dir * half;
        let wall = world.wall_crossed(front, front + dx, self.pos.y - p.height, self.pos.y)?;
        // Pas la place de se tenir entre la fenêtre et le bord de l'écran : on
        // l'ignore (desktopPet).
        let arena = world.arena();
        let room = if dir > 0.0 {
            wall.x - arena.left()
        } else {
            arena.right() - wall.x
        };
        if room < p.width {
            return None;
        }
        let x = wall.x - dir * half;
        let next = self.choose(list, world, input)?;
        Some((next, x))
    }

    /// Heurté par un autre animal : il part en arrière avec l'élan `push`
    /// (pt/s) et fait une culbute, puis se pose sans se retourner. Renvoie
    /// `false` si l'animal ne sait pas voler (aucun comportement de chute
    /// physique) : il fait alors simplement demi-tour.
    pub fn knock(&mut self, push: Vec2, world: &World, input: &Input) -> bool {
        if !self.can_bump() {
            return false;
        }
        self.bump_cooldown = BUMP_COOLDOWN;
        let def = Arc::clone(&self.def);
        // Une chute physique, pour que l'élan compte (pas une chute image par image).
        let flying = |i: &usize| def.behaviors[*i].motion == Motion::Fall;
        let next = self
            .choose(&def.reflexes.bump, world, input)
            .filter(flying)
            .or_else(|| {
                self.choose(&def.reflexes.thrown, world, input)
                    .filter(flying)
            })
            .or(def.default_air());
        let Some(next) = next else {
            if push.x * self.facing < 0.0 {
                self.facing = -self.facing;
            }
            return false;
        };
        self.vel = push;
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
        self.knocked = true;
        self.events.push(PetEvent::Bumped {
            speed: push.length(),
        });
        self.squash_kick(-0.7);
        self.anim.ear.kick(-4.0);
        if !def.drawn().roll {
            // Un tour complet en roulant comme une roue (vers la droite : sens
            // horaire), réparti sur la durée du vol.
            let flight = (2.0 * (-push.y).max(60.0) / def.physics.gravity.max(1.0)).max(0.2);
            let sign = if push.x >= 0.0 { 1.0 } else { -1.0 };
            self.anim.roll_left += sign * TAU;
            self.anim.roll_rate = TAU / flight;
        }
        self.enter(next);
        true
    }

    /// Écrasement ou étirement, sauf si les images le dessinent déjà.
    fn squash_kick(&mut self, amount: f32) {
        if !self.def.drawn().squash {
            self.anim.squash.kick(amount);
        }
    }

    /// Un clic sans glisser.
    pub fn poke(&mut self, world: &World, input: &Input) {
        if self.is_child() || self.leaving.is_some() {
            return;
        }
        self.events.push(PetEvent::Poked);
        self.squash_kick(0.8);
        self.anim.ear.kick(-3.0);
        if matches!(self.support, Support::Ground { .. }) {
            let def = Arc::clone(&self.def);
            if let Some(next) = self.choose(&def.reflexes.poke, world, input) {
                self.enter(next);
            }
        }
    }

    // ------------------------------------------------------------- simulation

    pub fn step(&mut self, dt: f32, world: &World, input: &Input) {
        self.prev_pos = self.pos;
        self.bump_cooldown = (self.bump_cooldown - dt).max(0.0);
        self.meet_cooldown = (self.meet_cooldown - dt).max(0.0);
        if let Some(leaving) = &mut self.leaving {
            leaving.time += dt;
            if leaving.fade.is_none() && leaving.time >= LEAVE_MAX {
                leaving.fade = Some(1.0);
            }
            if let Some(fade) = &mut leaving.fade {
                *fade -= dt / FADE_TIME;
                if *fade <= 0.0 {
                    self.finished = true;
                }
            }
        }
        self.update_screen(world);
        let scripted = matches!(self.def.behaviors[self.behavior].motion, Motion::Script(_));
        match self.support {
            Support::Held => self.step_held(dt),
            _ if scripted => self.step_script(dt, world, input),
            Support::Air => self.step_air(dt, world, input),
            Support::Ground { surface } => self.step_ground(dt, world, input, surface),
        }
        self.step_behavior(dt, world, input);
        self.step_anim(dt, input);
        self.step_particles(dt);
        self.update_shadow(world);
        self.prev_vel_x = self.vel.x;
    }

    fn step_ground(&mut self, dt: f32, world: &World, input: &Input, surface_id: SurfaceId) {
        let p = self.def.physics;
        let Some(surface) = world.surface(surface_id).copied() else {
            self.lift_off(world, input);
            return;
        };

        self.ride(&surface);
        self.pos.y = surface.y;

        let motion = &self.def.behaviors[self.behavior].motion;
        let desired = match *motion {
            Motion::Walk { speed } | Motion::Run { speed } => speed * self.facing,
            _ => 0.0,
        };
        let accel = if desired == 0.0 {
            p.ground_friction
        } else {
            p.walk_accel
        };
        self.vel.x = approach(self.vel.x, desired, accel * dt);
        self.vel.y = 0.0;
        // Le côté d'une fenêtre barre le chemin : il réagit, ou passe devant.
        if desired != 0.0
            && let Some((next, x)) = self.window_side(self.vel.x * dt, world, input, true)
        {
            self.pos.x = x;
            self.vel.x = 0.0;
            self.enter(next);
            return;
        }
        self.pos.x += self.vel.x * dt;

        // Bords de la surface : le museau prévient avant que les pattes n'y arrivent,
        // pour que l'animal se retourne sans déborder sur une fenêtre voisine.
        // Le sol d'un écran voisin à la même hauteur prolonge le chemin.
        let slide_margin = p.width * 0.2;
        let nose = self.pos.x + self.facing * p.width * 0.32;
        let next_floor = |x: f32| {
            world
                .floor_at(x, surface.y)
                .filter(|f| surface.kind == SurfaceKind::Floor && (f.y - surface.y).abs() < 1.0)
                .copied()
        };
        let heading_out = ((nose > surface.x1 && self.vel.x > 0.0)
            || (nose < surface.x0 && self.vel.x < 0.0))
            && next_floor(nose).is_none();
        if !self.walking_off && desired != 0.0 && heading_out {
            self.on_edge(world, input, &surface);
        } else if !surface.contains_x(self.pos.x, 0.0)
            && let Some(next) = next_floor(self.pos.x)
        {
            self.set_ground(&next);
        } else if self.pos.x > surface.x1 + slide_margin || self.pos.x < surface.x0 - slide_margin {
            self.lift_off(world, input);
            return;
        }

        // Bords de l'écran.
        let arena = world.arena();
        let half = p.width * 0.5;
        if self.pos.x - half < arena.left() {
            self.pos.x = arena.left() + half;
            if self.vel.x < 0.0 {
                self.vel.x = 0.0;
                self.on_wall(world, input);
            }
        } else if self.pos.x + half > arena.right() {
            self.pos.x = arena.right() - half;
            if self.vel.x > 0.0 {
                self.vel.x = 0.0;
                self.on_wall(world, input);
            }
        }
    }

    fn step_air(&mut self, dt: f32, world: &World, input: &Input) {
        let p = self.def.physics;
        self.vel.y = (self.vel.y + p.gravity * dt).min(p.terminal_velocity);
        self.vel.x *= (-p.air_drag * dt).exp();
        let from = self.pos;
        let mut to = self.pos + self.vel * dt;

        let arena = world.arena();
        let half = p.width * 0.5;
        if to.x - half < arena.left() {
            to.x = arena.left() + half;
            self.vel.x = self.vel.x.abs() * 0.45;
        } else if to.x + half > arena.right() {
            to.x = arena.right() - half;
            self.vel.x = -self.vel.x.abs() * 0.45;
        }
        // Lancé vers le haut, il peut monter jusqu'au bord de l'écran, en
        // passant derrière la barre des menus (et l'encoche), puis retombe.
        let ceiling = world.bounds().top() + p.height;
        if to.y < ceiling {
            to.y = ceiling;
            self.vel.y = self.vel.y.max(0.0);
        }

        if self.vel.y >= 0.0 {
            if let Some(s) = world
                .landing_surface(to.x, p.width * LANDING_REACH, from.y, to.y)
                .copied()
            {
                self.pos = Vec2::new(to.x, s.y);
                self.land(&s, world, input);
                return;
            }
            if to.y > arena.bottom() + 1.0
                && let Some(floor) = world.floor_under(to.x).copied()
            {
                self.pos = Vec2::new(to.x.clamp(floor.x0, floor.x1), floor.y);
                self.land(&floor, world, input);
                return;
            }
        }
        self.pos = to;
    }

    fn step_held(&mut self, dt: f32) {
        let Some(hold) = &self.hold else {
            self.support = Support::Air;
            return;
        };
        // Comme une fenêtre qu'on déplace : le point par lequel on l'a pris
        // reste sous le curseur, sans retard ni glissement.
        let target = hold.target - hold.offset;
        // Sa vitesse est celle du geste, lissée sur quelques centièmes de
        // seconde : elle donne l'élan au lâcher (rien si la main s'est
        // arrêtée) et le balancement du dessin.
        if dt > 0.0 {
            let gesture = (target - self.pos) * (1.0 / dt);
            self.vel = self.vel.lerp(gesture, 1.0 - (-dt / THROW_SMOOTHING).exp());
        }
        self.pos = target;
    }

    fn land(&mut self, surface: &Surface, world: &World, input: &Input) {
        let p = self.def.physics;
        let impact = self.vel.y.max(0.0);
        self.events.push(PetEvent::Landed { impact });
        self.squash_kick((impact * 0.0016).min(3.2));
        self.anim.ear.kick((impact * 0.004).min(6.0));
        if impact > 420.0 {
            self.emit_dust(impact);
        }
        if impact > p.bounce_min_speed && p.bounce > 0.0 {
            self.vel.y = -impact * p.bounce;
            self.vel.x *= 0.75;
            return;
        }
        // Lancé, il se pose tourné vers où il allait. Heurté, il est parti à
        // reculons et garde son sens ; un animal desktopPet, qui n'a pas d'idée
        // à lui pour aller ailleurs, foncerait de nouveau sur l'autre : il
        // tourne le dos au choc.
        if self.def.behaviors[self.behavior].motion == Motion::Fall
            && self.vel.x.abs() > 60.0
            && (!self.knocked || self.def.rules == Rules::DesktopPet)
        {
            self.facing = self.vel.x.signum();
        }
        self.knocked = false;
        // La culbute s'achève vite si le sol arrive avant la fin du tour.
        if self.anim.roll_left != 0.0 {
            self.anim.roll_rate = self.anim.roll_rate.max(self.anim.roll_left.abs() / 0.15);
        }
        self.vel.y = 0.0;
        self.vel.x *= 0.5;
        self.set_ground(surface);
        let def = Arc::clone(&self.def);
        let own = &def.behaviors[self.behavior].on_land;
        let hard = impact >= p.hard_landing_speed && !def.reflexes.land_hard.is_empty();
        let list = if !own.is_empty() {
            own
        } else if hard {
            &def.reflexes.land_hard
        } else {
            &def.reflexes.land
        };
        let next = self
            .choose(list, world, input)
            .unwrap_or(def.default_ground());
        self.enter(next);
    }

    fn lift_off(&mut self, world: &World, input: &Input) {
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
        let def = Arc::clone(&self.def);
        let behavior = &def.behaviors[self.behavior];
        if !behavior.motion.is_airborne() {
            let list = if behavior.on_fall.is_empty() {
                &def.reflexes.fall
            } else {
                &behavior.on_fall
            };
            let next = self.choose(list, world, input).or(def.default_air());
            if let Some(next) = next {
                self.enter(next);
            }
        }
    }

    fn on_edge(&mut self, world: &World, input: &Input, surface: &Surface) {
        let def = Arc::clone(&self.def);
        let own = &def.behaviors[self.behavior].on_edge;
        let list = if own.is_empty() {
            &def.reflexes.edge
        } else {
            own
        };
        match self.choose(list, world, input) {
            Some(next) => self.enter(next),
            None if def.behaviors[self.behavior].walk_off_edges => {}
            None => self.facing = -self.facing,
        }
        let reach = self.def.physics.width * 0.32;
        let nose = self.pos.x + self.facing * reach;
        let outward =
            (nose > surface.x1 && self.facing > 0.0) || (nose < surface.x0 && self.facing < 0.0);
        let moving = matches!(
            def.behaviors[self.behavior].motion,
            Motion::Walk { .. } | Motion::Run { .. }
        );
        if outward && moving {
            // Il a choisi de continuer : il va tomber du rebord.
            self.walking_off = true;
        } else {
            // Garde le museau sur le rebord (sans dépasser le milieu si le rebord est étroit).
            let lo = (surface.x0 + reach).min(surface.x0 + surface.width() * 0.5);
            let hi = (surface.x1 - reach).max(surface.x1 - surface.width() * 0.5);
            self.pos.x = self.pos.x.clamp(lo, hi);
            if !matches!(def.behaviors[self.behavior].motion, Motion::Jump { .. }) {
                self.vel.x = 0.0;
            }
        }
    }

    fn on_wall(&mut self, world: &World, input: &Input) {
        let def = Arc::clone(&self.def);
        let own = &def.behaviors[self.behavior].on_wall;
        let list = if own.is_empty() {
            &def.reflexes.wall
        } else {
            own
        };
        match self.choose(list, world, input) {
            Some(next) => self.enter(next),
            None => self.facing = -self.facing,
        }
    }

    fn step_behavior(&mut self, dt: f32, world: &World, input: &Input) {
        self.behavior_time += dt;
        let def = Arc::clone(&self.def);
        let behavior = &def.behaviors[self.behavior];
        if behavior.clip.is_some() {
            self.clip_timer += dt;
            loop {
                let interval = self.clip_interval();
                if self.clip_timer < interval {
                    break;
                }
                self.clip_timer -= interval;
                self.clip_step += 1;
            }
        }
        let desktoppet = def.rules == Rules::DesktopPet;
        let finished = match behavior.duration {
            Duration::Range(..) => self.behavior_time >= self.behavior_length,
            // desktopPet : T pas, puis un dernier qui garde la dernière image.
            Duration::Clip => {
                behavior.clip.is_none()
                    || self.clip_step >= self.clip_total + usize::from(desktoppet)
            }
            Duration::Forever => false,
        };
        if !finished || self.support == Support::Held || self.finished {
            return;
        }
        // L'animation d'adieu est finie : il s'efface sur sa dernière image.
        if let Some(leaving) = &mut self.leaving {
            leaving.fade.get_or_insert(1.0);
            return;
        }
        if behavior.flip_at_end {
            self.facing = -self.facing;
            // Les images ont déjà montré le demi-tour : pas de transition en plus.
            self.anim.flip = self.facing;
        }
        let scripted = matches!(behavior.motion, Motion::Script(_));
        if scripted && self.outside(world) {
            if self.is_child() {
                self.finished = true;
            } else {
                self.respawn(world);
            }
            return;
        }
        match self.choose(&behavior.next, world, input) {
            Some(next) => self.enter(next),
            // Un personnage secondaire s'en va ; un animal desktopPet réapparaît.
            None if self.is_child() => self.finished = true,
            None if scripted && desktoppet => self.respawn(world),
            None => {
                let next = if self.support == Support::Air {
                    self.behavior
                } else {
                    def.default_ground()
                };
                self.enter(next);
            }
        }
    }

    /// Durée du pas en cours d'une séquence de sprites (interpolée sur T pas).
    fn clip_interval(&self) -> f32 {
        let Some(clip) = &self.def.behaviors[self.behavior].clip else {
            return f32::INFINITY;
        };
        let t = (self.clip_step as f32 / self.clip_total.max(1) as f32).min(1.0);
        lerp(clip.interval_start, clip.interval_end, t).max(0.016)
    }

    /// Entièrement sorti des écrans (desktopPet le fait alors réapparaître).
    fn outside(&self, world: &World) -> bool {
        let p = self.def.physics;
        let b = world.bounds();
        let left = self.pos.x - p.width * 0.5;
        let top = self.pos.y - p.height;
        left < b.left() - p.width
            || left > b.right()
            || top < b.top() - p.height
            || top > b.bottom()
    }

    fn enter(&mut self, index: usize) {
        let def = Arc::clone(&self.def);
        let mut index = index.min(def.behaviors.len() - 1);
        let grounded = matches!(self.support, Support::Ground { .. });
        if grounded && def.behaviors[index].motion == Motion::Fall {
            index = def.default_ground();
        }
        let behavior = &def.behaviors[index];
        self.behavior = index;
        self.behavior_time = 0.0;
        self.clip_step = 0;
        self.clip_timer = 0.0;
        self.clip_total = match &behavior.clip {
            Some(c) => {
                let repeat = match &c.repeat_expr {
                    Some(expr) => self.eval(expr).trunc().clamp(0.0, 100_000.0) as u32,
                    None => c.repeat,
                };
                c.total_steps_with(repeat).max(1)
            }
            None => 1,
        };
        self.behavior_length = match behavior.duration {
            Duration::Range(min, max) => self.rng.range(min, max),
            Duration::Clip => behavior.clip.as_ref().map_or(1.0, |c| {
                self.clip_total as f32 * (c.interval_start + c.interval_end) * 0.5
            }),
            Duration::Forever => f32::INFINITY,
        };
        match &behavior.motion {
            Motion::Turn => {
                self.facing = -self.facing;
                // Petit sursaut pendant le demi-tour : il se lit mieux qu'un retournement à plat.
                self.squash_kick(-1.1);
            }
            &Motion::Jump { vx, vy } if grounded => {
                self.vel = Vec2::new(vx * self.facing, -vy);
                self.support = Support::Air;
                self.ground_ref = None;
                self.squash_kick(1.4);
            }
            Motion::Look => self.anim.glance_timer = 0.0,
            Motion::Script(script) => {
                // Valeurs entières comme dans desktopPet, puis mises à la taille d'affichage.
                let k = script.scale;
                let mut value = |e: &Expr| self.eval(e).trunc() as f32 * k;
                let start = Vec2::new(value(&script.start[0]), value(&script.start[1]));
                let end = Vec2::new(value(&script.end[0]), value(&script.end[1]));
                self.script = [start, end];
            }
            _ => {}
        }
        // Personnages secondaires (desktopPet : cinq générations au plus).
        if !behavior.children.is_empty() && self.child_depth < 5 {
            let p = def.physics;
            let (frame, _) = self.screen.unwrap_or_default();
            let flipped = self.facing > 0.0;
            for c in &behavior.children {
                let x = self.eval(if flipped { &c.x_flipped } else { &c.x }).trunc() as f32;
                let y = self.eval(&c.y).trunc() as f32;
                self.children.push(ChildRequest {
                    behavior: c.next,
                    pos: Vec2::new(frame.x + x + p.width * 0.5, frame.y + y + p.height),
                    facing: self.facing,
                    depth: self.child_depth + 1,
                    skin: self.skin,
                });
            }
        }
    }

    fn update_screen(&mut self, world: &World) {
        self.screen = world.screen_at(self.pos).map(|s| (s.frame, s.usable));
    }

    /// Évalue une expression desktopPet avec les mesures de l'écran et de l'animal.
    fn eval(&mut self, expr: &Expr) -> f64 {
        let (frame, usable) = self.screen.unwrap_or_default();
        let p = self.def.physics;
        let behavior = &self.def.behaviors[self.behavior];
        let offset = behavior.clip.as_ref().map_or(0.0, |c| c.offset_y);
        let scale = match &behavior.motion {
            Motion::Script(s) => f64::from(s.scale),
            _ => 1.0,
        };
        let image_x = f64::from(self.pos.x - p.width * 0.5 - frame.x);
        let image_y = f64::from(self.pos.y - p.height + offset - frame.y);
        let rand_s = f64::from(self.rand_s);
        // Un seul tirage par expression : toutes les occurrences de `random` valent pareil.
        let random = f64::from(self.rng.next_u32() % 100);
        expr.eval(&mut |name| {
            Some(match name {
                "screenW" => f64::from(frame.w),
                "screenH" => f64::from(frame.h),
                "areaW" => f64::from(usable.right() - frame.x),
                "areaH" => f64::from(usable.bottom() - frame.y),
                "imageW" => f64::from(p.width),
                "imageH" => f64::from(p.height),
                "imageX" => image_x,
                "imageY" => image_y,
                "random" => random,
                "randS" => rand_s,
                "scale" => scale,
                _ => return None,
            })
        })
    }

    fn choose(&mut self, list: &[Transition], world: &World, input: &Input) -> Option<usize> {
        let candidates: Vec<&Transition> = list
            .iter()
            .filter(|t| t.when.iter().all(|c| self.condition(*c, world, input)))
            .collect();
        let i = self
            .rng
            .pick_weighted(candidates.iter().map(|t| t.weight))?;
        Some(candidates[i].to)
    }

    fn condition(&self, condition: Condition, world: &World, input: &Input) -> bool {
        let surface_kind = match self.support {
            Support::Ground { surface } => world.surface(surface).map(|s| s.kind),
            _ => None,
        };
        let night = input.local_hour >= 22.0 || input.local_hour < 6.0;
        let center = self.pos - Vec2::new(0.0, self.def.physics.height * 0.5);
        let cursor_dist = input.cursor.map(|c| (c - center).length());
        match condition {
            Condition::OnWindow => surface_kind == Some(SurfaceKind::Window),
            Condition::OnFloor => surface_kind == Some(SurfaceKind::Floor),
            Condition::Night => night,
            Condition::Day => !night,
            Condition::CursorNear(r) => cursor_dist.is_some_and(|d| d <= r),
            Condition::CursorFar(r) => cursor_dist.is_none_or(|d| d > r),
            Condition::Grounded => surface_kind.is_some(),
        }
    }

    fn step_anim(&mut self, dt: f32, input: &Input) {
        let p = self.def.physics;
        let grounded = matches!(self.support, Support::Ground { .. });
        let behavior = &self.def.behaviors[self.behavior];
        let pose = match self.support {
            Support::Held => Pose::Dangle,
            Support::Air => Pose::Air,
            Support::Ground { .. } => behavior.motion.pose(),
        };
        let looking = behavior.motion == Motion::Look;
        let sprites = matches!(self.def.appearance, Appearance::Sprites(_));
        let drawn = self.def.drawn();
        let Self {
            anim: a,
            rng,
            vel,
            facing,
            pos,
            particles,
            prev_vel_x,
            ..
        } = self;

        // Un dessin vectoriel pivote en douceur ; des sprites se retournent d'un coup.
        // Un demi-tour dessiné dans les images (`flip` de desktopPet) est appliqué
        // d'un coup ailleurs ; ici, les autres demi-tours pivotent en douceur,
        // plus vite pour des sprites.
        a.flip = damp(a.flip, *facing, if sprites { 30.0 } else { 15.0 }, dt);

        // Démarche calée sur la distance parcourue : les pattes ne glissent pas.
        let speed = if grounded { vel.x.abs() } else { 0.0 };
        let stride = p.height * 0.9;
        a.gait_phase = (a.gait_phase + speed / stride * TAU * dt).rem_euclid(TAU);
        a.walk = damp(
            a.walk,
            if grounded {
                smoothstep(2.0, 36.0, speed)
            } else {
                0.0
            },
            10.0,
            dt,
        );
        a.run = damp(
            a.run,
            if grounded {
                smoothstep(80.0, 150.0, speed)
            } else {
                0.0
            },
            6.0,
            dt,
        );

        let weight = |w: Pose| if pose == w { 1.0 } else { 0.0 };
        a.sit = damp(a.sit, weight(Pose::Sit) + weight(Pose::Sleep), 5.0, dt);
        a.sleep = damp(a.sleep, weight(Pose::Sleep), 2.0, dt);
        a.dangle = damp(a.dangle, weight(Pose::Dangle), 9.0, dt);
        a.air = damp(a.air, weight(Pose::Air), 12.0, dt);

        a.squash.target = if pose == Pose::Air && !drawn.squash {
            -(vel.y.abs() / p.terminal_velocity).min(1.0) * 0.08
        } else {
            0.0
        };
        a.squash.step(dt);
        a.squash.value = a.squash.value.clamp(-0.3, 0.38);

        let accel_x = (vel.x - *prev_vel_x) / dt;
        let mut lean = (accel_x * *facing * 0.00025).clamp(-0.16, 0.16) + a.run * 0.08;
        if pose == Pose::Air {
            lean += (vel.x * *facing * 0.00028).clamp(-0.35, 0.35);
        }
        a.lean.target = lean;
        a.lean.step(dt);

        a.ear.target = if pose == Pose::Air {
            (-vel.y * 0.0007).clamp(-0.7, 0.7)
        } else {
            0.0
        } - a.sleep * 0.25;
        a.ear.step(dt);

        // Balancement dans le repère de l'écran : tiré vers la droite, le corps
        // reste en arrière, à gauche (rotation positive, y vers le bas).
        a.swing.target = if pose == Pose::Dangle {
            (vel.x * 0.0012).clamp(-0.75, 0.75)
        } else {
            0.0
        };
        a.swing.step(dt);

        // Culbute : l'angle avance jusqu'à faire un tour complet, puis revient à zéro.
        if a.roll_left != 0.0 {
            let d = (a.roll_rate * dt).min(a.roll_left.abs()) * a.roll_left.signum();
            a.roll += d;
            a.roll_left -= d;
            if a.roll_left.abs() < 1e-4 {
                a.roll_left = 0.0;
                a.roll = 0.0;
            }
        }

        a.tail_phase = (a.tail_phase + dt * (1.6 + a.walk * 5.0 + a.run * 4.0)).rem_euclid(TAU);

        // Clignements, parfois doubles ; yeux clos pendant le sommeil.
        if a.sleep > 0.5 {
            a.blink = damp(a.blink, 1.0, 5.0, dt);
            a.blink_left = 0.0;
        } else if a.blink_left > 0.0 {
            a.blink_left -= dt;
            let t = 1.0 - (a.blink_left / BLINK_TIME).clamp(0.0, 1.0);
            a.blink = (t * PI).sin();
            if a.blink_left <= 0.0 {
                a.blink = 0.0;
                if a.double_blink {
                    a.double_blink = false;
                    a.blink_timer = 0.1;
                } else {
                    a.blink_timer = rng.range(2.2, 6.5);
                    a.double_blink = rng.chance(0.2);
                }
            }
        } else {
            a.blink = damp(a.blink, 0.0, 14.0, dt);
            a.blink_timer -= dt;
            if a.blink_timer <= 0.0 {
                a.blink_left = BLINK_TIME;
            }
        }

        let period = lerp(3.6, 5.4, a.sleep);
        a.breath_phase = (a.breath_phase + TAU * dt / period).rem_euclid(TAU);

        // Regard : le curseur s'il est proche, sinon des coups d'œil.
        let head = *pos + Vec2::new(*facing * p.width * 0.32, -p.height * 0.72);
        let mut target = Vec2::new(0.35, 0.05);
        if looking {
            a.glance_timer -= dt;
            if a.glance_timer <= 0.0 {
                a.glance = Vec2::new(rng.range(-0.6, 1.0), rng.range(-0.7, 0.4));
                a.glance_timer = rng.range(0.6, 1.6);
            }
            target = a.glance;
        }
        if let Some(cursor) = input.cursor
            && grounded
            && a.sleep < 0.5
        {
            let d = cursor - head;
            let dist = d.length();
            if dist < 460.0 {
                let dir = d.normalize_or_zero();
                let strength = 0.45 + 0.55 * smoothstep(460.0, 80.0, dist);
                target = Vec2::new(dir.x * *facing, dir.y) * strength;
            }
        }
        a.look = a.look.lerp(target, 1.0 - (-9.0 * dt).exp());

        // Petites bouffées de poussière derrière les pattes quand il court.
        let ground_speed = if grounded { vel.x.abs() } else { 0.0 };
        if ground_speed > 70.0 * (p.height / 60.0).max(0.4) && !drawn.dust {
            a.dust_timer -= dt;
            if a.dust_timer <= 0.0 && particles.len() + 2 <= MAX_PARTICLES {
                a.dust_timer = rng.range(0.16, 0.3);
                let back = -vel.x.signum();
                for _ in 0..2 {
                    particles.push(Particle {
                        kind: ParticleKind::Dust,
                        pos: *pos + Vec2::new(back * rng.range(2.0, p.width * 0.3), -1.0),
                        vel: Vec2::new(back * rng.range(18.0, 40.0), -rng.range(6.0, 22.0)),
                        age: 0.0,
                        life: rng.range(0.35, 0.55),
                        size: rng.range(1.6, 3.0) * (p.height / 60.0).clamp(0.5, 1.5),
                        phase: rng.range(0.0, TAU),
                    });
                }
            }
        } else {
            a.dust_timer = 0.05;
        }

        // Bulles de sommeil.
        if a.sleep > 0.85 {
            a.bubble_timer -= dt;
            if a.bubble_timer <= 0.0 && particles.len() < MAX_PARTICLES {
                a.bubble_timer = rng.range(1.5, 2.6);
                particles.push(Particle {
                    kind: ParticleKind::Bubble,
                    pos: *pos + Vec2::new(*facing * p.width * 0.36, -p.height * 0.8),
                    vel: Vec2::new(*facing * 4.0, -16.0),
                    age: 0.0,
                    life: rng.range(2.6, 3.4),
                    size: rng.range(4.2, 6.2),
                    phase: rng.range(0.0, TAU),
                });
            }
        }
    }

    fn emit_dust(&mut self, impact: f32) {
        if self.def.drawn().dust {
            return;
        }
        let p = self.def.physics;
        let count = (4.0 + impact / 180.0).min(12.0) as usize;
        for i in 0..count {
            if self.particles.len() >= MAX_PARTICLES {
                break;
            }
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            let speed = self.rng.range(40.0, 70.0 + impact * 0.05);
            self.particles.push(Particle {
                kind: ParticleKind::Dust,
                pos: self.pos + Vec2::new(side * self.rng.range(4.0, p.width * 0.32), -1.0),
                vel: Vec2::new(side * speed, -self.rng.range(10.0, 45.0)),
                age: 0.0,
                life: self.rng.range(0.45, 0.8),
                size: self.rng.range(3.0, 5.5),
                phase: self.rng.range(0.0, TAU),
            });
        }
    }

    fn step_particles(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.age += dt;
            match p.kind {
                ParticleKind::Dust => {
                    p.vel *= (-4.5 * dt).exp();
                    p.vel.y += 20.0 * dt;
                }
                ParticleKind::Bubble => {
                    p.vel.x = (p.age * 2.3 + p.phase).sin() * 9.0;
                }
            }
            p.pos += p.vel * dt;
        }
        self.particles.retain(|p| p.age < p.life);
    }

    fn update_shadow(&mut self, world: &World) {
        self.shadow = match self.support {
            Support::Ground { .. } => Some(Shadow {
                y: self.pos.y,
                distance: 0.0,
            }),
            _ => world
                .surface_below(self.pos, self.def.physics.width * 0.2)
                .map(|s| Shadow {
                    y: s.y,
                    distance: s.y - self.pos.y,
                }),
        };
    }

    /// Image dans la variante de couleur de l'animal : les indices du fichier
    /// désignent la première variante (desktopPet).
    /// Image affichée au pas `step` : celle de la séquence, ou, vers la fin
    /// du pas, une image intermédiaire vers celle du pas suivant. Le passage
    /// dure au plus [`INBETWEEN_TIME`] par image intermédiaire : une pose
    /// longue reste tenue, puis l'animal change de pose en douceur.
    fn sprite_frame(&self, clip: &SpriteClip, step: usize, looping: bool) -> u32 {
        let frame = clip.frame_at(step);
        let Appearance::Sprites(sheet) = &self.def.appearance else {
            return frame;
        };
        // Pas de pas suivant : la séquence s'arrête (ou garde sa dernière image).
        if step != self.clip_step || (!looping && step + 1 >= self.clip_total.max(1)) {
            return frame;
        }
        let next = clip.frame_at(step + 1);
        let Some((first, count)) = (next != frame)
            .then(|| sheet.inbetween(frame, next))
            .flatten()
        else {
            return frame;
        };
        let interval = self.clip_interval();
        let n = count as f32;
        let span = (interval * n / (n + 1.0)).min(n * INBETWEEN_TIME);
        let into = self.clip_timer - (interval - span);
        if into < 0.0 || span <= 0.0 {
            return frame;
        }
        first + ((into / span * n) as u32).min(count - 1)
    }

    fn skinned(&self, frame: u32) -> u32 {
        match &self.def.appearance {
            Appearance::Sprites(sheet) if sheet.skins > 1 => {
                let range = sheet.tile_count() / sheet.skins;
                if frame < range {
                    frame + self.skin * range
                } else {
                    frame
                }
            }
            _ => frame,
        }
    }

    /// Vue interpolée entre le pas précédent et le pas courant (`alpha` entre 0 et 1).
    #[must_use]
    pub fn view(&self, alpha: f32) -> PetView<'_> {
        let a = &self.anim;
        let behavior = &self.def.behaviors[self.behavior];
        let (sprite_frame, sprite_offset_y, opacity) = match &behavior.clip {
            Some(clip) => {
                let total = self.clip_total.max(1);
                // Tenu, lancé ou d'une durée propre, l'animation boucle ; sinon le
                // dernier pas garde la dernière image.
                let looping = self.support == Support::Held
                    || behavior.motion == Motion::Fall
                    || behavior.duration != Duration::Clip;
                let step = if looping {
                    self.clip_step
                } else {
                    self.clip_step.min(total - 1)
                };
                let t = (self.clip_step as f32 / total as f32).min(1.0);
                (
                    Some(self.skinned(self.sprite_frame(clip, step, looping))),
                    if self.support == Support::Held {
                        0.0
                    } else {
                        lerp(clip.offset_y, clip.offset_y_end, t)
                    },
                    lerp(clip.opacity_start, clip.opacity_end, t),
                )
            }
            None => (None, 0.0, 1.0),
        };
        let fade = self
            .leaving
            .and_then(|l| l.fade)
            .map_or(1.0, |f| f.clamp(0.0, 1.0));
        let opacity = opacity * fade;
        PetView {
            id: self.id,
            // Tenu, il est dessiné exactement sous le curseur, sans attendre
            // le prochain pas de la simulation.
            ground: match &self.hold {
                Some(hold) => hold.target - hold.offset,
                None => self.prev_pos.lerp(self.pos, alpha.clamp(0.0, 1.0)),
            },
            facing: a.flip,
            pose: PoseSignals {
                gait_phase: a.gait_phase,
                walk: a.walk,
                run: a.run,
                sit: a.sit,
                sleep: a.sleep,
                dangle: a.dangle,
                air: a.air,
                squash: a.squash.value,
                lean: a.lean.value,
                ear: a.ear.value,
                tail: a.tail_phase.sin() * (0.12 + a.walk * 0.1),
                blink: a.blink,
                breath: a.breath_phase.sin(),
                look: a.look,
                swing: a.swing.value,
                roll: a.roll,
            },
            shadow: self.shadow,
            sprite_frame,
            sprite_offset_y,
            opacity,
            particles: &self.particles,
            support: self.support,
            behavior: &behavior.name,
            grip: self.hold.map(|hold| hold.target),
        }
    }
}

/// Le troupeau : tous les animaux, avancés ensemble à pas fixe.
#[derive(Clone, Debug, Default)]
pub struct Flock {
    pets: Vec<Pet>,
    accumulator: f32,
    next_id: u32,
    /// Mode collisions : deux animaux qui se rentrent dedans partent à la renverse.
    collisions: bool,
}

/// Élans donnés à deux animaux qui se rentrent dedans, ou rien.
fn bump(a: &Pet, b: &Pet) -> Option<(Vec2, Vec2)> {
    if !a.can_bump() || !b.can_bump() {
        return None;
    }
    let (pa, pb) = (a.def.physics, b.def.physics);
    let dx = b.pos.x - a.pos.x;
    // Les corps se touchent (museaux un peu écrasés), à la même hauteur.
    if dx.abs() >= 0.47 * (pa.width + pb.width) {
        return None;
    }
    let (a_top, b_top) = (a.pos.y - pa.height * 0.8, b.pos.y - pb.height * 0.8);
    if a.pos.y <= b_top || b.pos.y <= a_top {
        return None;
    }
    // Ils se rapprochent : l'un au moins va vers l'autre.
    let side = if dx == 0.0 { -a.facing } else { dx.signum() };
    let closing = (a.vel.x - b.vel.x) * side;
    let airborne = a.support == Support::Air || b.support == Support::Air;
    if closing < if airborne { 40.0 } else { 6.0 } {
        return None;
    }
    // Chacun recule d'un peu plus que sa largeur, avec un bond de la moitié de sa hauteur.
    let push = |p: &crate::def::Physics, away: f32| {
        let up = (2.0 * p.gravity * p.height * 0.55).sqrt();
        let flight = 2.0 * up / p.gravity.max(1.0);
        let back = p.width * 1.3 / flight.max(0.1) + closing.min(600.0) * 0.4;
        Vec2::new(away * back, -up)
    };
    Some((push(&pa, -side), push(&pb, side)))
}

/// `a` vient-il, en marchant, de toucher `b` posé au même niveau ?
fn meeting(a: &Pet, b: &Pet) -> bool {
    if !a.can_meet() || !b.can_meet() || (a.pos.y - b.pos.y).abs() > 2.0 {
        return false;
    }
    let dx = b.pos.x - a.pos.x;
    if dx == 0.0 || a.vel.x * dx.signum() <= 1.0 {
        return false;
    }
    // Ils se touchent à ce pas-ci (corps côte à côte, de leur largeur).
    let reach = 0.5 * (a.def.physics.width + b.def.physics.width);
    let before = (b.prev_pos.x - a.prev_pos.x).abs();
    dx.abs() <= reach && before > reach
}

/// `a` a touché `b` : ce que chacun fait, comme dans desktopPet.
///
/// 1. S'ils ont des réactions en commun (même nom, poids non nul), les
///    deux font la même chose.
/// 2. Sinon, si `b` joue une réaction que `a` connaît (même de poids nul),
///    `a` l'imite une fois sur deux.
/// 3. Sinon, chacun réagit de son côté ; `b` seulement s'il fait face à `a`.
///
/// Sans réaction, `a` passe son chemin (il croise l'autre).
fn encounter(a: &mut Pet, b: &mut Pet, world: &World, input: &Input) {
    let (def_a, def_b) = (Arc::clone(&a.def), Arc::clone(&b.def));
    let list_a = a.meet_reactions(&def_a);
    let list_b = b.meet_reactions(&def_b);
    let name_a = |t: &Transition| def_a.behaviors[t.to].name.as_str();
    let current_b = def_b.behaviors[b.behavior].name.as_str();
    // Même sens (ou `b` dans son dos) : `a` ne peut qu'imiter.
    let same_direction = a.facing == b.facing
        || (a.facing < 0.0 && b.pos.x > a.pos.x)
        || (a.facing > 0.0 && b.pos.x < a.pos.x);
    let mut common = Vec::new();
    let mut mimic = None;
    for t in list_a {
        if name_a(t) == current_b {
            mimic = Some(t.to);
        }
        if same_direction {
            if mimic.is_some() {
                break;
            }
            continue;
        }
        if t.weight > 0.0
            && list_b
                .iter()
                .any(|u| u.weight > 0.0 && def_b.behaviors[u.to].name == name_a(t))
        {
            common.push(t.clone());
        }
    }

    let mut a_reacts = false;
    let mut b_reacts = false;
    if !common.is_empty() {
        if let Some(next) = a.choose(&common, world, input) {
            a.enter(next);
            a_reacts = true;
            if let Some(next_b) = def_b.behavior_index(&def_a.behaviors[next].name) {
                b.enter(next_b);
                b_reacts = true;
            }
        }
    } else if let Some(next) = mimic.filter(|_| a.rng.chance(0.5)) {
        a.enter(next);
        a_reacts = true;
    } else {
        if let Some(next) = a.choose(list_a, world, input) {
            a.enter(next);
            a_reacts = true;
        }
        if !same_direction && let Some(next) = b.choose(list_b, world, input) {
            b.enter(next);
            b_reacts = true;
        }
    }
    if a_reacts {
        // Il s'arrête contre l'autre, sans le chevaucher.
        let dir = (b.pos.x - a.pos.x).signum();
        let reach = 0.5 * (def_a.physics.width + def_b.physics.width);
        a.pos.x = b.pos.x - dir * reach;
        a.vel.x = 0.0;
    }
    if a_reacts || b_reacts {
        a.meet_cooldown = MEET_COOLDOWN;
        b.meet_cooldown = MEET_COOLDOWN;
        a.events.push(PetEvent::Met { other: b.id });
        b.events.push(PetEvent::Met { other: a.id });
    }
}

impl Flock {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un animal tombé du haut de l'écran ; renvoie son identifiant.
    pub fn spawn(&mut self, def: Arc<PetDef>, seed: u64, world: &World, x: Option<f32>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.pets.push(Pet::spawn(id, def, seed, world, x));
        id
    }

    pub fn remove(&mut self, id: u32) {
        self.pets.retain(|p| p.id() != id);
    }

    pub fn clear(&mut self) {
        self.pets.clear();
    }

    #[must_use]
    pub fn pets(&self) -> &[Pet] {
        &self.pets
    }

    pub fn pets_mut(&mut self) -> &mut [Pet] {
        &mut self.pets
    }

    #[must_use]
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Pet> {
        self.pets.iter_mut().find(|p| p.id() == id)
    }

    /// Active ou coupe le mode collisions.
    pub fn set_collisions(&mut self, on: bool) {
        self.collisions = on;
    }

    #[must_use]
    pub fn collisions(&self) -> bool {
        self.collisions
    }

    /// Avance la simulation du temps écoulé et renvoie le facteur
    /// d'interpolation à passer à [`Pet::view`].
    pub fn advance(&mut self, elapsed: f32, world: &World, input: &Input) -> f32 {
        // Au-delà d'un quart de seconde (Mac réveillé, débogueur…), on ne rattrape pas.
        self.accumulator += elapsed.clamp(0.0, 0.25);
        while self.accumulator >= TICK {
            for pet in &mut self.pets {
                pet.step(TICK, world, input);
            }
            if self.collisions {
                self.collide(world, input);
            } else {
                self.meet(world, input);
            }
            self.adopt_children(world);
            self.accumulator -= TICK;
        }
        self.accumulator / TICK
    }

    /// Deux animaux qui se rentrent dedans partent chacun à la renverse.
    fn collide(&mut self, world: &World, input: &Input) {
        for i in 0..self.pets.len() {
            for j in i + 1..self.pets.len() {
                let Some((push_i, push_j)) = bump(&self.pets[i], &self.pets[j]) else {
                    continue;
                };
                let (left, right) = self.pets.split_at_mut(j);
                left[i].knock(push_i, world, input);
                right[0].knock(push_j, world, input);
            }
        }
    }

    /// Rencontres : un animal qui en atteint un autre en marchant réagit,
    /// et l'autre parfois avec lui (`petSide` de desktopPet).
    fn meet(&mut self, world: &World, input: &Input) {
        for i in 0..self.pets.len() {
            for j in 0..self.pets.len() {
                if i == j || !meeting(&self.pets[i], &self.pets[j]) {
                    continue;
                }
                let (a, b) = if i < j {
                    let (left, right) = self.pets.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                } else {
                    let (left, right) = self.pets.split_at_mut(i);
                    (&mut right[0], &mut left[j])
                };
                encounter(a, b, world, input);
            }
        }
    }

    /// Tous les animaux disent au revoir : chacun joue son animation
    /// d'adieu, s'efface, puis quitte le troupeau.
    pub fn leave_all(&mut self, world: &World, input: &Input) {
        for pet in &mut self.pets {
            pet.leave(world, input);
        }
    }

    /// Tous ensemble : chaque animal libre joue son comportement `together`.
    /// Renvoie le nombre d'animaux qui le jouent.
    pub fn together(&mut self, world: &World, input: &Input) -> usize {
        self.pets
            .iter_mut()
            .filter_map(|pet| pet.together(world, input).then_some(()))
            .count()
    }

    /// Fait apparaître les personnages secondaires demandés et retire ceux
    /// qui ont terminé.
    fn adopt_children(&mut self, world: &World) {
        let mut born = Vec::new();
        for pet in &mut self.pets {
            for request in pet.children.drain(..) {
                born.push((Arc::clone(&pet.def), pet.rng.next_u32(), request));
            }
        }
        for (def, seed, request) in born {
            let id = self.next_id;
            self.next_id += 1;
            self.pets
                .push(Pet::child(id, def, u64::from(seed), world, request));
        }
        self.pets.retain(|p| !p.finished);
    }

    /// Activité la plus élevée du troupeau.
    #[must_use]
    pub fn activity(&self) -> Option<Activity> {
        self.pets.iter().map(Pet::activity).max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::parse_pet_toml;
    use crate::math::Rect;
    use crate::world::{LedgeOptions, Screen, WindowRect};

    const PET: &str = r#"
        [pet]
        id = "t"
        name = "T"
        [appearance]
        rig = "a.svg"
        height = 60.0
        [reflexes]
        spawn = "fall"
        fall = "fall"
        land = "stand"
        thrown = "fall"
        edge = "turn"
        wall = "turn"
        [[behavior]]
        name = "stand"
        motion = "stand"
        duration = 0.5
        next = "walk"
        [[behavior]]
        name = "walk"
        motion = "walk"
        speed = 60.0
        duration = 30
        next = "stand"
        [[behavior]]
        name = "turn"
        motion = "turn"
        duration = 0.2
        next = "walk"
        [[behavior]]
        name = "fall"
        motion = "fall"
        duration = "forever"
    "#;

    fn def() -> Arc<PetDef> {
        Arc::new(parse_pet_toml(PET, |_| Ok(b"<svg/>".to_vec())).unwrap())
    }

    fn world() -> World {
        World::new(vec![Screen {
            frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }])
    }

    fn run(pet: &mut Pet, world: &World, seconds: f32) {
        let input = Input {
            cursor: None,
            local_hour: 14.0,
        };
        for _ in 0..(seconds / TICK) as usize {
            pet.step(TICK, world, &input);
        }
    }

    #[test]
    fn falls_and_lands_on_the_floor() {
        let world = world();
        let mut pet = Pet::spawn(0, def(), 1, &world, Some(400.0));
        assert_eq!(pet.support(), Support::Air);
        run(&mut pet, &world, 3.0);
        assert!(
            matches!(pet.support(), Support::Ground { .. }),
            "{:?}",
            pet.support()
        );
        assert_eq!(pet.position().y, 825.0);
        let events = pet.take_events();
        assert!(
            events
                .iter()
                .any(|e| matches!(e, PetEvent::Landed { impact } if *impact > 1000.0)),
            "{events:?}"
        );
    }

    #[test]
    fn lands_on_a_window_ledge() {
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 5,
                frame: Rect::new(200.0, 400.0, 600.0, 300.0),
            }],
            LedgeOptions::default(),
        );
        let mut pet = Pet::spawn(0, def(), 1, &world, Some(500.0));
        run(&mut pet, &world, 2.0);
        assert_eq!(pet.position().y, 400.0);
    }

    #[test]
    fn turns_at_the_edge_instead_of_falling() {
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 5,
                frame: Rect::new(200.0, 400.0, 300.0, 300.0),
            }],
            LedgeOptions::default(),
        );
        let mut pet = Pet::spawn(0, def(), 3, &world, Some(350.0));
        run(&mut pet, &world, 20.0);
        assert_eq!(pet.position().y, 400.0, "il ne doit pas tomber du rebord");
        assert!((200.0..=500.0).contains(&pet.position().x));
    }

    #[test]
    fn rides_a_moving_window_and_falls_when_it_closes() {
        let mut world = world();
        let mut frame = Rect::new(200.0, 400.0, 600.0, 300.0);
        world.set_windows(&[WindowRect { id: 5, frame }], LedgeOptions::default());
        let mut pet = Pet::spawn(0, def(), 4, &world, Some(500.0));
        run(&mut pet, &world, 2.0);
        let before = pet.position();
        frame.x += 100.0;
        frame.y -= 50.0;
        world.set_windows(&[WindowRect { id: 5, frame }], LedgeOptions::default());
        pet.step(TICK, &world, &Input::default());
        let after = pet.position();
        assert!(
            (after.x - before.x - 100.0).abs() < 2.0,
            "{before:?} -> {after:?}"
        );
        assert_eq!(after.y, 350.0);

        world.set_windows(&[], LedgeOptions::default());
        run(&mut pet, &world, 2.0);
        assert_eq!(pet.position().y, 825.0);
    }

    #[test]
    fn can_be_grabbed_and_thrown() {
        let world = world();
        let mut pet = Pet::spawn(0, def(), 5, &world, Some(700.0));
        run(&mut pet, &world, 2.0);
        let start = pet.position();
        pet.grab(start - Vec2::new(0.0, 30.0), &world);
        assert!(pet.is_held());
        for i in 0..30 {
            pet.drag_to(start + Vec2::new(i as f32 * 12.0, -300.0 - i as f32 * 4.0));
            pet.step(TICK, &world, &Input::default());
        }
        assert!(pet.position().y < start.y - 100.0);
        pet.release(&world);
        assert_eq!(pet.support(), Support::Air);
        assert!(pet.velocity().x > 0.0, "l'élan est conservé");
        run(&mut pet, &world, 4.0);
        assert!(matches!(pet.support(), Support::Ground { .. }));
    }

    fn two_screens() -> World {
        World::new(vec![
            Screen {
                frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
                usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
                scale: 2.0,
            },
            Screen {
                frame: Rect::new(1440.0, 0.0, 1440.0, 900.0),
                usable: Rect::new(1440.0, 25.0, 1440.0, 800.0),
                scale: 2.0,
            },
        ])
    }

    #[test]
    fn walks_on_to_the_next_screen() {
        let world = two_screens();
        let mut pet = Pet::spawn(0, def(), 2, &world, Some(1380.0));
        run(&mut pet, &world, 2.0);
        pet.facing = 1.0;
        pet.force("walk");
        run(&mut pet, &world, 3.0);
        assert!(pet.position().x > 1460.0, "{:?}", pet.position());
        assert_eq!(pet.position().y, 825.0);
        let Support::Ground { surface } = pet.support() else {
            panic!("{:?}", pet.support())
        };
        assert_eq!(world.surface(surface).unwrap().x0, 1440.0);
    }

    #[test]
    fn simulation_is_deterministic() {
        let world = world();
        let mut a = Pet::spawn(0, def(), 99, &world, None);
        let mut b = Pet::spawn(0, def(), 99, &world, None);
        run(&mut a, &world, 12.0);
        run(&mut b, &world, 12.0);
        assert_eq!(a.position(), b.position());
        assert_eq!(a.behavior_name(), b.behavior_name());
    }

    #[test]
    fn flock_interpolation_factor_is_in_range() {
        let world = world();
        let mut flock = Flock::new();
        flock.spawn(def(), 1, &world, None);
        for _ in 0..100 {
            let alpha = flock.advance(1.0 / 60.0, &world, &Input::default());
            assert!((0.0..1.0).contains(&alpha));
        }
    }

    // ------------------------------------------------------------ animal en images

    fn strips() -> Arc<PetDef> {
        Arc::new(
            parse_pet_toml(
                crate::format::tests::STRIPS,
                crate::format::tests::strips_resolve,
            )
            .unwrap(),
        )
    }

    #[test]
    fn inbetweens_show_up_at_the_end_of_each_step() {
        let world = world();
        let def = Arc::new(
            parse_pet_toml(
                &crate::format::tests::strips_with_inbetweens(),
                crate::format::tests::inbetweens_resolve,
            )
            .unwrap(),
        );
        // Marche à 10 images par seconde : chaque image dure 0,1 s, la
        // seconde moitié montre l'image intermédiaire vers la suivante.
        let mut pet = standing(def, &world, 700.0, -1.0, "marche");
        let mut seen = Vec::new();
        for _ in 0..(0.4 / TICK).round() as usize {
            pet.step(TICK, &world, &Input::default());
            let frame = pet.view(1.0).sprite_frame.unwrap();
            if seen.last() != Some(&frame) {
                seen.push(frame);
            }
        }
        assert_eq!(&seen[..6], &[4, 34, 5, 35, 6, 36], "{seen:?}");
        // Après la dernière image de la marche, rien : elle est sautée.
        assert!(!seen.contains(&37), "{seen:?}");
    }

    /// Animal posé au sol, dans le comportement voulu.
    fn standing(def: Arc<PetDef>, world: &World, x: f32, facing: f32, behavior: &str) -> Pet {
        let floor = *world.floor_under(x).unwrap();
        let mut pet = Pet::spawn_at(0, def, 3, world, Vec2::new(x, floor.y));
        pet.pos = Vec2::new(x, floor.y);
        pet.prev_pos = pet.pos;
        pet.set_ground(&floor);
        pet.facing = facing;
        pet.anim.flip = facing;
        assert!(pet.force(behavior));
        pet
    }

    #[test]
    fn a_strip_pet_moves_frame_by_frame() {
        let world = world();
        let mut pet = standing(strips(), &world, 700.0, 1.0, "marche");
        // 4 images à 10 im/s, 3 passages : 1,2 s, 4 pixels par image à l'échelle 0,5.
        run(&mut pet, &world, 1.19);
        assert_eq!(pet.behavior_name(), "marche");
        let moved = pet.position().x - 700.0;
        assert!((moved - 24.0).abs() < 0.6, "{moved}");
        // Fini : pas de pas supplémentaire comme dans desktopPet, il passe à la suite.
        run(&mut pet, &world, 0.02);
        assert!(pet.behavior_time < 0.02, "{}", pet.behavior_time);
    }

    #[test]
    fn a_strip_pet_turns_with_its_own_frames() {
        let world = world();
        let mut pet = standing(strips(), &world, 22.0, -1.0, "marche");
        run(&mut pet, &world, 0.35);
        assert_eq!(
            pet.behavior_name(),
            "demi_tour",
            "le bord de l'écran le fait se retourner"
        );
        assert!(
            (pet.position().x - 16.0).abs() < 0.01,
            "{:?}",
            pet.position()
        );
        assert_eq!(pet.facing, -1.0, "le demi-tour est dans les images");
        // 3 images à 12 im/s : 0,25 s, puis il regarde de l'autre côté.
        run(&mut pet, &world, 0.25);
        assert_eq!(pet.facing, 1.0);
        assert_eq!(pet.behavior_name(), "marche");
    }

    #[test]
    fn per_frame_moves_follow_the_frames() {
        let world = world();
        let mut pet = standing(strips(), &world, 700.0, 1.0, "cogne");
        // [0, -4, -2, 0] pixels à 10 im/s, à l'échelle 0,5 : 3 points en arrière.
        run(&mut pet, &world, 0.39);
        assert!(
            (pet.position().x - 697.0).abs() < 0.05,
            "{:?}",
            pet.position()
        );
    }

    #[test]
    fn a_strip_pet_falls_when_its_ledge_ends() {
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 5,
                frame: Rect::new(200.0, 400.0, 300.0, 300.0),
            }],
            LedgeOptions::default(),
        );
        let ledge = *world
            .surfaces()
            .iter()
            .find(|s| s.kind == crate::world::SurfaceKind::Window)
            .unwrap();
        let mut pet = standing(strips(), &world, 480.0, 1.0, "marche");
        pet.set_ground(&ledge);
        pet.pos.y = ledge.y;
        pet.walking_off = true;
        run(&mut pet, &world, 3.0);
        assert_eq!(pet.position().y, 825.0, "tombé du rebord jusqu'au sol");
        assert!(pet.position().x > 505.0, "il est parti en avant");
    }

    #[test]
    fn a_held_pet_stays_where_it_was_caught() {
        let world = world();
        let mut pet = standing(strips(), &world, 700.0, -1.0, "repos");
        // Attrapé par le bas, à droite des pattes.
        let caught = pet.position() + Vec2::new(10.0, -3.0);
        pet.grab(caught, &world);
        assert_eq!(pet.behavior_name(), "tenu");
        // Comme une fenêtre : le point de prise reste sous le curseur, à
        // chaque instant, sans glisser ni traîner derrière lui.
        for i in 1..=120 {
            let cursor = caught + Vec2::new(i as f32 * 3.0, -(i as f32) * 5.0);
            pet.drag_to(cursor);
            pet.step(TICK, &world, &Input::default());
            let held = pet.position() + Vec2::new(10.0, -3.0);
            assert!((held - cursor).length() < 1e-3, "{i} : {held:?} {cursor:?}");
        }
        // Tout en haut de l'écran aussi, et au-delà.
        let top = Vec2::new(600.0, -40.0);
        pet.drag_to(top);
        run(&mut pet, &world, 0.5);
        assert!((pet.position() + Vec2::new(10.0, -3.0) - top).length() < 1e-3);
        // La main s'arrête avant de le lâcher : il tombe sans élan.
        pet.release(&world);
        assert!(pet.velocity().length() < 5.0, "{:?}", pet.velocity());
        assert_eq!(
            pet.behavior_name(),
            "chute",
            "lâché sans élan, il tombe simplement"
        );
    }

    #[test]
    fn colliding_pets_tumble_backwards_and_land_upright() {
        let world = world();
        let mut flock = Flock::new();
        let def = strips();
        let a = flock.spawn(Arc::clone(&def), 1, &world, Some(600.0));
        let b = flock.spawn(Arc::clone(&def), 2, &world, Some(640.0));
        let input = Input::default();
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &input);
        }
        // Face à face, et l'un marche vers l'autre.
        for (id, x, facing, behavior) in [(a, 600.0, 1.0, "marche"), (b, 640.0, -1.0, "repos")] {
            let pet = flock.get_mut(id).unwrap();
            assert!(matches!(pet.support(), Support::Ground { .. }));
            pet.pos.x = x;
            pet.prev_pos = pet.pos;
            pet.facing = facing;
            pet.force(behavior);
        }
        flock.set_collisions(true);
        let mut bumped = Vec::new();
        let mut max_roll: f32 = 0.0;
        for _ in 0..(1.4 / TICK) as usize {
            flock.advance(TICK, &world, &input);
            for pet in flock.pets_mut() {
                max_roll = max_roll.max(pet.anim.roll.abs());
                for e in pet.take_events() {
                    if matches!(e, PetEvent::Bumped { .. }) {
                        bumped.push(pet.id());
                    }
                }
            }
        }
        bumped.sort_unstable();
        assert_eq!(bumped, vec![a, b], "les deux sont heurtés, une seule fois");
        assert!(max_roll > 3.0, "ils culbutent ({max_roll})");
        let pets = flock.pets();
        let (pa, pb) = (&pets[0], &pets[1]);
        assert!(
            pb.position().x - pa.position().x > 70.0,
            "écartés : {:?} {:?}",
            pa.position(),
            pb.position()
        );
        for pet in pets {
            assert!(matches!(pet.support(), Support::Ground { .. }));
            assert_eq!(pet.anim.roll, 0.0, "il finit sur ses pattes");
        }
        assert_eq!(
            (pa.facing, pb.facing),
            (1.0, -1.0),
            "ils ont reculé sans se retourner"
        );
    }

    #[test]
    fn desktoppet_pets_turn_their_backs_after_a_bump() {
        // Sans idée à lui, un animal desktopPet repartirait droit sur l'autre :
        // après la culbute, il se pose le dos tourné au choc.
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/pelote/pet.toml");
        let def = Arc::new(crate::format::load_pet(&path).unwrap());
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(Arc::clone(&def), 1, &world, Some(600.0));
        let b = flock.spawn(Arc::clone(&def), 2, &world, Some(720.0));
        let input = Input::default();
        for _ in 0..(9.0 / TICK) as usize {
            flock.advance(TICK, &world, &input);
        }
        for (id, x, facing) in [(a, 600.0, 1.0), (b, 720.0, -1.0)] {
            let pet = flock.get_mut(id).unwrap();
            assert!(
                matches!(pet.support(), Support::Ground { .. }),
                "{}",
                pet.behavior_name()
            );
            pet.pos.x = x;
            pet.prev_pos = pet.pos;
            pet.facing = facing;
            pet.anim.flip = facing;
            assert!(pet.force("walk"));
        }
        flock.set_collisions(true);
        let mut bumped = Vec::new();
        let mut landed: Vec<(u32, f32)> = Vec::new();
        for _ in 0..(5.0 / TICK) as usize {
            flock.advance(TICK, &world, &input);
            for pet in flock.pets_mut() {
                for e in pet.take_events() {
                    match e {
                        PetEvent::Bumped { .. } => bumped.push(pet.id()),
                        PetEvent::Landed { .. }
                            if bumped.contains(&pet.id())
                                && !landed.iter().any(|(id, _)| *id == pet.id()) =>
                        {
                            landed.push((pet.id(), pet.facing));
                        }
                        _ => {}
                    }
                }
            }
        }
        bumped.sort_unstable();
        assert_eq!(bumped, vec![a, b], "les deux sont heurtés, une seule fois");
        landed.sort_by_key(|(id, _)| *id);
        assert_eq!(landed, vec![(a, -1.0), (b, 1.0)], "dos à dos");
    }

    #[test]
    fn collisions_are_off_by_default() {
        let world = world();
        let mut flock = Flock::new();
        let def = strips();
        let a = flock.spawn(Arc::clone(&def), 1, &world, Some(600.0));
        let b = flock.spawn(Arc::clone(&def), 2, &world, Some(620.0));
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        for pet in flock.pets_mut() {
            assert!(
                !pet.take_events()
                    .iter()
                    .any(|e| matches!(e, PetEvent::Bumped { .. })),
                "{} {a} {b}",
                pet.id()
            );
        }
    }

    /// Longue promenade d'un animal livré avec Mouflon sur un bureau qui
    /// change, en mode collisions : il ne sort pas des écrans, ne reste pas
    /// coincé dans une animation et joue l'essentiel de son répertoire.
    /// Renvoie le temps passé dans chaque comportement.
    fn long_life(
        pet: &str,
        minutes: f32,
        margin: f32,
        collisions: bool,
    ) -> (Arc<PetDef>, std::collections::BTreeMap<String, f32>) {
        use std::collections::BTreeMap;
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../pets/{pet}/pet.toml"));
        let def = Arc::new(crate::format::load_pet(&path).unwrap());
        let mut world = world();
        let mut flock = Flock::new();
        flock.set_collisions(collisions);
        for seed in 0..4 {
            flock.spawn(Arc::clone(&def), seed * 7919 + 1, &world, None);
        }
        let mut seconds_in: BTreeMap<String, f32> = BTreeMap::new();
        let mut same_since: Vec<(u32, String, f32)> = Vec::new();
        for i in 0..(minutes * 60.0 / TICK) as usize {
            let t = i as f32 * TICK;
            let windows = match (t / 50.0) as i32 % 3 {
                0 => vec![WindowRect {
                    id: 1,
                    frame: Rect::new(200.0 + 150.0 * (t * 0.05).sin(), 420.0, 500.0, 300.0),
                }],
                1 => vec![WindowRect {
                    id: 2,
                    frame: Rect::new(900.0, 250.0, 380.0, 500.0),
                }],
                _ => Vec::new(),
            };
            world.set_windows(&windows, LedgeOptions::default());
            let input = Input {
                cursor: Some(Vec2::new(700.0, 700.0)),
                local_hour: if t > minutes * 30.0 { 23.0 } else { 15.0 },
            };
            flock.advance(TICK, &world, &input);
            for pet in flock.pets() {
                let p = pet.position();
                assert!(
                    p.x.is_finite()
                        && p.y.is_finite()
                        && p.x > -margin
                        && p.x < 1440.0 + margin
                        && p.y > -margin
                        && p.y <= 825.0 + 0.01,
                    "{p:?} {}",
                    pet.behavior_name()
                );
                *seconds_in
                    .entry(pet.behavior_name().to_owned())
                    .or_default() += TICK;
                // Pas plus de cinq minutes d'affilée dans la même animation.
                let name = pet.behavior_name().to_owned();
                match same_since.iter_mut().find(|(id, _, _)| *id == pet.id()) {
                    Some(entry) if entry.1 == name => {
                        assert!(t - entry.2 < 300.0, "{} bloqué dans {name}", pet.id());
                    }
                    Some(entry) => *entry = (pet.id(), name, t),
                    None => same_since.push((pet.id(), name, t)),
                }
            }
        }
        for (name, s) in &seconds_in {
            eprintln!("{name:>20} : {s:7.0} s");
        }
        (def, seconds_in)
    }

    fn never_played<'a>(
        def: &'a PetDef,
        seconds_in: &std::collections::BTreeMap<String, f32>,
    ) -> Vec<&'a str> {
        def.behaviors
            .iter()
            .map(|b| b.name.as_str())
            .filter(|n| !seconds_in.contains_key(*n))
            .collect()
    }

    #[test]
    fn lainou_lives_a_long_life() {
        // Sans collisions, les Lainou se rencontrent et se saluent.
        let (def, seconds_in) = long_life("lainou", 30.0, 40.0, false);
        let never: Vec<&str> = never_played(&def, &seconds_in)
            .into_iter()
            .filter(|n| !matches!(*n, "tenu" | "tombe" | "au_revoir" | "culbute"))
            .collect();
        assert!(never.len() <= 3, "jamais joués : {never:?}");
        assert!(
            seconds_in.contains_key("salue") || seconds_in.contains_key("fete"),
            "ils ne se sont jamais rencontrés"
        );
        // En mode collisions, ils se bousculent et ne restent pas coincés.
        let (_, seconds_in) = long_life("lainou", 10.0, 40.0, true);
        assert!(seconds_in.contains_key("culbute"));
    }

    #[test]
    fn pelote_lives_a_long_life() {
        // Animal desktopPet : il entre par les côtés de l'écran et y grimpe.
        let (def, seconds_in) = long_life("pelote", 30.0, 400.0, false);
        let never = never_played(&def, &seconds_in);
        eprintln!("jamais joués : {never:?}");
        // Les chutes des fenêtres sont hautes : « fall hard » plutôt que « fall soft ».
        for essential in [
            "walk",
            "run",
            "sleep1a",
            "fall",
            "fall hard",
            "vertical_walk_up",
        ] {
            assert!(
                seconds_in.contains_key(essential),
                "{essential} jamais joué"
            );
        }
        // En mode collisions, il culbute et se relève, sans y passer son temps.
        let (_, seconds_in) = long_life("pelote", 10.0, 400.0, true);
        let total: f32 = seconds_in.values().sum();
        let down = seconds_in.get("fall soft").copied().unwrap_or(0.0)
            + seconds_in.get("toss").copied().unwrap_or(0.0);
        assert!(down < total * 0.3, "{down} s à terre sur {total} s");
    }

    /// Animal qui salue ceux qu'il rencontre, se retourne devant les
    /// fenêtres, dit au revoir et sait jouer avec les autres.
    const SOCIAL: &str = r#"
        [pet]
        id = "s"
        name = "S"
        [appearance]
        rig = "a.svg"
        height = 60.0
        [reflexes]
        spawn = "fall"
        fall = "fall"
        land = "stand"
        thrown = "fall"
        edge = "turn"
        wall = "turn"
        meet = "greet"
        window_side = "turn"
        leave = "wave"
        together = "hop"
        [[behavior]]
        name = "stand"
        motion = "stand"
        duration = 0.5
        next = "walk"
        [[behavior]]
        name = "walk"
        motion = "walk"
        speed = 60.0
        duration = 30
        next = "stand"
        [[behavior]]
        name = "rest"
        motion = "sit"
        duration = 30
        next = "stand"
        [[behavior]]
        name = "turn"
        motion = "turn"
        duration = 0.2
        next = "walk"
        [[behavior]]
        name = "greet"
        motion = "stand"
        duration = 0.6
        next = "turn"
        [[behavior]]
        name = "wave"
        motion = "stand"
        duration = 0.5
        next = "stand"
        [[behavior]]
        name = "hop"
        motion = "sit"
        duration = 0.4
        next = "stand"
        [[behavior]]
        name = "fall"
        motion = "fall"
        duration = "forever"
    "#;

    fn social() -> Arc<PetDef> {
        Arc::new(parse_pet_toml(SOCIAL, |_| Ok(b"<svg/>".to_vec())).unwrap())
    }

    /// Pose un animal du troupeau sur le sol, tourné et occupé comme voulu.
    fn put(flock: &mut Flock, world: &World, id: u32, x: f32, facing: f32, behavior: &str) {
        let floor = *world.floor_under(x).unwrap();
        let pet = flock.get_mut(id).unwrap();
        pet.pos = Vec2::new(x, floor.y);
        pet.prev_pos = pet.pos;
        pet.vel = Vec2::ZERO;
        pet.set_ground(&floor);
        pet.face(facing);
        assert!(pet.force(behavior));
    }

    fn advance(flock: &mut Flock, world: &World, seconds: f32) {
        let input = Input {
            cursor: None,
            local_hour: 14.0,
        };
        for _ in 0..(seconds / TICK).round() as usize {
            flock.advance(TICK, world, &input);
        }
    }

    #[test]
    fn pets_meeting_face_to_face_greet_each_other_then_part() {
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(social(), 1, &world, None);
        let b = flock.spawn(social(), 2, &world, None);
        put(&mut flock, &world, a, 400.0, 1.0, "walk");
        put(&mut flock, &world, b, 560.0, -1.0, "walk");
        let reach = flock.pets()[0].def.physics.width;
        let mut met_at = None;
        for i in 0..(4.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
            let (pa, pb) = (&flock.pets()[0], &flock.pets()[1]);
            assert!(
                pb.position().x - pa.position().x >= reach - 1.5,
                "ils se chevauchent"
            );
            if met_at.is_none() && pa.behavior_name() == "greet" {
                assert_eq!(pb.behavior_name(), "greet", "ils se saluent ensemble");
                met_at = Some(i);
            }
        }
        assert!(met_at.is_some(), "ils se sont rencontrés");
        let events: Vec<PetEvent> = flock.pets_mut()[0].take_events();
        assert!(events.contains(&PetEvent::Met { other: b }), "{events:?}");
        // Après le salut, demi-tour : chacun repart de son côté.
        let gap = flock.pets()[1].position().x - flock.pets()[0].position().x;
        advance(&mut flock, &world, 2.0);
        let later = flock.pets()[1].position().x - flock.pets()[0].position().x;
        assert!(later > gap + 50.0, "{gap} → {later}");
    }

    #[test]
    fn a_resting_pet_is_left_alone() {
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(social(), 1, &world, None);
        let b = flock.spawn(social(), 2, &world, None);
        put(&mut flock, &world, a, 400.0, 1.0, "walk");
        put(&mut flock, &world, b, 520.0, -1.0, "rest");
        let mut greeted = false;
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
            greeted |= flock.pets()[0].behavior_name() == "greet";
            assert_eq!(
                flock.pets()[1].behavior_name(),
                "rest",
                "il ne se dérange pas"
            );
        }
        assert!(greeted, "celui qui marche salue quand même");
    }

    #[test]
    fn pets_without_greetings_walk_past_each_other() {
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(def(), 1, &world, None);
        let b = flock.spawn(def(), 2, &world, None);
        put(&mut flock, &world, a, 400.0, 1.0, "walk");
        put(&mut flock, &world, b, 560.0, -1.0, "walk");
        advance(&mut flock, &world, 3.0);
        assert!(
            flock.pets()[0].position().x > flock.pets()[1].position().x,
            "ils se sont croisés"
        );
    }

    #[test]
    fn the_side_of_a_window_turns_a_pet_back_or_lets_it_pass() {
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 4,
                frame: Rect::new(500.0, 600.0, 400.0, 400.0),
            }],
            LedgeOptions::default(),
        );
        let mut flock = Flock::new();
        let a = flock.spawn(social(), 1, &world, None);
        put(&mut flock, &world, a, 380.0, 1.0, "walk");
        let half = flock.pets()[0].def.physics.width * 0.5;
        let mut furthest: f32 = 0.0;
        for _ in 0..(4.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
            furthest = furthest.max(flock.pets()[0].position().x);
        }
        assert!(furthest <= 500.0 - half + 0.5, "{furthest}");
        assert_eq!(flock.pets()[0].facing, -1.0, "il a fait demi-tour");

        // Sans réaction prévue, il passe devant la fenêtre.
        let mut flock = Flock::new();
        let b = flock.spawn(def(), 1, &world, None);
        put(&mut flock, &world, b, 380.0, 1.0, "walk");
        advance(&mut flock, &world, 4.0);
        assert!(flock.pets()[0].position().x > 560.0);
    }

    #[test]
    fn a_pet_says_goodbye_then_fades_away() {
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(social(), 3, &world, None);
        put(&mut flock, &world, a, 600.0, 1.0, "stand");
        flock.leave_all(&world, &Input::default());
        let pet = &flock.pets()[0];
        assert!(pet.is_leaving());
        assert_eq!(pet.behavior_name(), "wave");
        advance(&mut flock, &world, 0.4);
        assert_eq!(flock.pets()[0].view(1.0).opacity, 1.0, "il salue d'abord");
        advance(&mut flock, &world, 0.5);
        let opacity = flock.pets()[0].view(1.0).opacity;
        assert!(opacity < 1.0 && opacity > 0.0, "puis s'efface : {opacity}");
        assert_eq!(
            flock.pets()[0].behavior_name(),
            "wave",
            "sur sa dernière pose"
        );
        advance(&mut flock, &world, 1.2);
        assert!(flock.pets().is_empty(), "il est parti");
    }

    #[test]
    fn a_pet_without_farewell_fades_at_once_even_when_held() {
        let world = world();
        let mut flock = Flock::new();
        let a = flock.spawn(def(), 3, &world, None);
        let b = flock.spawn(def(), 4, &world, None);
        put(&mut flock, &world, a, 600.0, 1.0, "walk");
        put(&mut flock, &world, b, 900.0, 1.0, "stand");
        let held = flock.get_mut(b).unwrap();
        held.grab(Vec2::new(900.0, 790.0), &world);
        flock.leave_all(&world, &Input::default());
        assert!(flock.pets().iter().all(|p| !p.is_held() && p.is_leaving()));
        advance(&mut flock, &world, 0.3);
        assert!(flock.pets().iter().all(|p| p.view(1.0).opacity < 1.0));
        // Il ne s'attrape plus pendant qu'il s'en va.
        let pet = flock.get_mut(a).unwrap();
        pet.grab(Vec2::new(600.0, 800.0), &world);
        assert!(!pet.is_held());
        advance(&mut flock, &world, 1.0);
        assert!(flock.pets().is_empty());
    }

    #[test]
    fn thrown_upward_it_slips_behind_the_menu_bar_then_falls_back() {
        let world = world();
        let mut pet = Pet::spawn(0, def(), 6, &world, Some(600.0));
        run(&mut pet, &world, 2.0);
        let h = pet.def.physics.height;
        pet.support = Support::Air;
        pet.ground_ref = None;
        pet.vel = Vec2::new(0.0, -2600.0);
        let mut highest = f32::MAX;
        for _ in 0..(3.0 / TICK) as usize {
            pet.step(TICK, &world, &Input::default());
            highest = highest.min(pet.position().y - h);
        }
        // Le haut de la tête passe au-dessus de la zone utile (sous la barre
        // des menus), sans dépasser le bord de l'écran.
        assert!((0.0..25.0).contains(&highest), "{highest}");
        assert_eq!(pet.position().y, 825.0, "il est retombé");
    }

    #[test]
    fn two_lainous_who_meet_do_the_same_thing() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/lainou/pet.toml");
        let lainou = Arc::new(crate::format::load_pet(&path).unwrap());
        let world = world();
        for seed in 0..12 {
            let mut flock = Flock::new();
            let a = flock.spawn(Arc::clone(&lainou), seed, &world, None);
            let b = flock.spawn(Arc::clone(&lainou), seed + 100, &world, None);
            put(&mut flock, &world, a, 500.0, 1.0, "marche");
            put(&mut flock, &world, b, 640.0, -1.0, "marche");
            let mut reaction = None;
            for _ in 0..(3.0 / TICK) as usize {
                flock.advance(TICK, &world, &Input::default());
                if flock.pets()[0].meet_cooldown > 0.0 {
                    let names: Vec<&str> = flock.pets().iter().map(Pet::behavior_name).collect();
                    reaction = Some(names.join(" + "));
                    break;
                }
            }
            let reaction = reaction.expect("ils se rencontrent");
            assert!(
                [
                    "salue + salue",
                    "fete + fete",
                    "demi_tour + demi_tour",
                    "surpris + surpris"
                ]
                .contains(&reaction.as_str()),
                "{reaction}"
            );
        }
    }

    #[test]
    fn together_makes_every_free_pet_play_along() {
        let world = world();
        let mut flock = Flock::new();
        let ids: Vec<u32> = (0..3)
            .map(|i| flock.spawn(social(), i, &world, None))
            .collect();
        for (k, id) in ids.iter().enumerate() {
            put(
                &mut flock,
                &world,
                *id,
                300.0 + 200.0 * k as f32,
                1.0,
                "walk",
            );
        }
        assert_eq!(flock.together(&world, &Input::default()), 3);
        assert!(flock.pets().iter().all(|p| p.behavior_name() == "hop"));
        // Un animal sans « together » ne bouge pas.
        let mut flock = Flock::new();
        let id = flock.spawn(def(), 1, &world, None);
        put(&mut flock, &world, id, 300.0, 1.0, "walk");
        assert_eq!(flock.together(&world, &Input::default()), 0);
        assert_eq!(flock.pets()[0].behavior_name(), "walk");
    }
}
