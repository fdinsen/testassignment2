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