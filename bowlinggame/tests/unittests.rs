#[cfg(test)]
mod tests {
    extern crate bowlinggame;
    use bowlinggame::{Game};

    #[test]
    fn test_gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 0);
        assert_eq!(0, game.score());
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 1);
        assert_eq!(20, game.score());
    }

    #[test]
    fn test_spare() {
        let mut game = Game::new();
        roll_spare(&mut game);
        game.roll(3);
        roll_many(&mut game, 17, 0);
        assert_eq!(16, game.score());
    }

    #[test]
    fn test_strike() {
        let mut game = Game::new();
        roll_strike(&mut game);
        game.roll(3);
        game.roll(4);
        roll_many(&mut game, 16, 0);
        assert_eq!(24, game.score());
    }

    #[test]
    fn test_perfect_game() {
        let mut game = Game::new();
        roll_many(&mut game, 12, 10);
        assert_eq!(300, game.score());
    }

    #[test]
    fn test_single_roll() {
        let mut game = Game::new();
        game.roll(3);
        assert_eq!(3, game.score());
    }

    #[test]
    fn test_two_rolls() {
        let mut game = Game::new();
        roll_many(&mut game, 2, 3);
        assert_eq!(6, game.score());
    }

    #[test]
    fn test_three_rolls() {
        let mut game = Game::new();
        roll_many(&mut game, 3, 1);
        assert_eq!(3, game.score());
    }

    #[test]
    fn test_11_rolls() {
        let mut game = Game::new();
        roll_many(&mut game, 11, 1);
        assert_eq!(11, game.score());
    }

    #[test]
    fn test_too_many_rolls_strikes() {
        let mut game = Game::new();
        roll_many(&mut game, 12, 10);
        let res = game.roll(10); //13th roll
        assert!(res.is_err());
    }

    #[test]
    fn test_too_many_rolls_no_strikes() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 3);
        let res = game.roll(5); //21st roll
        assert!(res.is_err());
    }

    #[test]
    fn test_max_rolls_spares() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 5);
        let res = game.roll(5); //21st roll
        assert!(!res.is_err());
    }

    #[test]
    fn test_roll_too_many_pins() {
        let mut game = Game::new();
        let res = game.roll(11); 
        assert!(res.is_err());
    }

    #[test]
    fn test_roll_example_game() {
        let mut game = Game::new();
        game.roll(1);  //Frame 1
        game.roll(4); 
        game.roll(4);  //Frame 2
        game.roll(5); 
        game.roll(6);  //Frame 3
        game.roll(4);
        game.roll(5);  //Frame 4
        game.roll(5); 
        game.roll(10); //Frame 5
        game.roll(0);  //Frame 6
        game.roll(1); 
        game.roll(7);  //Frame 7
        game.roll(3); 
        game.roll(6);  //Frame 8
        game.roll(4); 
        game.roll(10); //Frame 9
        game.roll(2);  //Frame 10
        game.roll(8); 
        let res = game.roll(6); 
        assert_eq!(133, game.score());
    }

    #[test]
    fn test_roll_example_game_too_many() {
        let mut game = Game::new();
        game.roll(1);  //Frame 1
        game.roll(4); 
        game.roll(4);  //Frame 2
        game.roll(5); 
        game.roll(6);  //Frame 3
        game.roll(4);
        game.roll(5);  //Frame 4
        game.roll(5); 
        game.roll(10); //Frame 5
        game.roll(0);  //Frame 6
        game.roll(1); 
        game.roll(7);  //Frame 7
        game.roll(3); 
        game.roll(6);  //Frame 8
        game.roll(4); 
        game.roll(10); //Frame 9
        game.roll(2);  //Frame 10
        game.roll(8); 
        game.roll(6); 
        let res = game.roll(1);
        assert!(res.is_err());
    }

    //Helper function
    fn roll_many(game: &mut Game, n: i32, pins: i32) {
        for _ in 0..n {
            game.roll(pins);
        }
    }
    fn roll_strike(game: &mut Game) {
        game.roll(10);
    }
    fn roll_spare(game: &mut Game) {
        game.roll(5);
        game.roll(5);
    }
}