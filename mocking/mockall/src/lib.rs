use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
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

    #[test]
    fn can_heal_pokemon() {
        // given
        let pikachu_no_hp = Pokemon { name: "Pikachu".to_owned(), hp: 0 };
        let pikachu_full_hp = Pokemon { name: "Pikachu".to_owned(), hp: 100 };

        let mut nurse_joy = MockNurse::new(); // MockNurse is automatically generated
        nurse_joy.expect_heal() //What do we expect the parameters, the return value and the amount of calls to be?
            .with(eq( //this is the parameter we expect it to be called with
                pikachu_no_hp.clone()
            ))
            .times(1) //amount of times it is called
            .returning(move|_x|  //following is the method body for when heal() is called. _x is parameter
                    Ok(Pokemon {
                        name: _x.name, //can construct returned object from parameter
                        hp: 100
                    })
                );

        let mut pokemon_center = PokemonCenter {
            nurse: Box::new(nurse_joy),
            pokemons: vec![],
        };

        // when
        //using mockall there is technically no need to do assert_eq!. 
        //The assertion is whether the expect_heal function is called with the given constraints set above.
        pokemon_center.accept(pikachu_no_hp);
        let healed_pikachu = pokemon_center.collect().unwrap(); // heal() is called within collect()

        //then (not required)
        assert_eq!(healed_pikachu.hp, pikachu_full_hp.hp);
    }
}