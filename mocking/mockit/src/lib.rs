
//"Interface" to be mocked
#[cfg_attr(test, mock_it::mock_it)]
trait Nurse {
    fn heal(&self, pokemon: Pokemon) -> Result<Pokemon, String>;
}

#[derive(Debug, PartialEq, Clone)]

pub struct Pokemon {
    name: String,
    hp: i32,
}

struct PokemonCenter {
    nurse: Box<dyn Nurse>,
    pokemons: Vec<Pokemon>,
}

impl PokemonCenter {
    pub fn accept(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    pub fn collect(&mut self) -> Result<Pokemon, String> {
        let pokemon = match self.pokemons.pop() {
            Some(val) => val,
            None => return Err("No pokemon".to_string()),
        };
        self.nurse.heal(pokemon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_it::{any, eq};

    #[test]
    fn can_heal_pokemon() {
        // given
        let pikachu_no_hp = Pokemon { name: "Pikachu".to_owned(), hp: 0 };
        let pikachu_full_hp = Pokemon { name: "Pikachu".to_owned(), hp: 100 };

        let nurse_joy = NurseMock::new(); // NurseMock is automatically generated
        //Here we mock the heal() method
        nurse_joy.when_heal(eq(
            pikachu_no_hp.clone()) // we mock for this parameter only
            )
            .will_return( // we set the return value for given parameter
                Ok(pikachu_full_hp.clone())
            );

        let mut pokemon_center = PokemonCenter {
            nurse: Box::new(nurse_joy.clone()),
            pokemons: vec![],
        };

        // when
        pokemon_center.accept(pikachu_no_hp);
        let healed_pikachu = pokemon_center.collect().unwrap();

        //then
        assert_eq!(healed_pikachu, pikachu_full_hp);
        //With mockit we need to assert that the method was called with the constraints
        assert!(nurse_joy.expect_heal(any()).times(1).called());
    }
}