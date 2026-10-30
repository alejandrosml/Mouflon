//! Rendu de Mouflon.
//!
//! - [`rig`] : lecture d'un dessin SVG découpé en calques nommés ;
//! - [`animate`] : animation procédurale des calques à partir des signaux du moteur ;
//! - [`paint`] : dessin complet d'un animal (ombre, lumière, particules) ;
//! - [`display`] : liste d'affichage intermédiaire ;
//! - [`vello_scene`] et [`svg`] : traduction vers Vello (écran) ou SVG (aperçu).
#![forbid(unsafe_code)]

pub mod animate;
pub mod bake;
pub mod display;
pub mod paint;
pub mod rig;
pub mod sprites;
pub mod svg;
pub mod vello_scene;

pub use display::DisplayList;
pub use paint::{Canvas, Frame, Look, PetArt, SpriteMotion, Style, paint_pet};
pub use rig::{Part, Rig};
pub use vello;
