#[derive(Debug)]
pub struct Game {
    rolls: Vec<u8>,
    current_roll: usize
}

impl Game {
    pub fn new() -> Self {
        Self { rolls: vec![0;21], current_roll:0}
    }
    
    pub fn roll(&mut self, pins: i32) {
        self.rolls[self.current_roll] = pins as u8;
        self.current_roll += 1;
    }

    pub fn score(&self) -> i32 {
        let mut score:i32 = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.is_strike(frame_index) {
                score += 10 + self.strike_bonus(frame_index);
                frame_index += 1;
            }
            else if self.is_spare(frame_index){
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                score += self.sum_balls_in_frame(frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == 10 
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10 
    }

    fn strike_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index+1] as i32 + self.rolls[frame_index+2] as i32
    }

    fn spare_bonus(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index+2] as i32
    }

    fn sum_balls_in_frame(&self, frame_index: usize) -> i32 {
        self.rolls[frame_index] as i32 + 
        self.rolls[frame_index+1] as i32
    }
}
