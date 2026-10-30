//! Rendu de Mouflon.
//!
//! - [`rig`] : lecture d'un dessin SVG découpé en calques nommés ;
//! - [`animate`] : animation procédurale des calques à partir des signaux du moteur ;
//! - [`paint`] : dessin complet d'un animal (ombre, lumière, particules) ;
//! - [`display`] : liste d'affichage intermédiaire ;
//! - [`bc7`] : compression des images des animaux pour le GPU ;
//! - [`cache`] : images des animaux déjà prêtes, gardées sur le disque ;
//! - [`vello_scene`] et [`svg`] : traduction vers Vello (écran) ou SVG (aperçu).
#![forbid(unsafe_code)]

pub mod animate;
pub mod bake;
pub mod bc7;
pub mod cache;
pub mod display;
pub mod paint;
pub mod rig;
pub mod sprites;
pub mod svg;
pub mod vello_scene;

pub use display::DisplayList;
pub use paint::{Canvas, Frame, Look, PetArt, SpriteKey, SpriteMotion, Style, paint_pet};
pub use rig::{Part, Rig};
pub use vello;
