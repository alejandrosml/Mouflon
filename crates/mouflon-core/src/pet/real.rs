//! Physique réaliste d'un animal (voir [`crate::physics`]).
//!
//! Le comportement reste celui de l'animal (marcher, se retourner au bout
//! d'un rebord, réagir à un atterrissage…) ; ce qui change, c'est la
//! mécanique : au sol, la plateforme qui le porte l'emporte par le
//! frottement de ses pattes et peut se dérober sous lui ; en l'air, son
//! corps tourne, rebondit et heurte ce qu'il rencontre ; tenu, il pend sous
//! la main comme un pendule.

use std::f32::consts::TAU;
use std::sync::Arc;

use super::{Input, LANDING_REACH, Pet, Support, THROW_SMOOTHING};
use crate::def::Rules;
use crate::math::{Vec2, approach};
use crate::physics::{
    self, Back, GRIP, GROUND_FRICTION, HOLD_DAMPING, HOLD_STIFFNESS, MAX_SPIN, Obb, Rigid,
    SPIN_DRAG, Tracker, WALL_FRICTION, WALL_RESTITUTION, box_inertia, collide_static, pet_surface,
};
use crate::world::{Surface, SurfaceId, SurfaceKind, World};

/// La plateforme se dérobe quand l'écart avec ses pattes dépasse ceci (pt)…
const SEPARATION: f32 = 1.0;
/// … et qu'elle s'éloigne de lui plus vite que ceci (pt/s) : en dessous,
/// c'est le bruit des relevés, pas une chute.
const SEPARATION_SPEED: f32 = 40.0;
/// Le sol retrouvé avant ce temps (et sans choc) : il reprend ce qu'il
/// faisait, comme s'il n'avait pas décollé.
const GRACE: f32 = 0.12;
const GRACE_IMPACT: f32 = 300.0;

impl Pet {
    /// Son corps : un rectangle autour du milieu de son dessin, tourné de
    /// son angle.
    #[must_use]
    pub fn body(&self) -> Obb {
        let p = self.def.physics;
        Obb {
            center: self.pos - Vec2::new(0.0, p.height * 0.5),
            half: Vec2::new(p.width * 0.5, p.height * 0.5),
            angle: self.angle,
        }
    }

    /// Son corps vu par un choc (masse unité).
    pub(crate) fn rigid(&self) -> Rigid {
        let p = self.def.physics;
        Rigid::boxed(self.vel + self.carry, self.spin, p.width, p.height)
    }

    /// Le dos sur lequel un autre peut se poser : au sol, debout, et ni
    /// tenu, ni en partance, ni personnage secondaire.
    pub(crate) fn back(&self) -> Option<Back> {
        if !matches!(self.support, Support::Ground { .. })
            || self.angle.abs() > 0.35
            || self.is_child()
            || self.leaving.is_some()
        {
            return None;
        }
        let p = self.def.physics;
        Some(Back {
            surface: Surface {
                id: pet_surface(self.id),
                kind: SurfaceKind::Window,
                x0: self.pos.x - p.width * 0.32,
                x1: self.pos.x + p.width * 0.32,
                y: self.pos.y - p.height * 0.92,
            },
            vel: self.vel + self.carry,
        })
    }

    /// Point de prise de la main dans le repère du corps (depuis son
    /// centre, avant rotation) : ce que retient `Hold::offset` en physique
    /// réaliste. Le corps pend sous ce point et tourne autour.
    pub(super) fn grip_offset(&self, cursor: Vec2) -> Vec2 {
        (cursor - self.body().center).rotated(-self.angle)
    }

    /// Où il est dessiné quand la main le tient à `hand` par le point `grip`
    /// de son corps (physique réaliste) : son centre est sous la main, à la
    /// distance que donne son angle ; comme toujours, le dessin tourne
    /// ensuite autour de ce centre.
    pub(super) fn held_ground(&self, hand: Vec2, grip: Vec2) -> Vec2 {
        hand - grip.rotated(self.angle) + Vec2::new(0.0, self.def.physics.height * 0.5)
    }

    /// Retour à la physique classique : il se redresse et n'a plus que sa
    /// vitesse à lui.
    pub(super) fn leave_realistic(&mut self) {
        self.angle = 0.0;
        self.spin = 0.0;
        self.vel += self.carry;
        self.carry = Vec2::ZERO;
        self.free_fall = false;
        self.grace = false;
        if let Some(hold) = &mut self.hold {
            hold.offset = hold.target - self.pos;
        }
    }

    /// Passage à la physique réaliste pendant qu'on le tient : la prise
    /// devient un point de son corps.
    pub(super) fn enter_realistic(&mut self) {
        if let Some(hold) = self.hold {
            let offset = self.grip_offset(hold.target);
            if let Some(hold) = &mut self.hold {
                hold.offset = offset;
            }
        }
    }

    /// Un pas de physique réaliste.
    pub(super) fn step_physics(
        &mut self,
        dt: f32,
        tracker: &Tracker,
        input: &Input,
        scripted: bool,
    ) {
        let world = tracker.world();
        match self.support {
            Support::Held => self.step_held_real(dt),
            Support::Air if self.free_fall || !scripted => {
                self.step_air_real(dt, tracker, input, scripted);
            }
            Support::Ground { surface } if self.drops_away(dt, tracker, surface) => {
                self.fall_off(dt);
            }
            Support::Ground { surface } if !scripted => {
                self.step_ground_real(dt, tracker, input, surface);
            }
            // Animation image par image (Lainou, Pelote, desktopPet) : ses
            // pas l'avancent sur la surface, et la plateforme l'emporte par
            // ses pattes, comme les autres.
            Support::Ground { surface } => {
                self.carry_on(dt, tracker, surface);
                self.step_script(dt, world, input);
            }
            // En l'air, son animation le mène (une escalade, un saut).
            _ => self.step_script(dt, world, input),
        }
        if matches!(self.support, Support::Ground { .. }) && (self.angle != 0.0 || self.spin != 0.0)
        {
            (self.angle, self.spin) = physics::righting_step(self.angle, self.spin, dt);
        }
    }

    /// La plateforme qui le porte se dérobe-t-elle ? Elle descend plus vite
    /// que la pesanteur ne le ferait tomber, ou, montée vite puis arrêtée net,
    /// elle le laisse filer vers le haut.
    fn drops_away(&self, dt: f32, tracker: &Tracker, surface: SurfaceId) -> bool {
        let Some(s) = tracker.world().surface(surface) else {
            return false;
        };
        let fall = self.carry.y + self.def.physics.gravity * dt;
        let gap = s.y - (self.pos.y + fall * dt);
        gap > SEPARATION && (tracker.velocity(surface).y - fall).abs() > SEPARATION_SPEED
    }

    /// Plus rien sous ses pattes : il part avec l'élan qu'il avait, et garde
    /// un instant ce qu'il faisait (voir [`GRACE`]).
    fn fall_off(&mut self, dt: f32) {
        let fall = self.carry.y + self.def.physics.gravity * dt;
        self.carry.y = 0.0;
        self.vel.y = fall;
        self.support = Support::Air;
        self.ground_ref = None;
        self.walking_off = false;
        self.take_off();
        self.grace = true;
        self.pos.y += fall * dt;
    }

    /// La grâce est finie : il tombe pour de bon (réflexe de chute).
    fn start_falling(&mut self, world: &World, input: &Input) {
        self.grace = false;
        let def = Arc::clone(&self.def);
        let behavior = &def.behaviors[self.behavior];
        if behavior.motion.is_airborne() {
            return;
        }
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

    fn step_ground_real(&mut self, dt: f32, tracker: &Tracker, input: &Input, id: SurfaceId) {
        let world = tracker.world();
        self.slide = 0.0;
        let Some(surface) = self.carry_on(dt, tracker, id) else {
            self.lift_off(world, input);
            return;
        };
        self.pos.y = surface.y;
        self.walk_on(dt, world, input, &surface);
    }

    /// La plateforme qui le porte l'emporte : ses pattes la suivent tant
    /// qu'elles adhèrent ; secouée d'un coup sec, elle file sous lui et il
    /// glisse. Elle lui donne aussi sa vitesse verticale, qu'il garde si
    /// elle se dérobe ([`Self::drops_away`]). Renvoie la surface, si elle
    /// est toujours là.
    fn carry_on(&mut self, dt: f32, tracker: &Tracker, id: SurfaceId) -> Option<Surface> {
        let surface = tracker.world().surface(id).copied()?;
        // Déplacée d'un coup : il la suit tel quel.
        self.pos.x += tracker.jumped(id).x;
        let platform = tracker.velocity(id);
        self.carry.y = platform.y;
        self.carry.x = approach(
            self.carry.x,
            platform.x,
            GRIP * self.def.physics.gravity * dt,
        );
        self.pos.x += self.carry.x * dt;
        // Le déplacement de la surface est déjà compté : `ride` ne l'emporte
        // pas une seconde fois.
        self.ground_ref = Some(super::GroundRef {
            surface: surface.id,
            x0: surface.x0,
            x1: surface.x1,
        });
        Some(surface)
    }

    fn step_held_real(&mut self, dt: f32) {
        self.slide = 0.0;
        let Some(hold) = self.hold else {
            self.support = Support::Air;
            return;
        };
        let p = self.def.physics;
        // La main : sa vitesse, lissée sur quelques centièmes de seconde (elle
        // donne l'élan du lancer), et son accélération.
        let gesture = match self.hand_at {
            Some(at) if dt > 0.0 => (hold.target - at) * (1.0 / dt),
            _ => Vec2::ZERO,
        };
        self.hand_at = Some(hold.target);
        let before = self.hand;
        self.hand = self.hand.lerp(gesture, 1.0 - (-dt / THROW_SMOOTHING).exp());
        let hand_accel =
            ((self.hand - before) * (1.0 / dt.max(1e-4))).clamp_length(p.gravity * 6.0);
        // Pendule autour du point de prise : la pesanteur, et l'inertie quand
        // la main accélère, font tourner le corps ; il se raidit un peu.
        let arm = -hold.offset.rotated(self.angle);
        let inertia = box_inertia(1.0, p.width, p.height) + arm.dot(arm);
        let force = Vec2::new(0.0, p.gravity) - hand_accel;
        let upright = self.angle - TAU * (self.angle / TAU).round();
        let accel =
            arm.cross(force) / inertia - HOLD_STIFFNESS * upright - HOLD_DAMPING * self.spin;
        self.spin = (self.spin + accel * dt).clamp(-MAX_SPIN, MAX_SPIN);
        self.angle += self.spin * dt;
        self.pos = self.held_ground(hold.target, hold.offset);
        // Vitesse du centre du corps : celle de la main, plus le balancement.
        let arm = -hold.offset.rotated(self.angle);
        self.vel = self.hand + arm.perp() * self.spin;
    }

    /// Lâché : le centre du corps garde la vitesse qu'il avait (la main plus
    /// le balancement) et sa rotation.
    pub(super) fn throw_real(&mut self) {
        let Some(hold) = self.hold else {
            return;
        };
        let arm = -hold.offset.rotated(self.angle);
        self.vel = self.hand + arm.perp() * self.spin;
        self.pos = self.held_ground(hold.target, hold.offset);
        self.prev_pos = self.pos;
        self.hand_at = None;
    }

    fn step_air_real(&mut self, dt: f32, tracker: &Tracker, input: &Input, scripted: bool) {
        let world = tracker.world();
        let def = Arc::clone(&self.def);
        let p = def.physics;
        self.slide = 0.0;
        self.air_time += dt;
        if self.grace && self.air_time > GRACE {
            self.start_falling(world, input);
        }
        self.vel.y = (self.vel.y + p.gravity * dt).min(p.terminal_velocity);
        self.vel.x *= (-p.air_drag * dt).exp();
        self.spin = (self.spin * (-SPIN_DRAG * dt).exp()).clamp(-MAX_SPIN, MAX_SPIN);
        let from = self.body();
        let mut body = Obb {
            center: from.center + self.vel * dt,
            angle: self.angle + self.spin * dt,
            ..from
        };
        let mut rigid = self.rigid();
        let arena = world.arena();

        // Bords des écrans, et le haut (derrière la barre des menus).
        let ex = body.extent_x();
        for (side, edge) in [(1.0, arena.left()), (-1.0, arena.right())] {
            if (body.center.x - edge) * side < ex {
                let n = Vec2::new(side, 0.0);
                let point = body.support(-n);
                collide_static(
                    &mut rigid,
                    point - body.center,
                    n,
                    Vec2::ZERO,
                    WALL_RESTITUTION,
                    WALL_FRICTION,
                );
                body.center.x = edge + side * ex;
            }
        }
        let ey = body.extent_y();
        let ceiling = world.bounds().top();
        if body.center.y - ey < ceiling {
            let n = Vec2::new(0.0, 1.0);
            let point = body.support(-n);
            collide_static(
                &mut rigid,
                point - body.center,
                n,
                Vec2::ZERO,
                WALL_RESTITUTION,
                WALL_FRICTION,
            );
            body.center.y = ceiling + ey;
        }

        // Côtés des fenêtres, quand il y arrive de l'extérieur.
        for wall in world.walls() {
            let side = wall.facing;
            let outside = (from.center.x - wall.x) * side >= from.extent_x() - 0.5;
            let inside = (body.center.x - wall.x) * side < ex;
            let level = body.center.y + ey > wall.y0 + 2.0 && body.center.y - ey < wall.y1 - 2.0;
            if !(outside && inside && level) {
                continue;
            }
            let n = Vec2::new(side, 0.0);
            let mut point = body.support(-n);
            point.y = point.y.clamp(wall.y0, wall.y1);
            let hit = collide_static(
                &mut rigid,
                point - body.center,
                n,
                tracker.window_velocity(wall.window),
                WALL_RESTITUTION,
                WALL_FRICTION,
            );
            body.center.x = wall.x + side * ex;
            if hit > 200.0 {
                self.squash_kick(-0.5);
            }
        }

        // Atterrissage : le point le plus bas du corps franchit une surface
        // (un rebord, un sol, le dos d'un autre), en tenant compte de son
        // mouvement à elle.
        let low_from = from.center.y + from.extent_y();
        let low_to = body.center.y + ey;
        let lowest = body.lowest_point();
        let margin = p.width * LANDING_REACH;
        let mut landing: Option<(Surface, Vec2)> = None;
        for s in world.surfaces() {
            if !s.contains_x(lowest.x, margin) || Some(s.id) == self.own_back() {
                continue;
            }
            let v = tracker.velocity(s.id);
            if low_from <= s.y - v.y * dt + 0.5
                && low_to >= s.y - 0.01
                && landing.is_none_or(|(best, _)| s.y < best.y)
            {
                landing = Some((*s, v));
            }
        }
        if landing.is_none()
            && low_to > arena.bottom() + 1.0
            && let Some(floor) = world.floor_under(body.center.x).copied()
        {
            landing = Some((floor, Vec2::ZERO));
        }
        let Some((surface, platform)) = landing else {
            self.place(&body, rigid);
            return;
        };
        body.center.y = surface.y - ey;
        // Il monte encore : c'est un coin qui touche la surface parce qu'il
        // tourne (une culbute qui part du ras du sol). Il s'appuie sur ce
        // coin, sans s'y poser.
        if rigid.vel.y - platform.y < -1.0 {
            collide_static(
                &mut rigid,
                lowest - body.center,
                Vec2::new(0.0, -1.0),
                platform,
                0.0,
                GROUND_FRICTION,
            );
            self.place(&body, rigid);
            return;
        }
        let impact = rigid.vel.y - platform.y;

        // Retrouvé tout de suite, sans choc : il reprend ce qu'il faisait.
        if self.grace && impact < GRACE_IMPACT {
            self.place(&body, rigid);
            self.grace = false;
            self.free_fall = false;
            self.set_ground(&surface);
            self.carry = platform;
            self.vel = Vec2::new(rigid.vel.x - platform.x, 0.0);
            self.pos.y = surface.y;
            return;
        }
        // Un animal desktopPet dans une animation scriptée décide de ce
        // qu'il fait d'une surface : s'y poser, ou passer devant. (Un animal
        // Mouflon, lui, s'y pose toujours.)
        if scripted && self.free_fall && def.rules == Rules::DesktopPet {
            self.place(&body, rigid);
            if self.land_script(&surface, impact, world, input) {
                self.free_fall = false;
                self.grace = false;
                self.carry = platform;
                self.vel = Vec2::ZERO;
            }
            return;
        }
        self.grace = false;
        let bounce = if impact > p.bounce_min_speed {
            p.bounce
        } else {
            0.0
        };
        collide_static(
            &mut rigid,
            lowest - body.center,
            Vec2::new(0.0, -1.0),
            platform,
            bounce,
            GROUND_FRICTION,
        );
        self.place(&body, rigid);
        self.touch_down(impact.max(0.0));
        if bounce > 0.0 && rigid.vel.y - platform.y < -60.0 {
            // Il rebondit : encore en l'air.
            return;
        }
        // Il se pose : sa vitesse devient relative à la surface.
        self.pos.y = surface.y;
        self.vel = Vec2::new(rigid.vel.x - platform.x, 0.0);
        self.settle(&surface, impact.max(0.0), world, input);
        self.carry = platform;
        self.free_fall = false;
    }

    /// Le dos de cet animal, sur lequel il ne peut pas se poser lui-même.
    fn own_back(&self) -> Option<SurfaceId> {
        Some(pet_surface(self.id))
    }

    /// En l'air, son corps reste au-dessus du sol (un choc avec un autre
    /// peut l'y enfoncer).
    pub(super) fn keep_above_floor(&mut self, world: &World) {
        if self.support != Support::Air {
            return;
        }
        let body = self.body();
        if let Some(floor) = world.floor_under(body.center.x) {
            let below = body.center.y + body.extent_y() - floor.y;
            if below > 0.0 {
                self.pos.y -= below;
            }
        }
    }

    /// Reprend la position, l'angle et les vitesses du corps.
    fn place(&mut self, body: &Obb, rigid: Rigid) {
        self.pos = body.center + Vec2::new(0.0, body.half.y);
        self.angle = body.angle;
        self.vel = rigid.vel;
        self.spin = rigid.spin.clamp(-MAX_SPIN, MAX_SPIN);
        // Les grands angles restent lisibles : on garde l'angle près de zéro.
        if self.angle.abs() > TAU * 4.0 {
            self.angle -= TAU * (self.angle / TAU).trunc();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::parse_pet_toml;
    use crate::math::Rect;
    use crate::pet::{Flock, TICK};
    use crate::world::{LedgeOptions, Screen, WindowRect};

    const PET: &str = r#"
        [pet]
        id = "r"
        name = "R"
        [appearance]
        rig = "a.svg"
        height = 60.0
        [physics]
        width = 70.0
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
        duration = "forever"
        [[behavior]]
        name = "walk"
        motion = "walk"
        speed = 60.0
        duration = "forever"
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

    fn def() -> Arc<crate::PetDef> {
        Arc::new(parse_pet_toml(PET, |_: &str| Ok(b"<svg/>".to_vec())).unwrap())
    }

    fn screen() -> Screen {
        Screen {
            frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }
    }

    fn world(window: Option<Rect>) -> World {
        let mut world = World::new(vec![screen()]);
        poll(&mut world, window);
        world
    }

    fn poll(world: &mut World, window: Option<Rect>) {
        let windows: Vec<WindowRect> = window
            .map(|frame| WindowRect { id: 9, frame })
            .into_iter()
            .collect();
        world.set_windows(&windows, LedgeOptions::default());
    }

    /// Un animal livré avec Mouflon.
    fn bundled(name: &str) -> Arc<crate::PetDef> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../pets/{name}/pet.toml"));
        Arc::new(crate::format::load_pet(&path).unwrap())
    }

    /// Un troupeau en physique réaliste, avec un animal posé sur la fenêtre.
    fn on_window(window: Rect, behavior: &str) -> (Flock, World, u32) {
        on_window_with(def(), window, behavior)
    }

    fn on_window_with(
        def: Arc<crate::PetDef>,
        window: Rect,
        behavior: &str,
    ) -> (Flock, World, u32) {
        let world = world(Some(window));
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let id = flock.spawn(def, 1, &world, Some(window.x + window.w * 0.5));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let pet = flock.get_mut(id).unwrap();
        assert!(
            matches!(pet.support(), Support::Ground { surface } if surface >> 8 == 9),
            "posé sur la fenêtre : {:?} à {:?}",
            pet.support(),
            pet.position()
        );
        assert!(pet.force(behavior));
        (flock, world, id)
    }

    /// Fait bouger la fenêtre de `velocity` pendant `seconds`, relevée
    /// toutes les 33 ms comme dans l'app.
    fn drag(flock: &mut Flock, world: &mut World, window: &mut Rect, velocity: Vec2, seconds: f32) {
        let steps = (seconds / TICK) as usize;
        for step in 0..steps {
            if step % 4 == 0 {
                window.x += velocity.x * TICK * 4.0;
                window.y += velocity.y * TICK * 4.0;
                poll(world, Some(*window));
            }
            flock.advance(TICK, world, &Input::default());
        }
    }

    fn pet(flock: &Flock, id: u32) -> &Pet {
        flock.pets().iter().find(|p| p.id() == id).unwrap()
    }

    #[test]
    fn a_window_dragged_calmly_carries_its_pet() {
        let mut window = Rect::new(300.0, 400.0, 600.0, 300.0);
        let (mut flock, mut world, id) = on_window(window, "stand");
        let start = pet(&flock, id).position().x - window.x;
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(150.0, 40.0),
            1.5,
        );
        drag(&mut flock, &mut world, &mut window, Vec2::ZERO, 0.5);
        let pet = pet(&flock, id);
        assert!(
            matches!(pet.support(), Support::Ground { .. }),
            "{:?}",
            pet.support()
        );
        let offset = pet.position().x - window.x;
        assert!(
            (offset - start).abs() < 6.0,
            "il a glissé de {}",
            offset - start
        );
        assert!((pet.position().y - window.y).abs() < 1.0);
    }

    #[test]
    fn a_window_seen_moving_after_a_calm_spell_takes_its_pet_along() {
        // Relevée toutes les 250 ms au calme, la fenêtre qu'on commence à
        // tirer est vue d'un coup 120 pt plus loin : on ne sait pas comment
        // elle est partie, l'animal la suit sans glisser ni décoller.
        let mut window = Rect::new(300.0, 400.0, 600.0, 300.0);
        let (mut flock, mut world, id) = on_window(window, "stand");
        let start = pet(&flock, id).position().x - window.x;
        window.x += 120.0;
        poll(&mut world, Some(window));
        // Puis relevée toutes les 33 ms, comme dans l'app quand une fenêtre
        // qui porte un animal bouge : elle ne bouge plus.
        drag(&mut flock, &mut world, &mut window, Vec2::ZERO, 0.5);
        let carried = pet(&flock, id);
        assert!(
            matches!(carried.support(), Support::Ground { .. }),
            "{:?}",
            carried.support()
        );
        let offset = carried.position().x - window.x;
        assert!((offset - start).abs() < 2.0, "{start} → {offset}");
        // Montée d'un coup de même, un relevé en retard : il ne s'envole pas.
        window.y -= 120.0;
        poll(&mut world, Some(window));
        let mut highest = f32::MAX;
        for _ in 0..(0.5 / TICK) as usize {
            drag(&mut flock, &mut world, &mut window, Vec2::ZERO, TICK);
            highest = highest.min(pet(&flock, id).position().y);
        }
        assert!(highest > window.y - 8.0, "{highest} {}", window.y);
    }

    #[test]
    fn a_window_pulled_down_fast_lets_its_pet_fall() {
        let mut window = Rect::new(300.0, 300.0, 600.0, 300.0);
        let (mut flock, mut world, id) = on_window(window, "stand");
        let y0 = pet(&flock, id).position().y;
        // Tirée vers le bas à 2500 pt/s : plus vite que la pesanteur.
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(0.0, 2500.0),
            0.12,
        );
        let falling = pet(&flock, id);
        assert_eq!(falling.support(), Support::Air);
        assert!(falling.position().y < window.y - 10.0, "il reste au-dessus");
        assert!(falling.position().y > y0, "il tombe");
        // Il la rattrape une fois arrêtée.
        drag(&mut flock, &mut world, &mut window, Vec2::ZERO, 1.5);
        let landed = pet(&flock, id);
        assert!(matches!(landed.support(), Support::Ground { .. }));
        assert!((landed.position().y - window.y).abs() < 1.0);
    }

    #[test]
    fn a_window_raised_then_stopped_tosses_its_pet() {
        let mut window = Rect::new(300.0, 600.0, 600.0, 200.0);
        let (mut flock, mut world, id) = on_window(window, "stand");
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(0.0, -900.0),
            0.25,
        );
        let mut highest = f32::MAX;
        let mut flew = false;
        for _ in 0..(1.0 / TICK) as usize {
            drag(&mut flock, &mut world, &mut window, Vec2::ZERO, TICK);
            let p = pet(&flock, id);
            flew |= p.support() == Support::Air;
            highest = highest.min(p.position().y);
        }
        assert!(flew, "il a décollé");
        assert!(
            highest < window.y - 5.0,
            "au-dessus du rebord : {highest} {}",
            window.y
        );
        assert!(
            matches!(pet(&flock, id).support(), Support::Ground { .. }),
            "retombé"
        );
    }

    #[test]
    fn a_jerked_window_slides_under_its_pet() {
        let mut window = Rect::new(300.0, 400.0, 600.0, 300.0);
        let (mut flock, mut world, id) = on_window(window, "stand");
        let start = pet(&flock, id).position().x - window.x;
        // Un coup sec : 4000 pt/s pendant 70 ms.
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(4000.0, 0.0),
            0.07,
        );
        drag(&mut flock, &mut world, &mut window, Vec2::ZERO, 1.0);
        let offset = pet(&flock, id).position().x - window.x;
        assert!(
            offset < start - 40.0,
            "la fenêtre a filé sous lui : {start} → {offset}"
        );
    }

    #[test]
    fn an_image_by_image_pet_feels_its_window_too() {
        // Lainou est animée image par image : la fenêtre l'emporte par ses
        // pattes comme les autres (pas collée à elle comme en physique
        // classique).
        let mut window = Rect::new(300.0, 500.0, 600.0, 250.0);
        let (mut flock, mut world, id) = on_window_with(bundled("lainou"), window, "dort");
        let start = pet(&flock, id).position().x - window.x;
        // Tirée d'un coup sec vers la gauche : elle file sous lui.
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(-1000.0, 0.0),
            0.15,
        );
        drag(&mut flock, &mut world, &mut window, Vec2::ZERO, 0.4);
        let sleeper = pet(&flock, id);
        let offset = sleeper.position().x - window.x;
        assert!(
            offset > start + 30.0,
            "la fenêtre a filé sous lui : {start} → {offset}"
        );
        assert!(matches!(sleeper.support(), Support::Ground { .. }));
        // Montée vite puis arrêtée net : elle le lance en l'air, et il
        // retombe dessus (sans passer à travers).
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(0.0, -900.0),
            0.25,
        );
        let mut highest = f32::MAX;
        for _ in 0..(1.5 / TICK) as usize {
            drag(&mut flock, &mut world, &mut window, Vec2::ZERO, TICK);
            highest = highest.min(pet(&flock, id).position().y);
        }
        assert!(highest < window.y - 10.0, "{highest} {}", window.y);
        let landed = pet(&flock, id);
        assert!(
            matches!(landed.support(), Support::Ground { surface } if surface >> 8 == 9),
            "{:?} à {:?}",
            landed.support(),
            landed.position()
        );
        assert!((landed.position().y - window.y).abs() < 1.0);
    }

    #[test]
    fn a_desktoppet_sheep_still_climbs_in_realistic_physics() {
        // Son animation le fait grimper le long du bord de l'écran : elle le
        // mène, la pesanteur ne le fait pas retomber.
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let pelote = bundled("pelote");
        let id = flock.spawn(Arc::clone(&pelote), 3, &world, Some(700.0));
        let floor = *world.floor_under(10.0).unwrap();
        {
            let pet = flock.get_mut(id).unwrap();
            pet.pos = Vec2::new(pelote.physics.width * 0.5, floor.y);
            pet.prev_pos = pet.pos;
            pet.set_ground(&floor);
            pet.facing = -1.0;
            assert!(pet.force("vertical_walk_up"));
        }
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let climber = pet(&flock, id);
        assert_eq!(climber.behavior_name(), "vertical_walk_up");
        assert!(
            climber.position().y < floor.y - 30.0,
            "il grimpe : {:?}",
            climber.position()
        );
    }

    #[test]
    fn held_off_center_it_hangs_tilted_then_spins_when_thrown() {
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let id = flock.spawn(def(), 3, &world, Some(700.0));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        // Pris par l'épaule droite.
        let body = pet(&flock, id).body();
        let grip = body.center + Vec2::new(body.half.x * 0.8, -body.half.y * 0.8);
        flock.get_mut(id).unwrap().grab(grip, &world);
        for _ in 0..(1.5 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let hanging = pet(&flock, id);
        let roll = hanging.view(1.0).pose.roll;
        // Pris par l'épaule droite, le corps pend sous la main : il tourne
        // dans le sens inverse des aiguilles d'une montre, sans aller jusqu'à
        // l'aplomb (il se raidit).
        assert!(roll < -0.15 && roll > -0.86, "{roll}");
        // Lancé vers la droite d'un geste : il part avec l'élan et tourne.
        let mut hand = grip;
        for _ in 0..12 {
            hand.x += 15.0;
            let pet = flock.get_mut(id).unwrap();
            pet.drag_to(hand);
            flock.advance(TICK, &world, &Input::default());
        }
        let pet = flock.get_mut(id).unwrap();
        pet.release(&world);
        assert_eq!(pet.support(), Support::Air);
        assert!(pet.velocity().x > 800.0, "{:?}", pet.velocity());
        let mut turned = 0.0_f32;
        let mut last = pet.view(1.0).pose.roll;
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
            let roll = flock.pets()[0].view(1.0).pose.roll;
            turned = turned.max((roll - last).abs());
            last = roll;
        }
        let landed = &flock.pets()[0];
        assert!(matches!(landed.support(), Support::Ground { .. }));
        assert_eq!(landed.view(1.0).pose.roll, 0.0, "debout");
    }

    #[test]
    fn thrown_at_the_screen_edge_it_bounces_back() {
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let id = flock.spawn(def(), 4, &world, Some(1300.0));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let pet = flock.get_mut(id).unwrap();
        pet.support = Support::Air;
        pet.ground_ref = None;
        pet.vel = Vec2::new(1500.0, -800.0);
        let mut bounced = false;
        for _ in 0..(0.5 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
            bounced |= flock.pets()[0].velocity().x < -100.0;
        }
        assert!(bounced, "il repart vers la gauche");
        assert!(flock.pets()[0].position().x < 1440.0);
    }

    #[test]
    fn two_pets_meeting_in_the_air_trade_their_momentum() {
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        flock.set_collisions(true);
        let a = flock.spawn(def(), 5, &world, Some(400.0));
        let b = flock.spawn(def(), 6, &world, Some(800.0));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        for (id, x, vx) in [(a, 500.0, 600.0), (b, 700.0, -600.0)] {
            let pet = flock.get_mut(id).unwrap();
            pet.support = Support::Air;
            pet.ground_ref = None;
            pet.pos = Vec2::new(x, 400.0);
            pet.prev_pos = pet.pos;
            pet.vel = Vec2::new(vx, 0.0);
        }
        for _ in 0..(0.2 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        assert!(pet(&flock, a).velocity().x < 0.0, "a repart à gauche");
        assert!(pet(&flock, b).velocity().x > 0.0, "b repart à droite");
        assert!(pet(&flock, a).position().x < pet(&flock, b).position().x);
    }

    #[test]
    fn a_pet_dropped_on_another_rides_on_its_back() {
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        flock.set_collisions(true);
        let below = flock.spawn(def(), 7, &world, Some(600.0));
        let above = flock.spawn(def(), 8, &world, Some(200.0));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let x = pet(&flock, below).position().x;
        let pet_above = flock.get_mut(above).unwrap();
        pet_above.support = Support::Air;
        pet_above.ground_ref = None;
        pet_above.pos = Vec2::new(x, 500.0);
        pet_above.prev_pos = pet_above.pos;
        pet_above.vel = Vec2::ZERO;
        for _ in 0..(1.5 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        assert_eq!(
            pet(&flock, above).support(),
            Support::Ground {
                surface: pet_surface(below)
            }
        );
        // Celui du dessous se met à marcher : il emporte l'autre.
        assert!(flock.get_mut(below).unwrap().force("walk"));
        let x0 = pet(&flock, above).position().x;
        for _ in 0..(1.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let moved = pet(&flock, above).position().x - x0;
        assert!(moved.abs() > 20.0, "il n'a pas bougé ({moved})");
    }

    #[test]
    fn a_desktoppet_sheep_falls_for_real_when_its_window_drops() {
        use crate::desktoppet::{import, tests::sample};
        let sheep = Arc::new(import(&sample()).unwrap());
        let mut window = Rect::new(300.0, 300.0, 600.0, 200.0);
        let mut world = world(Some(window));
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let id = flock.spawn(sheep, 11, &world, Some(600.0));
        let ledge = *world
            .surfaces()
            .iter()
            .find(|s| s.kind == SurfaceKind::Window)
            .unwrap();
        {
            let pet = flock.get_mut(id).unwrap();
            pet.pos = Vec2::new(600.0, ledge.y);
            pet.prev_pos = pet.pos;
            pet.set_ground(&ledge);
            assert!(pet.force("walk"));
        }
        for _ in 0..(0.5 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        assert!(matches!(pet(&flock, id).support(), Support::Ground { .. }));
        // La fenêtre descend d'un coup : il tombe sous la pesanteur, de plus
        // en plus vite (pas à la vitesse constante d'une animation).
        drag(
            &mut flock,
            &mut world,
            &mut window,
            Vec2::new(0.0, 3000.0),
            0.1,
        );
        let speeds: Vec<f32> = (0..20)
            .map(|_| {
                flock.advance(TICK, &world, &Input::default());
                pet(&flock, id).velocity().y
            })
            .collect();
        assert_eq!(pet(&flock, id).support(), Support::Air);
        assert!(speeds.windows(2).all(|w| w[1] >= w[0] - 1e-3), "{speeds:?}");
        assert!(speeds[19] > speeds[0] + 100.0, "{speeds:?}");
        for _ in 0..(3.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        let landed = pet(&flock, id);
        assert!(
            matches!(landed.support(), Support::Ground { .. }),
            "{:?}",
            landed.support()
        );
    }

    #[test]
    fn classic_physics_is_unchanged_by_the_switch() {
        // Passer en physique réaliste puis revenir : l'animal se redresse et
        // reprend la physique d'avant.
        let world = world(None);
        let mut flock = Flock::new();
        flock.set_realistic(true);
        let id = flock.spawn(def(), 9, &world, Some(700.0));
        for _ in 0..(2.0 / TICK) as usize {
            flock.advance(TICK, &world, &Input::default());
        }
        flock.get_mut(id).unwrap().spin = 5.0;
        flock.get_mut(id).unwrap().angle = 1.0;
        flock.set_realistic(false);
        flock.advance(TICK, &world, &Input::default());
        assert_eq!(pet(&flock, id).view(1.0).pose.roll, 0.0);
        assert!(!flock.realistic());
    }

    /// Coût de la physique réaliste : 32 animaux, collisions, fenêtres.
    /// `cargo test --release -p mouflon-core cost_of -- --ignored --nocapture`
    #[test]
    #[ignore = "mesure, à lancer à la main"]
    fn cost_of_realistic_physics() {
        let mut world = world(Some(Rect::new(300.0, 400.0, 600.0, 300.0)));
        let mut windows: Vec<WindowRect> = (0..12)
            .map(|i| WindowRect {
                id: 100 + i,
                frame: Rect::new(60.0 * i as f32, 200.0 + 30.0 * i as f32, 400.0, 300.0),
            })
            .collect();
        world.set_windows(&windows, LedgeOptions::default());
        for realistic in [false, true] {
            let mut flock = Flock::new();
            flock.set_realistic(realistic);
            flock.set_collisions(true);
            for i in 0..32 {
                flock.spawn(def(), 60 + i, &world, Some(40.0 + 42.0 * i as f32));
            }
            let steps = (20.0 / TICK) as usize;
            let started = std::time::Instant::now();
            for step in 0..steps {
                if step % 4 == 0 {
                    windows[0].frame.x = 300.0 + 200.0 * (step as f32 * TICK).sin();
                    world.set_windows(&windows, LedgeOptions::default());
                }
                flock.advance(TICK, &world, &Input::default());
            }
            let per_step = started.elapsed().as_secs_f64() / steps as f64 * 1e6;
            eprintln!(
                "{} : {per_step:.1} µs par pas de 1/120 s pour 32 animaux",
                if realistic { "réaliste" } else { "classique" }
            );
        }
    }

    #[test]
    fn realistic_runs_stay_deterministic() {
        let run = || {
            let world = world(Some(Rect::new(300.0, 400.0, 600.0, 300.0)));
            let mut flock = Flock::new();
            flock.set_realistic(true);
            flock.set_collisions(true);
            for i in 0..6 {
                flock.spawn(def(), 40 + i, &world, Some(200.0 + 180.0 * i as f32));
            }
            for _ in 0..(8.0 / TICK) as usize {
                flock.advance(TICK, &world, &Input::default());
            }
            flock
                .pets()
                .iter()
                .map(|p| (p.position(), p.behavior_name().to_owned()))
                .collect::<Vec<_>>()
        };
        assert_eq!(format!("{:?}", run()), format!("{:?}", run()));
    }
}
