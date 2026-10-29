//! Définition d'un animal, indépendante du format de fichier d'origine
//! (TOML de Mouflon ou XML de desktopPet).

use std::collections::HashMap;
use std::sync::Arc;

use crate::expr::Expr;
use crate::math::Vec2;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PetMeta {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
}

/// Apparence de l'animal.
#[derive(Clone, Debug)]
pub enum Appearance {
    /// Dessin vectoriel (SVG) découpé en calques nommés, animé de façon procédurale.
    Rig { svg: Arc<[u8]>, height: f32 },
    /// Planche de sprites (format desktopPet).
    Sprites(SpriteSheet),
}

/// Sens dans lequel les images d'une planche sont dessinées.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Facings {
    /// Tournées vers la gauche (desktopPet) : retournées en miroir vers la droite.
    #[default]
    Left,
    /// Tournées vers la droite : retournées en miroir vers la gauche.
    Right,
    /// Chaque image existe dans les deux sens, rien n'est retourné : dans
    /// chaque variante de couleur, la seconde moitié des cases regarde vers
    /// la droite (même ordre que la première).
    Both,
}

/// Effets que les images dessinent déjà : Mouflon ne les ajoute pas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Drawn {
    /// Ombre au sol (Mouflon garde la sienne quand l'animal est en l'air).
    pub shadow: bool,
    /// Poussière de course et d'atterrissage.
    pub dust: bool,
    /// Écrasement et étirement.
    pub squash: bool,
    /// Culbute d'un animal lancé ou heurté : les images tournent déjà.
    pub roll: bool,
}

/// Une image PNG découpée en cases de même taille.
#[derive(Clone, Debug)]
pub struct SpritePage {
    pub png: Arc<[u8]>,
    pub tiles_x: u32,
    pub tiles_y: u32,
}

/// Images intermédiaires entre deux images d'une animation, pour un
/// mouvement plus fluide (calculées par `tools/inbetween`).
#[derive(Clone, Debug, Default)]
pub struct Inbetweens {
    /// Leurs planches, de mêmes cases que la planche principale : dans le
    /// premier sens, puis dans l'autre si les images existent dans les deux.
    pub pages: Vec<SpritePage>,
    /// (image de départ, image d'arrivée) → (première image intermédiaire,
    /// nombre d'images). Numéros du premier sens ; ceux des images
    /// intermédiaires suivent toutes les cases de la planche principale.
    pub pairs: HashMap<(u32, u32), (u32, u32)>,
}

impl Inbetweens {
    /// Nombre total de cases de leurs planches.
    #[must_use]
    pub fn tile_count(&self) -> u32 {
        self.pages.iter().map(|p| p.tiles_x * p.tiles_y).sum()
    }
}

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    /// Planches dans l'ordre : les numéros des cases se suivent de l'une à
    /// l'autre (une seule pour desktopPet, une par bande d'animation sinon).
    pub pages: Vec<SpritePage>,
    /// Couleur rendue transparente (le magenta des planches desktopPet).
    pub key_color: Option<[u8; 3]>,
    /// Facteur d'affichage : 1.0 = un pixel de la planche par point.
    pub scale: f32,
    /// Variantes de couleur : la planche contient autant de blocs de cases
    /// consécutifs, et chaque animal en reçoit un (1 = une seule apparence).
    pub skins: u32,
    pub facings: Facings,
    /// Point d'appui (milieu des pattes) dans une case, en pixels, image
    /// tournée vers la gauche ; sans valeur, le milieu du bas de la case.
    pub anchor: Option<[f32; 2]>,
    pub drawn: Drawn,
    /// Images intermédiaires, s'il y en a.
    pub inbetweens: Option<Arc<Inbetweens>>,
}

impl SpriteSheet {
    /// Planche unique façon desktopPet.
    #[must_use]
    pub fn single(png: Arc<[u8]>, tiles_x: u32, tiles_y: u32) -> Self {
        Self {
            pages: vec![SpritePage {
                png,
                tiles_x,
                tiles_y,
            }],
            key_color: None,
            scale: 1.0,
            skins: 1,
            facings: Facings::Left,
            anchor: None,
            drawn: Drawn::default(),
            inbetweens: None,
        }
    }

    /// Images intermédiaires pour aller de l'image `from` à l'image `to`
    /// (numéros du premier sens) : première image et nombre d'images.
    #[must_use]
    pub fn inbetween(&self, from: u32, to: u32) -> Option<(u32, u32)> {
        self.inbetweens
            .as_ref()?
            .pairs
            .get(&(from, to))
            .copied()
            .filter(|&(_, count)| count > 0)
    }

    /// Nombre total de cases.
    #[must_use]
    pub fn tile_count(&self) -> u32 {
        self.pages.iter().map(|p| p.tiles_x * p.tiles_y).sum()
    }

    /// Taille d'une case en pixels, lue dans l'en-tête de la première planche.
    #[must_use]
    pub fn tile_size(&self) -> Option<(u32, u32)> {
        let page = self.pages.first()?;
        let (w, h) = png_size(&page.png)?;
        Some((w / page.tiles_x.max(1), h / page.tiles_y.max(1)))
    }

    /// Cases que désignent les comportements : celles de la première variante
    /// de couleur, dans le premier sens.
    #[must_use]
    pub fn frames_per_facing(&self) -> u32 {
        let facings = if self.facings == Facings::Both { 2 } else { 1 };
        self.tile_count() / (self.skins.max(1) * facings)
    }

    /// Point d'appui dans une case (pixels), image tournée vers la gauche.
    #[must_use]
    pub fn anchor_or_default(&self) -> [f32; 2] {
        self.anchor.unwrap_or_else(|| {
            let (w, h) = self.tile_size().unwrap_or((0, 0));
            [w as f32 * 0.5, h as f32]
        })
    }
}

/// Largeur et hauteur d'un PNG, lues dans son en-tête.
#[must_use]
pub fn png_size(png: &[u8]) -> Option<(u32, u32)> {
    if png.len() < 24 || &png[..8] != b"\x89PNG\r\n\x1a\n" || &png[12..16] != b"IHDR" {
        return None;
    }
    let w = u32::from_be_bytes(png[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(png[20..24].try_into().ok()?);
    Some((w, h))
}

/// Paramètres physiques, en points et secondes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Physics {
    pub gravity: f32,
    pub terminal_velocity: f32,
    /// Freinage horizontal dans les airs (1/s).
    pub air_drag: f32,
    /// Accélération pour atteindre la vitesse de marche (pt/s²).
    pub walk_accel: f32,
    /// Freinage d'une glissade au sol (pt/s²).
    pub ground_friction: f32,
    /// Coefficient de rebond à l'atterrissage.
    pub bounce: f32,
    /// Vitesse d'impact minimale pour rebondir.
    pub bounce_min_speed: f32,
    /// Au-delà de cette vitesse d'impact, l'atterrissage est « dur ».
    pub hard_landing_speed: f32,
    pub max_throw_speed: f32,
    /// Boîte de l'animal : largeur et hauteur en points.
    pub width: f32,
    pub height: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            gravity: 2200.0,
            terminal_velocity: 1700.0,
            air_drag: 0.6,
            walk_accel: 520.0,
            ground_friction: 1100.0,
            bounce: 0.22,
            bounce_min_speed: 520.0,
            hard_landing_speed: 1150.0,
            max_throw_speed: 2600.0,
            width: 76.0,
            height: 64.0,
        }
    }
}

/// Ce que fait le corps pendant un comportement.
#[derive(Clone, Debug, PartialEq)]
pub enum Motion {
    Stand,
    Walk {
        speed: f32,
    },
    Run {
        speed: f32,
    },
    /// Demi-tour, puis enchaînement.
    Turn,
    Sit,
    Sleep,
    /// Regarde autour de lui (ou le curseur).
    Look,
    /// Saut : vitesse horizontale vers l'avant et verticale vers le haut.
    Jump {
        vx: f32,
        vy: f32,
    },
    Fall,
    /// Récupération après un atterrissage.
    Land,
    /// Tenu par le curseur.
    Held,
    /// Déplacement image par image façon desktopPet (voir [`Script`]).
    Script(Arc<Script>),
}

/// Animation desktopPet : à chaque pas de la séquence, le corps avance de
/// `x` points (vers l'avant) et descend de `y` points (vers le haut si négatif),
/// valeurs interpolées du premier au dernier pas. Pas de gravité : la chute
/// elle-même est une animation, et les bords déclenchent les transitions.
///
/// Les expressions sont recalculées à chaque début de séquence (elles peuvent
/// dépendre de `random`, de la taille de l'écran ou de l'image).
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    /// `[x, y]` au premier pas.
    pub start: [Expr; 2],
    /// `[x, y]` au dernier pas.
    pub end: [Expr; 2],
    /// Déplacement propre à chaque image de la séquence (x vers l'avant, y
    /// vers le bas), dans l'ordre de ses images ; remplace `start` et `end`.
    pub steps: Option<Arc<[Vec2]>>,
    /// Facteur appliqué aux valeurs calculées (taille d'affichage de l'animal).
    pub scale: f32,
}

impl Script {
    /// Déplacement constant à chaque pas.
    #[must_use]
    pub fn constant(x: f32, y: f32, scale: f32) -> Self {
        let pair = || [Expr::constant(f64::from(x)), Expr::constant(f64::from(y))];
        Self {
            start: pair(),
            end: pair(),
            steps: None,
            scale,
        }
    }
}

impl Motion {
    /// Le comportement se déroule dans les airs.
    #[must_use]
    pub fn is_airborne(&self) -> bool {
        matches!(self, Self::Fall | Self::Jump { .. })
    }

    #[must_use]
    pub fn pose(&self) -> Pose {
        match self {
            Self::Stand | Self::Turn | Self::Look => Pose::Stand,
            Self::Walk { .. } => Pose::Walk,
            Self::Run { .. } => Pose::Run,
            Self::Sit => Pose::Sit,
            Self::Sleep => Pose::Sleep,
            Self::Jump { .. } | Self::Fall => Pose::Air,
            Self::Land => Pose::Land,
            Self::Held => Pose::Dangle,
            // Les animaux desktopPet portent leurs poses dans leurs images.
            Self::Script(_) => Pose::Stand,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pose {
    Stand,
    Walk,
    Run,
    Sit,
    Sleep,
    Air,
    Land,
    Dangle,
}

/// Durée d'un comportement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Duration {
    /// Durée tirée au hasard dans l'intervalle (secondes).
    Range(f32, f32),
    /// Jusqu'à la fin de la séquence de sprites.
    Clip,
    /// Jusqu'à ce qu'un événement (atterrissage, bord…) le termine.
    Forever,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Condition {
    OnWindow,
    OnFloor,
    /// Entre 22 h et 6 h, heure locale.
    Night,
    Day,
    CursorNear(f32),
    CursorFar(f32),
    /// Au sol (sur une fenêtre ou en bas de l'écran).
    Grounded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    pub to: usize,
    pub weight: f32,
    pub when: Vec<Condition>,
}

/// Séquence de sprites façon desktopPet.
///
/// Elle compte `T` pas (voir [`SpriteClip::total_steps_with`]) suivis d'un
/// pas de plus qui garde la dernière image, comme dans desktopPet.
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteClip {
    pub frames: Vec<u32>,
    /// Durée d'un pas au début et à la fin de la séquence (secondes).
    pub interval_start: f32,
    pub interval_end: f32,
    /// Nombre de répétitions après le premier passage.
    pub repeat: u32,
    /// Répétitions calculées à chaque début de séquence (expression desktopPet).
    pub repeat_expr: Option<Expr>,
    /// Le nombre de répétitions change à chaque fois (`random`…) : desktopPet
    /// compte alors les pas un peu différemment.
    pub dynamic_repeat: bool,
    /// Indice de la première image répétée.
    pub repeat_from: usize,
    /// Décalage vertical d'affichage au début et à la fin (points, positif vers le bas).
    pub offset_y: f32,
    pub offset_y_end: f32,
    pub opacity_start: f32,
    pub opacity_end: f32,
}

impl SpriteClip {
    /// Nombre de pas de la séquence.
    #[must_use]
    pub fn total_steps(&self) -> usize {
        self.total_steps_with(self.repeat)
    }

    /// Nombre de pas pour un nombre de répétitions donné, selon les règles de
    /// desktopPet : `F + F·R` sans `repeatfrom`, sinon `F + (F − RF − 1)·R`
    /// (ou `F + (F − RF)·R` si les répétitions sont tirées au hasard).
    #[must_use]
    pub fn total_steps_with(&self, repeat: u32) -> usize {
        let n = self.frames.len();
        let from = self.repeat_from.min(n.saturating_sub(1));
        let looped = if from == 0 {
            n
        } else if self.dynamic_repeat {
            n - from
        } else {
            n - from - 1
        };
        n + repeat as usize * looped
    }

    /// Image affichée au pas `step`.
    #[must_use]
    pub fn frame_at(&self, step: usize) -> u32 {
        self.frames
            .get(self.frame_index_at(step))
            .copied()
            .unwrap_or(0)
    }

    /// Position dans `frames` de l'image affichée au pas `step`.
    #[must_use]
    pub fn frame_index_at(&self, step: usize) -> usize {
        let n = self.frames.len();
        if n == 0 || step < n {
            return step.min(n.saturating_sub(1));
        }
        let from = self.repeat_from.min(n - 1);
        let looped = n - from;
        from + (step - n) % looped.max(1)
    }
}

/// Un son de l'animal : une ou plusieurs prises (fichiers audio : MP3, AAC,
/// WAV…), l'une tirée au hasard à chaque fois.
#[derive(Clone, Debug)]
pub struct Sound {
    pub name: String,
    pub clips: Vec<Arc<[u8]>>,
    /// Volume, de 0 à 1.
    pub volume: f32,
}

/// Un son qu'un comportement fait entendre.
#[derive(Clone, Debug, PartialEq)]
pub struct SoundCue {
    /// Rang dans [`PetDef::sounds`].
    pub sound: usize,
    /// Chance de l'entendre à chaque occasion (de 0 à 1).
    pub chance: f32,
    /// Image de la séquence (rang dans ses images) qui le déclenche, à chaque
    /// passage ; sans valeur, au début du comportement.
    pub frame: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Behavior {
    pub name: String,
    pub motion: Motion,
    pub duration: Duration,
    pub next: Vec<Transition>,
    /// Remplace le réflexe `edge` pendant ce comportement.
    pub on_edge: Vec<Transition>,
    /// Remplace le réflexe `wall` pendant ce comportement.
    pub on_wall: Vec<Transition>,
    /// Remplace le réflexe `fall` quand le sol disparaît pendant ce comportement.
    pub on_fall: Vec<Transition>,
    /// Remplace les réflexes `land` et `land_hard` à l'atterrissage.
    pub on_land: Vec<Transition>,
    /// En montant, l'animal touche le haut de l'écran (animations scriptées).
    pub on_ceiling: Vec<Transition>,
    /// Remplace le réflexe `meet` quand l'animal en rencontre un autre
    /// pendant ce comportement (`petSide` de desktopPet). Une suite de poids
    /// nul n'est jamais tirée au sort, mais l'animal peut imiter l'autre
    /// s'il est en train de la jouer.
    pub on_meet: Vec<Transition>,
    /// Remplace le réflexe `window_side` quand le côté d'une fenêtre barre le
    /// chemin pendant ce comportement (`windowSide` de desktopPet).
    pub on_window_side: Vec<Transition>,
    /// Animations scriptées : vérifie à chaque pas que l'animal a encore un appui
    /// et passe sinon à `on_fall` (élément `gravity` de desktopPet).
    pub gravity: bool,
    /// Fait demi-tour à la fin du comportement (action `flip` de desktopPet).
    pub flip_at_end: bool,
    /// Sans transition de bord, continue tout droit et tombe du rebord au lieu de se retourner.
    pub walk_off_edges: bool,
    pub clip: Option<SpriteClip>,
    /// Personnages secondaires qui apparaissent avec ce comportement
    /// (la fleur que le mouton mange, le mouton noir…).
    pub children: Vec<ChildSpawn>,
    /// Sons qu'il fait entendre.
    pub sounds: Vec<SoundCue>,
}

/// Personnage secondaire (élément `child` de desktopPet). Il joue ses propres
/// animations puis disparaît quand sa séquence se termine sans suite.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildSpawn {
    /// Coin haut gauche de son image, relatif à l'écran ; `imageX` et `imageY`
    /// désignent la position du parent.
    pub x: Expr,
    /// Variante de `x` quand le parent est retourné (`imageW` change de signe).
    pub x_flipped: Expr,
    pub y: Expr,
    pub next: usize,
}

/// Point d'apparition (format desktopPet) : coin haut gauche de l'image.
#[derive(Clone, Debug, PartialEq)]
pub struct SpawnPoint {
    pub x: Expr,
    pub y: Expr,
    pub weight: f32,
    pub next: usize,
}

/// Réactions aux événements physiques.
#[derive(Clone, Debug, Default)]
pub struct Reflexes {
    pub spawn: Vec<Transition>,
    pub fall: Vec<Transition>,
    pub land: Vec<Transition>,
    pub land_hard: Vec<Transition>,
    pub held: Vec<Transition>,
    pub thrown: Vec<Transition>,
    pub edge: Vec<Transition>,
    pub wall: Vec<Transition>,
    pub poke: Vec<Transition>,
    /// Heurté par un autre animal (mode collisions) ; à défaut, `thrown`.
    pub bump: Vec<Transition>,
    /// Rencontre d'un autre animal en marchant (hors mode collisions). Sans
    /// réaction, les deux animaux se croisent.
    pub meet: Vec<Transition>,
    /// Le côté d'une fenêtre barre le chemin. Sans réaction, l'animal passe
    /// devant la fenêtre.
    pub window_side: Vec<Transition>,
    /// Au revoir, quand on retire l'animal (`kill` de desktopPet) : il joue
    /// ce comportement, puis s'efface.
    pub leave: Vec<Transition>,
    /// Tous ensemble : tous les animaux jouent ce comportement en même temps
    /// (`sync` de desktopPet).
    pub together: Vec<Transition>,
}

/// Règles du moteur pour ce qui n'est pas prévu par l'animal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Rules {
    /// Animal Mouflon : sans suite prévue, il revient à son comportement au
    /// sol ; sans réaction prévue, il fait demi-tour au bord et tombe quand
    /// le sol disparaît.
    #[default]
    Mouflon,
    /// Animal desktopPet : comme le moteur d'origine, une séquence compte un
    /// pas de plus ; sans suite, l'animal réapparaît ; sans réaction prévue,
    /// il sort de l'écran ou tombe du rebord.
    DesktopPet,
}

#[derive(Clone, Debug)]
pub struct PetDef {
    pub meta: PetMeta,
    pub appearance: Appearance,
    pub physics: Physics,
    pub behaviors: Vec<Behavior>,
    pub reflexes: Reflexes,
    /// Points d'apparition ; vide : l'animal tombe du haut de l'écran.
    pub spawns: Vec<SpawnPoint>,
    pub rules: Rules,
    /// Sons de l'animal (voir [`Behavior::sounds`]).
    pub sounds: Vec<Sound>,
    by_name: HashMap<String, usize>,
    default_ground: usize,
    default_air: Option<usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum DefError {
    #[error("l'animal ne définit aucun comportement")]
    NoBehavior,
    #[error("le comportement « {0} » est défini deux fois")]
    Duplicate(String),
    #[error("« {from} » renvoie vers « {to} », qui n'existe pas")]
    UnknownTarget { from: String, to: String },
    #[error("aucun comportement au sol (stand, walk, sit…) n'est défini")]
    NoGroundBehavior,
    #[error("{0}")]
    Invalid(String),
}

impl PetDef {
    /// Assemble et valide une définition.
    pub fn new(
        meta: PetMeta,
        appearance: Appearance,
        physics: Physics,
        behaviors: Vec<Behavior>,
        reflexes: Reflexes,
    ) -> Result<Self, DefError> {
        if behaviors.is_empty() {
            return Err(DefError::NoBehavior);
        }
        let mut by_name = HashMap::with_capacity(behaviors.len());
        for (i, b) in behaviors.iter().enumerate() {
            if by_name.insert(b.name.clone(), i).is_some() {
                return Err(DefError::Duplicate(b.name.clone()));
            }
        }
        let check = |from: &str, list: &[Transition]| -> Result<(), DefError> {
            for t in list {
                if t.to >= behaviors.len() {
                    return Err(DefError::UnknownTarget {
                        from: from.to_owned(),
                        to: format!("#{}", t.to),
                    });
                }
            }
            Ok(())
        };
        for b in &behaviors {
            check(&b.name, &b.next)?;
            check(&b.name, &b.on_edge)?;
            check(&b.name, &b.on_wall)?;
            check(&b.name, &b.on_fall)?;
            check(&b.name, &b.on_land)?;
            check(&b.name, &b.on_ceiling)?;
            check(&b.name, &b.on_meet)?;
            check(&b.name, &b.on_window_side)?;
            if let Some(c) = b.children.iter().find(|c| c.next >= behaviors.len()) {
                return Err(DefError::UnknownTarget {
                    from: b.name.clone(),
                    to: format!("#{}", c.next),
                });
            }
            if let Some(cue) = b.sounds.iter().find(|c| !(0.0..=1.0).contains(&c.chance)) {
                return Err(DefError::Invalid(format!(
                    "« {} » : chance de son {} (attendu entre 0 et 1)",
                    b.name, cue.chance
                )));
            }
        }
        for (name, list) in [
            ("spawn", &reflexes.spawn),
            ("fall", &reflexes.fall),
            ("land", &reflexes.land),
            ("land_hard", &reflexes.land_hard),
            ("held", &reflexes.held),
            ("thrown", &reflexes.thrown),
            ("edge", &reflexes.edge),
            ("wall", &reflexes.wall),
            ("poke", &reflexes.poke),
            ("bump", &reflexes.bump),
            ("meet", &reflexes.meet),
            ("window_side", &reflexes.window_side),
            ("leave", &reflexes.leave),
            ("together", &reflexes.together),
        ] {
            check(name, list)?;
        }
        let default_ground = behaviors
            .iter()
            .position(|b| matches!(b.motion, Motion::Stand | Motion::Look))
            .or_else(|| {
                behaviors
                    .iter()
                    .position(|b| !b.motion.is_airborne() && b.motion != Motion::Held)
            })
            .ok_or(DefError::NoGroundBehavior)?;
        let default_air = behaviors.iter().position(|b| b.motion == Motion::Fall);
        if !(physics.width > 0.0 && physics.height > 0.0) {
            return Err(DefError::Invalid(
                "la taille de l'animal doit être positive".into(),
            ));
        }
        Ok(Self {
            meta,
            appearance,
            physics,
            behaviors,
            reflexes,
            spawns: Vec::new(),
            rules: Rules::Mouflon,
            sounds: Vec::new(),
            by_name,
            default_ground,
            default_air,
        })
    }

    /// Donne ses sons à l'animal ; chaque son désigné par un comportement doit exister.
    pub fn set_sounds(&mut self, sounds: Vec<Sound>) -> Result<(), DefError> {
        for b in &self.behaviors {
            if let Some(cue) = b.sounds.iter().find(|c| c.sound >= sounds.len()) {
                return Err(DefError::Invalid(format!(
                    "« {} » : son #{} inconnu",
                    b.name, cue.sound
                )));
            }
        }
        if let Some(sound) = sounds.iter().find(|s| s.clips.is_empty()) {
            return Err(DefError::Invalid(format!(
                "le son « {} » n'a aucun fichier",
                sound.name
            )));
        }
        self.sounds = sounds;
        Ok(())
    }

    /// Images de l'animal, s'il est fait de sprites.
    #[must_use]
    pub fn sprites(&self) -> Option<&SpriteSheet> {
        match &self.appearance {
            Appearance::Sprites(sheet) => Some(sheet),
            Appearance::Rig { .. } => None,
        }
    }

    /// Effets déjà dessinés dans les images.
    #[must_use]
    pub fn drawn(&self) -> Drawn {
        self.sprites().map_or_else(Drawn::default, |s| s.drawn)
    }

    /// Ajoute des points d'apparition (leurs cibles doivent exister).
    pub fn set_spawns(&mut self, spawns: Vec<SpawnPoint>) -> Result<(), DefError> {
        if let Some(bad) = spawns.iter().find(|s| s.next >= self.behaviors.len()) {
            return Err(DefError::UnknownTarget {
                from: "spawn".into(),
                to: format!("#{}", bad.next),
            });
        }
        self.spawns = spawns;
        Ok(())
    }

    #[must_use]
    pub fn behavior_index(&self, name: &str) -> Option<usize> {
        self.by_name.get(name).copied()
    }

    /// Le même animal, `k` fois plus grand : dessin, corps, vitesses et
    /// pesanteur suivent, pour que tout bouge comme à sa taille d'origine,
    /// en plus grand. Les comportements gardent leur rang.
    #[must_use]
    pub fn scaled(&self, k: f32) -> Self {
        let mut def = self.clone();
        // Pas de taille nulle, négative ou indéfinie ; 1 : rien à changer.
        if k.is_nan() || k <= 0.0 || k == 1.0 {
            return def;
        }
        match &mut def.appearance {
            Appearance::Rig { height, .. } => *height *= k,
            Appearance::Sprites(sheet) => sheet.scale *= k,
        }
        let p = &mut def.physics;
        for length in [
            &mut p.gravity,
            &mut p.terminal_velocity,
            &mut p.walk_accel,
            &mut p.ground_friction,
            &mut p.bounce_min_speed,
            &mut p.hard_landing_speed,
            &mut p.max_throw_speed,
            &mut p.width,
            &mut p.height,
        ] {
            *length *= k;
        }
        for b in &mut def.behaviors {
            match &mut b.motion {
                Motion::Walk { speed } | Motion::Run { speed } => *speed *= k,
                Motion::Jump { vx, vy } => {
                    *vx *= k;
                    *vy *= k;
                }
                Motion::Script(script) => {
                    let mut s = (**script).clone();
                    s.scale *= k;
                    *script = Arc::new(s);
                }
                _ => {}
            }
            if let Some(clip) = &mut b.clip {
                clip.offset_y *= k;
                clip.offset_y_end *= k;
            }
        }
        def
    }

    /// Comportement au sol utilisé quand rien d'autre ne s'applique.
    #[must_use]
    pub fn default_ground(&self) -> usize {
        self.default_ground
    }

    #[must_use]
    pub fn default_air(&self) -> Option<usize> {
        self.default_air
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clip(frames: &[u32], repeat: u32, from: usize) -> SpriteClip {
        SpriteClip {
            frames: frames.to_vec(),
            interval_start: 0.2,
            interval_end: 0.2,
            repeat,
            repeat_expr: None,
            dynamic_repeat: false,
            repeat_from: from,
            offset_y: 0.0,
            offset_y_end: 0.0,
            opacity_start: 1.0,
            opacity_end: 1.0,
        }
    }

    #[test]
    fn sprite_clip_repeats_from_index() {
        // Cas vérifié par les tests de desktopPet : [2,3,4,5], repeatfrom 2, T = 6.
        let mut c = clip(&[2, 3, 4, 5], 1, 2);
        c.dynamic_repeat = true;
        assert_eq!(c.total_steps(), 6);
        let seq: Vec<u32> = (0..c.total_steps()).map(|s| c.frame_at(s)).collect();
        assert_eq!(seq, vec![2, 3, 4, 5, 4, 5]);
    }

    #[test]
    fn total_steps_follow_desktoppet() {
        // walk : 2 images, repeat 20 → 42 pas (et non 40).
        assert_eq!(clip(&[2, 3], 20, 0).total_steps(), 42);
        // kill : 3 images, repeat 20, repeatfrom 1 (fixe) → 23 pas.
        assert_eq!(clip(&[3, 96, 96], 20, 1).total_steps(), 23);
        assert_eq!(clip(&[7], 0, 0).total_steps(), 1);
    }
}
