//! Moteur de comportement de Mouflon.
//!
//! Ce crate ne touche à aucune API système. Il simule les animaux dans un
//! repère « bureau » abstrait (points logiques, origine en haut à gauche de
//! l'écran principal, y vers le bas) et fournit à chaque image une
//! [`PetView`] que le rendu transforme en pixels.
//!
//! - [`world`] : écrans, sols et rebords de fenêtres visibles ;
//! - [`def`] : définition d'un animal (comportements, physique, apparence) ;
//! - [`format`] : format natif `pet.toml` ;
//! - [`desktoppet`] : import des animaux desktopPet (eSheep 64) ;
//! - [`pet`] : simulation à pas fixe et signaux d'animation.
#![forbid(unsafe_code)]

pub mod def;
pub mod desktoppet;
pub mod expr;
pub mod format;
pub mod math;
pub mod pet;
pub mod world;

pub use def::{Appearance, Drawn, Facings, PetDef, Pose, Rules, SpritePage, SpriteSheet};
pub use format::{LoadError, load_pet, parse_pet_toml};
pub use math::{Rect, Vec2};
pub use pet::{
    Activity, Flock, Input, Particle, ParticleKind, Pet, PetEvent, PetView, PoseSignals, Shadow,
    Support, TICK,
};
pub use world::{
    DOCK_ID, LedgeOptions, Screen, Surface, SurfaceKind, Wall, WindowRect, World, exact_dock,
};
