//! Les paires d'animaux assez proches pour se toucher.
//!
//! Comparer chaque animal à tous les autres coûte le carré de leur nombre :
//! un demi-million de paires pour mille moutons, cent vingt fois par
//! seconde. Or deux animaux ne se heurtent, ne se rencontrent et ne se
//! cognent que s'ils se touchent presque : chacun occupe un rectangle (son
//! dessin, son corps tourné, ce qu'il atteint), et seuls comptent ceux dont
//! les rectangles se chevauchent.
//!
//! On les range par bord gauche, puis on balaie : pour chaque animal, on ne
//! regarde que ceux qui commencent avant son bord droit. D'un pas à l'autre,
//! les animaux bougent à peine : le rangement du pas d'avant est presque
//! bon, et le tri (qui profite de l'ordre déjà là) ne coûte presque rien.

use super::Pet;

/// Ce qu'un animal peut bouger pendant qu'on traite les paires d'un pas
/// (séparé d'un autre, posé contre lui) : ajouté autour de chacun, pour ne
/// manquer aucune paire qui se toucherait après coup.
const SLACK: f32 = 16.0;

/// Le rectangle d'un animal, marge comprise.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Span {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
}

impl Span {
    fn of(pet: &Pet) -> Self {
        let p = pet.def.physics;
        let (front, back) = pet.reach.unwrap_or((p.width * 0.5, p.width * 0.5));
        // Son corps tourné tient dans un cercle de ce rayon autour de son milieu.
        let half_diagonal = 0.5 * p.width.hypot(p.height);
        let reach = front.max(back).max(half_diagonal) + SLACK;
        let middle = pet.pos.y - p.height * 0.5;
        Self {
            x0: pet.pos.x - reach,
            x1: pet.pos.x + reach,
            // De son corps tourné à sa hauteur debout ; deux points de plus
            // dessous (deux animaux « au même niveau » se rencontrent).
            y0: (middle - half_diagonal).min(pet.pos.y - p.height) - SLACK,
            y1: (middle + half_diagonal).max(pet.pos.y) + 2.0 + SLACK,
        }
    }

    fn overlaps_y(&self, other: &Self) -> bool {
        self.y0 <= other.y1 && other.y0 <= self.y1
    }
}

/// Les paires proches, refaites à chaque pas.
#[derive(Clone, Debug, Default)]
pub(super) struct Near {
    /// Les animaux (leurs rangs), par bord gauche.
    order: Vec<u32>,
    spans: Vec<Span>,
    pairs: Vec<(u32, u32)>,
}

impl Near {
    /// Les paires `(i, j)` (rangs dans `pets`, `i < j`) dont les rectangles
    /// se chevauchent, dans un ordre qui ne dépend que de leurs places.
    pub(super) fn update(&mut self, pets: &[Pet]) -> &[(u32, u32)] {
        self.spans.clear();
        self.spans.extend(pets.iter().map(Span::of));
        if self.order.len() != pets.len() {
            self.order.clear();
            self.order.extend(0..pets.len() as u32);
        }
        let spans = &self.spans;
        // Tri adaptatif : presque rien à faire quand l'ordre est déjà bon.
        self.order.sort_by(|&a, &b| {
            spans[a as usize]
                .x0
                .total_cmp(&spans[b as usize].x0)
                .then(a.cmp(&b))
        });
        self.pairs.clear();
        for (k, &i) in self.order.iter().enumerate() {
            let a = spans[i as usize];
            for &j in &self.order[k + 1..] {
                let b = spans[j as usize];
                if b.x0 > a.x1 {
                    break;
                }
                if a.overlaps_y(&b) {
                    self.pairs.push((i.min(j), i.max(j)));
                }
            }
        }
        &self.pairs
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::math::{Rect, Vec2};
    use crate::world::{Screen, World};

    fn world() -> World {
        World::new(vec![Screen {
            frame: Rect::new(0.0, 0.0, 3000.0, 2000.0),
            usable: Rect::new(0.0, 0.0, 3000.0, 1900.0),
            scale: 2.0,
        }])
    }

    /// Une foule au hasard : des animaux de tailles, de places, d'angles et
    /// d'étendues variés.
    fn crowd(count: usize, seed: u64) -> Vec<Pet> {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pets/lainou/pet.toml");
        let def = Arc::new(crate::format::load_pet(&path).unwrap());
        let world = world();
        let mut rng = crate::math::Rng::new(seed);
        let mut unit = || (rng.next_u32() >> 8) as f32 / (1u32 << 24) as f32;
        (0..count)
            .map(|i| {
                let mut pet = Pet::spawn(i as u32, Arc::clone(&def), i as u64, &world, None);
                pet.pos = Vec2::new(unit() * 1500.0, 400.0 + unit() * 300.0);
                pet.angle = (unit() - 0.5) * 6.0;
                if unit() < 0.5 {
                    pet.reach = Some((20.0 + unit() * 90.0, 20.0 + unit() * 90.0));
                }
                pet.facing = if unit() < 0.5 { -1.0 } else { 1.0 };
                pet
            })
            .collect()
    }

    /// Ce qui fait se toucher deux animaux, quels que soient leurs états :
    /// leurs dessins à la même hauteur (heurts, rencontres) ou leurs corps
    /// tournés (physique réaliste).
    fn touching(a: &Pet, b: &Pet) -> bool {
        let dx = b.pos.x - a.pos.x;
        let side = if dx == 0.0 { -a.facing } else { dx.signum() };
        let reach = a.reach_toward(side) + b.reach_toward(-side);
        let (ha, hb) = (a.def.physics.height, b.def.physics.height);
        let level = a.pos.y > b.pos.y - hb * 0.8 && b.pos.y > a.pos.y - ha * 0.8;
        let bump = dx.abs() < reach && level;
        let meeting = dx.abs() <= reach && (a.pos.y - b.pos.y).abs() <= 2.0;
        let bodies = crate::physics::obb_contact(&a.body(), &b.body()).is_some();
        bump || meeting || bodies
    }

    #[test]
    fn no_touching_pair_is_missed() {
        for seed in 0..20 {
            let pets = crowd(300, seed);
            let mut near = Near::default();
            let pairs: std::collections::HashSet<(u32, u32)> =
                near.update(&pets).iter().copied().collect();
            let mut touching_pairs = 0;
            for i in 0..pets.len() {
                for j in i + 1..pets.len() {
                    if touching(&pets[i], &pets[j]) {
                        touching_pairs += 1;
                        assert!(
                            pairs.contains(&(i as u32, j as u32)),
                            "paire {i}, {j} manquée"
                        );
                    }
                }
            }
            assert!(touching_pairs > 0);
            // Et bien moins de paires que toutes les paires.
            assert!(
                pairs.len() < pets.len() * (pets.len() - 1) / 2 / 4,
                "{}",
                pairs.len()
            );
        }
    }

    #[test]
    fn pairs_are_listed_once_and_in_the_same_order_each_time() {
        let pets = crowd(200, 7);
        let mut near = Near::default();
        let first = near.update(&pets).to_vec();
        let set: std::collections::HashSet<_> = first.iter().copied().collect();
        assert_eq!(set.len(), first.len(), "une paire en double");
        assert!(first.iter().all(|&(i, j)| i < j));
        // Recalculées (l'ordre de départ est celui du pas d'avant) : les mêmes,
        // dans le même ordre.
        assert_eq!(near.update(&pets), first.as_slice());
        let mut fresh = Near::default();
        assert_eq!(fresh.update(&pets), first.as_slice());
    }
}
