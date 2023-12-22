//! This is the entry point for the Rust library of `Pokémon Gallaecia`

use godot::prelude::*;

struct PokemonGallaecia;

#[gdextension]
unsafe impl ExtensionLibrary for PokemonGallaecia {}
