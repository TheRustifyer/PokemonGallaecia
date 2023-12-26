//! This is the entry point for the Rust library of `Pokémon Gallaecia`.
//! 
//! ## Migration docs from `Godot 3 + GDNative` to `Godot 4 + GDext`
//! #[derive(GodotClass)] automatically registers the type annotated with such derive macro.
//! Therefore, there's no need an explicit add_class() registration call, or a .gdns file as it was the case with `GDNative`.
//! 
//! Before Godot 4.2, you will need to restart the Godot editor for it to take effect.

#![crate_type = "cdylib"]

use godot::prelude::*;

pub mod game;

struct PokemonGallaecia;

#[gdextension]
unsafe impl ExtensionLibrary for PokemonGallaecia {}
