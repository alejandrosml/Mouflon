//! Une vraie rencontre mère–agneau, échantillonnée du premier pas à la fin
//! de la tétée. Les deux lignes montrent deux âges et les deux orientations.

use super::*;
use mouflon_core::{Age, PetDef, SexRatio, TICK};

const CELL_W: u32 = 440;
const CELL_H: u32 = 250;
const ZOOM: f64 = 2.8;

pub(super) fn render(source: &PetDef, directory: &Path) -> Result<(), Error> {
    let mut list = DisplayList::new(CELL_W * 6, CELL_H * 2);
    list.fill(
        rounded(
            Rect::new(0.0, 0.0, list.width as f32, list.height as f32),
            0.0,
        ),
        Affine::IDENTITY,
        Paint::Solid(rgba(240, 242, 247, 1.0)),
    );
    let mut notes = String::from(
        "# La mère et son agneau\n\nChaque ligne vient de la simulation réelle, à 120 Hz. Le jeune utilise encore le dessin adulte réduit, sans variante anatomique. La scène force le sexe de la mère pour vérifier chaque rig : un pack dessiné comme un bélier conserve donc ici son dessin, ce qui ne constitue pas une variante de brebis.\n\n",
    );
    for (row, weeks, facing) in [(0, 2.0, 1.0), (1, 7.0, -1.0)] {
        let mut mother_def = source.clone();
        mother_def.sex_ratio = SexRatio::FEMALE;
        mother_def.age = Age::Adult;
        let mother_def = Arc::new(mother_def);
        let mut mother_art = PetArt::load(&mother_def)?;
        mother_art.baked = None;
        let lamb_def = Arc::new(mother_def.as_lamb(weeks).ok_or("âge d'agneau invalide")?);
        let mut lamb_art = PetArt::load(&lamb_def)?;
        lamb_art.baked = None;
        let world = World::new(vec![screen()]);
        let input = Input {
            local_hour: 15.0,
            ..Input::default()
        };
        let mut flock = Flock::new();
        let mother = flock.spawn(Arc::clone(&mother_def), 33, &world, Some(450.0));
        fit(&mut flock, mother, &mother_art);
        for _ in 0..360 {
            flock.advance(TICK, &world, &input);
        }
        let ewe = flock.get_mut(mother).ok_or("brebis absente")?;
        ewe.face(facing);
        if !ewe.force("stand") {
            return Err("l'aperçu famille demande un comportement stand".into());
        }
        let lamb = flock
            .spawn_lamb_with(mother, lamb_def, 51, &world)
            .ok_or("cet animal ne peut pas avoir d'agneau")?;
        fit(&mut flock, lamb, &lamb_art);
        let mut next = 0;
        let mut nursing_at = None;
        let mut finished_at = None;
        notes.push_str(&format!(
            "Ligne {} : agneau de {weeks:.0} semaines, mère vers {}.\n\n",
            row + 1,
            if facing > 0.0 {
                "la droite"
            } else {
                "la gauche"
            }
        ));
        for tick in 0..(90.0 / TICK) as u32 {
            let elapsed = tick as f32 * TICK;
            let alpha = flock.advance(TICK, &world, &input);
            let young = flock
                .pets()
                .iter()
                .find(|pet| pet.id() == lamb)
                .ok_or("agneau absent")?;
            let pose = young.view(alpha).pose;
            let nursing = pose.gesture == Some(Gesture::Nurse) && pose.gesture_amount > 0.5;
            if nursing {
                nursing_at.get_or_insert(elapsed);
            }
            if nursing_at.is_some() && !nursing {
                finished_at.get_or_insert(elapsed);
            }
            let since = nursing_at.map_or(-1.0, |start| elapsed - start);
            let capture = match next {
                0 => true,
                1 => elapsed >= 0.6,
                2 => nursing && since >= 0.4,
                3 => nursing && since >= 2.0,
                4 => nursing && since >= 6.0,
                5 => finished_at.is_some_and(|end| elapsed - end >= 0.8),
                _ => false,
            };
            if capture {
                paint_cell(
                    &flock,
                    mother,
                    &mother_art,
                    &lamb_art,
                    alpha,
                    row,
                    next,
                    &mut list,
                );
                let phase = [
                    "arrivée",
                    "approche",
                    "début de tétée",
                    "tétée",
                    "queue qui frétille",
                    "fin de tétée",
                ][next as usize];
                notes.push_str(&format!(
                    "- Colonne {} : {phase}, {elapsed:.2} s.\n",
                    next + 1
                ));
                next += 1;
                if next == 6 {
                    break;
                }
            }
            for pet in flock.pets_mut() {
                pet.take_events();
            }
        }
        if next != 6 {
            return Err(format!(
                "la rencontre à {weeks} semaines n'a pas achevé sa tétée ({} vues)",
                next
            )
            .into());
        }
        notes.push('\n');
    }
    rasterize(&list, &directory.join("famille.png"))?;
    std::fs::write(directory.join("famille.md"), notes)?;
    Ok(())
}

fn fit(flock: &mut Flock, id: u32, art: &PetArt) {
    if let Some(pet) = flock.get_mut(id) {
        if let Some((front, back)) = art.reach() {
            pet.set_reach(front, back);
        }
        if let Some((udder, muzzle)) = art.nursing_points() {
            pet.set_nursing_points(udder, muzzle);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_cell(
    flock: &Flock,
    mother: u32,
    mother_art: &PetArt,
    lamb_art: &PetArt,
    alpha: f32,
    row: u32,
    col: u32,
    list: &mut DisplayList,
) {
    let ewe = flock
        .pets()
        .iter()
        .find(|pet| pet.id() == mother)
        .expect("brebis de la scène");
    let ground = ewe.view(alpha).ground;
    // Centrer le couple, y compris avant son rapprochement : dans le sens
    // gauche, centrer seulement la mère couperait l'agneau de la première case.
    let mut left = f32::INFINITY;
    let mut right = f32::NEG_INFINITY;
    for pet in flock.pets() {
        let art = if pet.id() == mother {
            mother_art
        } else {
            lamb_art
        };
        let view = pet.view(alpha);
        let (front, back) = art.reach().unwrap_or((art.width * 0.5, art.width * 0.5));
        let (before, after) = if view.facing < 0.0 {
            (front, back)
        } else {
            (back, front)
        };
        left = left.min(view.ground.x - before);
        right = right.max(view.ground.x + after);
    }
    let origin = Vec2::new(
        (left + right) * 0.5 - (col as f32 + 0.5) * CELL_W as f32 / ZOOM as f32,
        ground.y - (row as f32 + 0.8) * CELL_H as f32 / ZOOM as f32,
    );
    for pet in flock.pets() {
        let art = if pet.id() == mother {
            mother_art
        } else {
            lamb_art
        };
        paint_pet(
            art,
            &pet.view(alpha),
            &Frame {
                origin,
                scale: ZOOM,
            },
            &Style::default(),
            list,
        );
    }
}
