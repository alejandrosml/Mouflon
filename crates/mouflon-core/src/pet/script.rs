//! Animations scriptées façon desktopPet.
//!
//! Le corps suit le déplacement prévu à chaque pas de la séquence ; les bords
//! (côtés et haut de l'écran, sol, rebords de fenêtres) déclenchent les
//! transitions, et la « gravité » fait passer à l'animation de chute quand
//! plus rien ne porte l'animal. Là où desktopPet avance par à-coups à chaque
//! pas, Mouflon répartit le déplacement sur la durée du pas : le mouton
//! glisse à 120 Hz.

use std::sync::Arc;

use super::{Input, LANDING_REACH, Pet, PetEvent, Support};
use crate::def::{Motion, Rules, Transition};
use crate::math::Vec2;
use crate::world::{Surface, SurfaceKind, World};

/// Sous cet écart avec le sol, l'animal s'y pose au lieu de tomber (desktopPet : 3 pixels).
const SNAP: f32 = 3.0;

/// Un rebord trop près du haut de l'écran n'arrête pas une chute (desktopPet : 20 pixels).
const LEDGE_HEADROOM: f32 = 20.0;

impl Pet {
    pub(super) fn step_script(&mut self, dt: f32, world: &World, input: &Input) {
        let def = Arc::clone(&self.def);
        let behavior = &def.behaviors[self.behavior];
        let p = def.physics;
        let half = p.width * 0.5;

        // Appui : une fenêtre qui bouge emporte l'animal, une fenêtre qui
        // disparaît le fait tomber.
        let mut support = None;
        if let Support::Ground { surface } = self.support {
            let Some(s) = world.surface(surface).copied() else {
                self.lift_off(world, input);
                return;
            };
            if s.kind == SurfaceKind::Window {
                self.ride(&s);
            }
            self.pos.y = s.y;
            support = Some(s);
        }

        // Déplacement du pas en cours : celui de l'image affichée, ou interpolé
        // sur T − 1 pas (le pas supplémentaire de fin prolonge la droite, comme
        // desktopPet).
        let total = self.clip_total.max(1);
        let step = self.clip_step.min(total);
        let per_step = match (&behavior.motion, &behavior.clip) {
            (Motion::Script(script), Some(clip)) if script.steps.is_some() => {
                let steps = script.steps.as_deref().unwrap_or_default();
                let k = clip.frame_index_at(step.min(total - 1));
                steps.get(k).copied().unwrap_or(Vec2::ZERO) * script.scale
            }
            _ => {
                let [a, b] = self.script;
                if total > 1 {
                    a + (b - a) * (step as f32 / (total - 1) as f32)
                } else {
                    a
                }
            }
        };
        let interval = self.clip_interval();
        let v = Vec2::new(per_step.x * self.facing, per_step.y) * (1.0 / interval);
        self.vel = v;
        let dx = v.x * dt;
        let mut dy = v.y * dt;
        let arena = world.arena();
        let native = def.rules == Rules::Mouflon;

        // Bords dans le sens de la marche : côté de l'écran, bout du rebord.
        if dx != 0.0 {
            let dir = dx.signum();
            let front = self.pos.x + dir * half;
            let wall = if dir < 0.0 {
                arena.left()
            } else {
                arena.right()
            };
            if (front + dx - wall) * dir > 0.0 {
                if let Some(next) = self.pick(&behavior.on_wall, &def.reflexes.wall, world, input) {
                    self.pos.x = wall - dir * half;
                    self.enter(next);
                    return;
                }
                if native {
                    // Sans réaction prévue, il s'arrête au bord et se retourne s'il y allait.
                    self.stop_at(wall - dir * half, dir);
                    return;
                }
                // desktopPet : sans réaction prévue, il sort de l'écran (et réapparaîtra).
            } else if let Some(s) = support.filter(|s| s.kind == SurfaceKind::Window) {
                let edge = if dir < 0.0 { s.x0 } else { s.x1 };
                if !self.walking_off && (front + dx - edge) * dir > 0.0 {
                    if let Some(next) =
                        self.pick(&behavior.on_edge, &def.reflexes.edge, world, input)
                    {
                        self.pos.x = edge - dir * half;
                        self.enter(next);
                        return;
                    }
                    if native && !behavior.walk_off_edges {
                        self.stop_at(edge - dir * half, dir);
                        return;
                    }
                    // Il continue tout droit et tombera quand son centre aura dépassé le bord.
                    self.walking_off = true;
                }
            }
        }

        // Bords dans le sens vertical.
        if dy > 0.0 {
            let floor = world.floor_under(self.pos.x).copied();
            let on_floor = support.is_some_and(|s| s.kind == SurfaceKind::Floor);
            if let Some(f) = floor
                && (on_floor || self.pos.y + dy > f.y)
            {
                // Le sol (desktopPet : la barre des tâches) arrête toujours la descente.
                if self.land_script(&f, v.y, world, input) {
                    return;
                }
                dy = (f.y - self.pos.y).max(0.0);
                if !on_floor {
                    self.set_ground(&f);
                    support = Some(f);
                }
            } else {
                if support.is_some() {
                    // Il descend du rebord : il le traverse.
                    self.detach();
                    support = None;
                }
                if let Some(w) = self.ledge_crossed(world, dy)
                    && self.land_script(&w, v.y, world, input)
                {
                    return;
                }
            }
        } else if dy < 0.0 {
            if support.is_some() {
                self.detach();
                support = None;
            }
            let head = self.pos.y - p.height;
            if head + dy < arena.top()
                && let Some(next) = self.choose(&behavior.on_ceiling, world, input)
            {
                self.pos.y = arena.top() + p.height;
                self.enter(next);
                return;
            }
        }

        // Gravité : plus rien sous les pattes.
        if behavior.gravity
            && support.is_none()
            && step < total
            && let Some(f) = world.floor_under(self.pos.x).copied()
            && self.pos.y + dy < f.y
        {
            if self.pos.y + dy + SNAP >= f.y {
                dy = f.y - self.pos.y;
                self.set_ground(&f);
            } else {
                self.pos += Vec2::new(dx, dy);
                // Un animal Mouflon tombe comme d'habitude ; desktopPet le fait réapparaître.
                let next = if native {
                    self.pick(&behavior.on_fall, &def.reflexes.fall, world, input)
                        .or(def.default_air())
                } else {
                    self.choose(&behavior.on_fall, world, input)
                };
                match next {
                    Some(next) => self.enter(next),
                    None if self.is_child() => self.finished = true,
                    None => self.respawn(world),
                }
                return;
            }
        }

        self.pos += Vec2::new(dx, dy);

        // Son centre a dépassé le bout de la surface : il quitte un rebord ;
        // au sol, il passe sur l'écran voisin (et en descend s'il est plus bas).
        // Un animal Mouflon tombe ensuite pour de vrai : il attend d'être
        // assez loin pour ne pas se reposer aussitôt sur le rebord.
        let reach = if native { p.width * LANDING_REACH } else { 0.0 };
        if let Some(s) = support
            && !s.contains_x(self.pos.x, 0.0)
        {
            match s.kind {
                SurfaceKind::Window if !s.contains_x(self.pos.x, reach) => self.detach(),
                SurfaceKind::Window => {}
                SurfaceKind::Floor => {
                    if let Some(f) = world.floor_at(self.pos.x, s.y).copied() {
                        if f.y <= s.y + 1.0 {
                            self.pos.y = f.y;
                            self.set_ground(&f);
                        } else {
                            self.detach();
                        }
                    }
                }
            }
        }
    }

    /// Arrêté contre un bord en allant dans le sens `dir` : il se retourne
    /// s'il y allait de face (pas s'il reculait).
    fn stop_at(&mut self, x: f32, dir: f32) {
        self.pos.x = x;
        self.vel.x = 0.0;
        if dir * self.facing > 0.0 {
            self.facing = -self.facing;
        }
    }

    /// Réaction propre au comportement, sinon le réflexe général.
    fn pick(
        &mut self,
        own: &[Transition],
        fallback: &[Transition],
        world: &World,
        input: &Input,
    ) -> Option<usize> {
        let list = if own.is_empty() { fallback } else { own };
        self.choose(list, world, input)
    }

    /// Rebord de fenêtre franchi en descendant de `dy` (le plus haut).
    fn ledge_crossed(&self, world: &World, dy: f32) -> Option<Surface> {
        let top = self.pos.y - self.def.physics.height;
        if top <= world.arena().top() + LEDGE_HEADROOM {
            return None;
        }
        world
            .surfaces()
            .iter()
            .filter(|s| {
                s.kind == SurfaceKind::Window
                    && s.y > self.pos.y
                    && s.y <= self.pos.y + dy
                    && s.contains_x(self.pos.x, 0.0)
            })
            .min_by(|a, b| a.y.total_cmp(&b.y))
            .copied()
    }

    /// Arrivée sur une surface en descendant : la réaction vient de `on_land`
    /// (le sol et les fenêtres peuvent avoir des suites différentes). Renvoie
    /// `true` si l'animal a changé d'animation.
    fn land_script(&mut self, surface: &Surface, speed: f32, world: &World, input: &Input) -> bool {
        let before = (self.support, self.ground_ref, self.walking_off);
        // Les conditions « sur le sol » ou « sur une fenêtre » portent sur la surface d'arrivée.
        self.set_ground(surface);
        let def = Arc::clone(&self.def);
        let Some(next) = self.choose(&def.behaviors[self.behavior].on_land, world, input) else {
            (self.support, self.ground_ref, self.walking_off) = before;
            return false;
        };
        self.pos.y = surface.y;
        self.events.push(PetEvent::Landed { impact: speed });
        // Petit écrasement à l'arrivée, proportionné à la vitesse.
        self.squash_kick((speed * 0.0022).min(1.6));
        if speed > 180.0 {
            self.emit_dust(speed * 1.6);
        }
        self.enter(next);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktoppet::{import, tests::sample};
    use crate::math::Rect;
    use crate::pet::{Flock, TICK};
    use crate::world::{LedgeOptions, Screen, WindowRect};

    fn def() -> Arc<crate::PetDef> {
        Arc::new(import(&sample()).unwrap())
    }

    fn world() -> World {
        World::new(vec![Screen {
            frame: Rect::new(0.0, 0.0, 1440.0, 900.0),
            usable: Rect::new(0.0, 25.0, 1440.0, 800.0),
            scale: 2.0,
        }])
    }

    const FLOOR: f32 = 825.0;

    fn run(pet: &mut Pet, world: &World, seconds: f32) {
        let input = Input {
            cursor: None,
            local_hour: 14.0,
        };
        for _ in 0..(seconds / TICK).round() as usize {
            pet.step(TICK, world, &input);
        }
    }

    /// Animal posé à `x` sur la surface donnée, dans le comportement voulu.
    fn standing(world: &World, x: f32, surface: Surface, behavior: &str, facing: f32) -> Pet {
        let mut pet = Pet::spawn_at(0, def(), 7, world, Vec2::new(x, surface.y));
        pet.pos = Vec2::new(x, surface.y);
        pet.prev_pos = pet.pos;
        pet.set_ground(&surface);
        pet.facing = facing;
        assert!(pet.force(behavior));
        pet
    }

    fn floor(world: &World) -> Surface {
        *world.floor_under(0.0).unwrap()
    }

    #[test]
    fn walks_in_from_beside_the_screen() {
        let world = world();
        let mut pet = Pet::spawn(0, def(), 3, &world, None);
        let start = pet.position();
        assert!(
            start.x < 0.0 || start.x > 1440.0,
            "apparu hors de l'écran : {start:?}"
        );
        assert_eq!(start.y, FLOOR);
        assert!(matches!(pet.support(), Support::Ground { .. }));
        assert_eq!(pet.behavior_name(), "walk");
        run(&mut pet, &world, 3.0);
        // 2 points par pas de 200 ms : 30 points en 3 s, vers l'intérieur.
        let moved = (pet.position().x - start.x).abs();
        assert!((moved - 30.0).abs() < 2.5, "{moved}");
        assert!((pet.position().x - 720.0).abs() < (start.x - 720.0).abs());
        assert_eq!(pet.position().y, FLOOR);
    }

    #[test]
    fn walks_on_to_the_next_screen() {
        let world = World::new(vec![
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
        ]);
        let mut pet = standing(&world, 1430.0, floor(&world), "walk", 1.0);
        run(&mut pet, &world, 2.5);
        assert!(pet.position().x > 1450.0, "{:?}", pet.position());
        assert_eq!(pet.position().y, FLOOR);
        let Support::Ground { surface } = pet.support() else {
            panic!("{:?}", pet.support())
        };
        assert_eq!(world.surface(surface).unwrap().x0, 1440.0);
    }

    #[test]
    fn a_sequence_lasts_one_step_more_than_its_frames() {
        let world = world();
        // rotate : une image, T = 1, donc deux pas de 200 ms, puis demi-tour et marche.
        let mut pet = standing(&world, 700.0, floor(&world), "rotate", -1.0);
        run(&mut pet, &world, 0.35);
        assert_eq!(pet.behavior_name(), "rotate");
        run(&mut pet, &world, 0.1);
        assert_eq!(pet.behavior_name(), "walk");
        assert_eq!(pet.facing, 1.0);
    }

    #[test]
    fn reacts_to_the_side_of_the_screen() {
        let world = world();
        let mut pet = standing(&world, 12.0, floor(&world), "walk", -1.0);
        run(&mut pet, &world, 2.0);
        assert!(
            matches!(pet.behavior_name(), "rotate" | "climb" | "walk"),
            "{}",
            pet.behavior_name()
        );
        assert!(
            pet.position().x >= 2.0,
            "il ne sort pas : {:?}",
            pet.position()
        );
    }

    #[test]
    fn climbs_up_to_the_top_of_the_screen() {
        let world = world();
        let mut pet = standing(&world, 2.0, floor(&world), "climb", -1.0);
        // 2 points tous les 150 ms : environ 60 s pour monter 800 points.
        run(&mut pet, &world, 5.0);
        assert_eq!(pet.support(), Support::Air);
        assert!(
            (pet.position().y - (FLOOR - 66.7)).abs() < 3.0,
            "{:?}",
            pet.position()
        );
        assert_eq!(
            pet.behavior_name(),
            "climb",
            "pas de gravité pendant l'escalade"
        );
        run(&mut pet, &world, 60.0);
        assert_ne!(pet.behavior_name(), "climb", "le haut de l'écran l'arrête");
    }

    #[test]
    fn falls_by_script_and_lands_on_the_floor() {
        let world = world();
        let mut pet = Pet::spawn(0, def(), 5, &world, Some(500.0));
        assert_eq!(pet.behavior_name(), "fall");
        let top = pet.position().y;
        run(&mut pet, &world, 1.0);
        // Chute de desktopPet : lente au début (1 point par pas de 100 ms).
        assert!(pet.position().y - top < 60.0, "{:?}", pet.position());
        run(&mut pet, &world, 12.0);
        assert!(matches!(pet.support(), Support::Ground { .. }));
        assert_eq!(pet.position().y, FLOOR);
        assert!(
            pet.take_events()
                .iter()
                .any(|e| matches!(e, PetEvent::Landed { .. }))
        );
    }

    #[test]
    fn stays_on_a_window_ledge_and_falls_when_it_closes() {
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 9,
                frame: Rect::new(300.0, 400.0, 300.0, 200.0),
            }],
            LedgeOptions::default(),
        );
        let ledge = *world
            .surfaces()
            .iter()
            .find(|s| s.kind == SurfaceKind::Window)
            .unwrap();
        let mut pet = standing(&world, 450.0, ledge, "walk", 1.0);
        run(&mut pet, &world, 30.0);
        assert_eq!(
            pet.position().y,
            400.0,
            "il se retourne ou regarde en bas au bout"
        );
        assert!((300.0..=600.0).contains(&pet.position().x));

        world.set_windows(&[], LedgeOptions::default());
        run(&mut pet, &world, 20.0);
        assert_eq!(pet.position().y, FLOOR, "fenêtre fermée : il tombe");
    }

    #[test]
    fn scenes_bring_a_second_character_that_leaves() {
        let world = world();
        let mut flock = Flock::new();
        let id = flock.spawn(def(), 1, &world, Some(700.0));
        let input = Input::default();
        for _ in 0..(15.0 / TICK) as usize {
            flock.advance(TICK, &world, &input);
        }
        let pet = flock.get_mut(id).unwrap();
        pet.force("eat");
        flock.advance(TICK, &world, &input);
        assert_eq!(flock.pets().len(), 2, "la fleur apparaît");
        let flower = &flock.pets()[1];
        assert!(flower.is_child());
        assert_eq!(flower.behavior_name(), "flower");
        // flower : 4 pas de 100 ms puis plus de suite : elle s'en va.
        for _ in 0..(0.7 / TICK) as usize {
            flock.advance(TICK, &world, &input);
        }
        assert_eq!(flock.pets().len(), 1);
    }

    /// Longue simulation d'un vrai fichier (non livré avec Mouflon) :
    /// `MOUFLON_DESKTOPPET=…/animations.xml cargo test -p mouflon-core long_run -- --ignored --nocapture`
    #[test]
    #[ignore = "nécessite un fichier desktopPet local"]
    fn long_run_with_a_real_file() {
        use std::collections::BTreeMap;
        let Ok(path) = std::env::var("MOUFLON_DESKTOPPET") else {
            return;
        };
        let def = Arc::new(import(&std::fs::read_to_string(path).unwrap()).unwrap());
        let mut world = world();
        let mut flock = Flock::new();
        for seed in 0..6 {
            flock.spawn(Arc::clone(&def), seed * 7919 + 1, &world, None);
        }
        let mut seconds_in: BTreeMap<String, f32> = BTreeMap::new();
        let mut entries: BTreeMap<String, u32> = BTreeMap::new();
        let mut last: Vec<(u32, String)> = Vec::new();
        let mut max_pets = 0;
        let minutes = 60.0;
        for i in 0..(minutes * 60.0 / TICK) as usize {
            let t = i as f32 * TICK;
            // Des fenêtres qui apparaissent, bougent et se ferment.
            let phase = (t / 40.0) as i32 % 3;
            let x = 200.0 + 150.0 * (t * 0.05).sin();
            let windows = match phase {
                0 => vec![
                    WindowRect {
                        id: 1,
                        frame: Rect::new(x, 420.0, 500.0, 300.0),
                    },
                    WindowRect {
                        id: 2,
                        frame: Rect::new(900.0, 250.0, 380.0, 500.0),
                    },
                ],
                1 => vec![WindowRect {
                    id: 2,
                    frame: Rect::new(900.0, 250.0, 380.0, 500.0),
                }],
                _ => Vec::new(),
            };
            world.set_windows(&windows, LedgeOptions::default());
            flock.advance(
                TICK,
                &world,
                &Input {
                    cursor: None,
                    local_hour: 15.0,
                },
            );
            max_pets = max_pets.max(flock.pets().len());
            for pet in flock.pets() {
                let p = pet.position();
                assert!(
                    p.x.is_finite() && p.y.is_finite(),
                    "{}",
                    pet.behavior_name()
                );
                assert!(
                    p.y < 2000.0 && p.y > -2000.0 && p.x > -3000.0 && p.x < 4500.0,
                    "{p:?} {}",
                    pet.behavior_name()
                );
                *seconds_in
                    .entry(pet.behavior_name().to_owned())
                    .or_default() += TICK;
                let key = (pet.id(), pet.behavior_name().to_owned());
                if !last.contains(&key) {
                    last.retain(|(id, _)| *id != pet.id());
                    last.push(key);
                    *entries.entry(pet.behavior_name().to_owned()).or_default() += 1;
                }
            }
        }
        eprintln!("{} : {} pets au plus", def.meta.name, max_pets);
        for (name, count) in &entries {
            eprintln!("{name:>22} : {count:5} entrées, {:7.0} s", seconds_in[name]);
        }
        let never: Vec<&str> = def
            .behaviors
            .iter()
            .map(|b| b.name.as_str())
            .filter(|n| !entries.contains_key(*n))
            .collect();
        eprintln!("jamais joués : {never:?}");
    }

    #[test]
    fn a_thrown_pet_tumbles_and_lands_facing_its_way() {
        let world = world();
        let mut pet = standing(&world, 300.0, floor(&world), "walk", -1.0);
        let start = pet.position();
        pet.grab(start - Vec2::new(0.0, 2.0), &world);
        assert_eq!(pet.behavior_name(), "drag");
        pet.drag_to(start - Vec2::new(0.0, 300.0));
        run(&mut pet, &world, 1.0);
        // Un geste vif vers la droite, loin du bord de l'écran.
        for i in 0..20 {
            pet.drag_to(start + Vec2::new(i as f32 * 6.0, -300.0));
            pet.step(TICK, &world, &Input::default());
        }
        pet.release(&world);
        assert_eq!(pet.behavior_name(), "toss", "lancé avec élan");
        run(&mut pet, &world, 4.0);
        assert_eq!(pet.position().y, FLOOR);
        assert!(
            matches!(pet.behavior_name(), "fall soft" | "fall hard" | "walk"),
            "{}",
            pet.behavior_name()
        );
        assert_eq!(pet.facing, 1.0, "il se pose tourné vers où il allait");
    }

    #[test]
    fn a_gentle_drop_plays_the_fall_animation() {
        let world = world();
        let mut pet = standing(&world, 400.0, floor(&world), "walk", -1.0);
        let start = pet.position();
        pet.grab(start - Vec2::new(0.0, 2.0), &world);
        pet.drag_to(start - Vec2::new(0.0, 200.0));
        run(&mut pet, &world, 2.0);
        pet.release(&world);
        assert_eq!(pet.behavior_name(), "fall");
    }

    /// Scènes d'un vrai fichier (bain, mouton noir, descente des fenêtres) :
    /// `MOUFLON_DESKTOPPET=…/animations.xml cargo test -p mouflon-core scenes_of -- --ignored --nocapture`
    #[test]
    #[ignore = "nécessite un fichier desktopPet local"]
    fn scenes_of_a_real_file() {
        let Ok(path) = std::env::var("MOUFLON_DESKTOPPET") else {
            return;
        };
        let def = Arc::new(import(&std::fs::read_to_string(path).unwrap()).unwrap());
        let mut world = world();
        world.set_windows(
            &[WindowRect {
                id: 3,
                frame: Rect::new(500.0, 420.0, 400.0, 300.0),
            }],
            LedgeOptions::default(),
        );
        let input = Input::default();
        let trace = |flock: &Flock, seconds: f32| {
            let names: Vec<String> = flock
                .pets()
                .iter()
                .map(|p| {
                    format!(
                        "{}{}@({:.0},{:.0})",
                        if p.is_child() { "enfant " } else { "" },
                        p.behavior_name(),
                        p.position().x,
                        p.position().y
                    )
                })
                .collect();
            eprintln!("{seconds:5.1} s : {}", names.join(", "));
        };
        // Les points d'apparition 3 et 4 lancent les scènes du bain et du mouton noir.
        for spawn in 2..def.spawns.len() {
            let mut flock = Flock::new();
            let id = flock.spawn(Arc::clone(&def), 11, &world, Some(100.0));
            let pet = flock.get_mut(id).unwrap();
            pet.place_at(&world, &def.spawns[spawn].clone());
            eprintln!("--- apparition {} : {}", spawn + 1, pet.behavior_name());
            let mut children = 0;
            for i in 0..(90.0 / TICK) as usize {
                flock.advance(TICK, &world, &input);
                children = children.max(flock.pets().len() - 1);
                if i % (5.0 / TICK) as usize == 0 {
                    trace(&flock, i as f32 * TICK);
                }
            }
            assert!(children >= 1, "la scène a son personnage secondaire");
            assert_eq!(flock.pets().len(), 1, "il est reparti");
        }
        // Descente d'une fenêtre : regard vers le bas, saut, ou suspendu au rebord.
        let ledge = *world
            .surfaces()
            .iter()
            .find(|s| s.kind == SurfaceKind::Window)
            .unwrap();
        for name in ["jump_down", "fall_wina"] {
            let mut flock = Flock::new();
            let id = flock.spawn(Arc::clone(&def), 5, &world, Some(100.0));
            let pet = flock.get_mut(id).unwrap();
            pet.pos = Vec2::new(ledge.x0 + 20.0, ledge.y);
            pet.prev_pos = pet.pos;
            pet.set_ground(&ledge);
            pet.facing = -1.0;
            assert!(pet.force(name));
            eprintln!("--- {name}");
            for i in 0..(12.0 / TICK) as usize {
                flock.advance(TICK, &world, &input);
                if i % (0.5 / TICK) as usize == 0 {
                    trace(&flock, i as f32 * TICK);
                }
            }
            assert_eq!(flock.pets()[0].position().y, FLOOR, "il finit au sol");
        }
    }

    #[test]
    fn a_main_pet_without_follow_up_reappears() {
        let world = world();
        let mut pet = standing(&world, 700.0, floor(&world), "flower", 1.0);
        run(&mut pet, &world, 0.6);
        // Comme dans desktopPet : nouvelle apparition au point d'apparition.
        assert_eq!(pet.behavior_name(), "walk");
        assert!(pet.position().x < 0.0 || pet.position().x > 1440.0);
    }
}
