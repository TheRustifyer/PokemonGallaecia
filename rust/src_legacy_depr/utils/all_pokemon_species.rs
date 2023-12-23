use crate::game::pokemon_specie::PokemonSpecie;

pub fn all_pokemon_species_data() -> Vec<PokemonSpecie> {
    let mut all_pokemon_species: Vec<PokemonSpecie> = Vec::new();

    // Bulbasaur
    let bulbasaur = PokemonSpecie::new_pokemon(1, "Bulbasaur","Planta","Veneno",
        60.0,60.0,
        "",
    );
    all_pokemon_species.push(bulbasaur);

    // Ivysaur
    let ivysaur = PokemonSpecie::new_pokemon(2, "Ivysaur","Planta","Veneno",
    60.0,60.0,
    "",
    );
    all_pokemon_species.push(ivysaur);

    // Venasaur
    let venusaur = PokemonSpecie::new_pokemon(3, "Venasaur","Planta","Veneno",
    60.0,60.0,
    "",
    );
    all_pokemon_species.push(venusaur);

    // Charmander
    let charmander = PokemonSpecie::new_pokemon(4, "Charmander","Fuego","",
    60.0,60.0,
    "",
    );
    all_pokemon_species.push(charmander);

    // Charmeleon
    let charmeleon = PokemonSpecie::new_pokemon(5, "Charmeleon","Fuego","",
    60.0,60.0,
    "",
    );
    all_pokemon_species.push(charmeleon);

    // Charizard
    let charizard = PokemonSpecie::new_pokemon(6, "Charizard","Fuego","Volador",
    60.0,60.0,
    "",
    );
    all_pokemon_species.push(charizard);

    // RETURN
    all_pokemon_species

}