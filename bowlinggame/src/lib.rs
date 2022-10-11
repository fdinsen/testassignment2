mod err;
use err::{BowlingErr};

#[derive(Debug)]
pub struct Game {
    frames: Vec<Frame>, //vec is equivalent to list
    current_frame: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Frame {
    Simple(Option<u8>, Option<u8>), //u8 is unsigned 8-bit integer
    Tenth(Option<u8>, Option<u8>, Option<u8>),
    None,
}

#[derive(Debug)]
pub enum FrameResult {
    Strike,
    Spare(u8),
    Hit(u8, u8),
    FinalHit(u8, u8, u8),
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            frames: vec![Frame::empty_simple(); 9],
            current_frame: 0,
        };
        game.frames.push(Frame::empty_tenth());
        game
    }

    pub fn roll(&mut self, pins: i32) -> Result<(), BowlingErr>{
        let frame = get_frame(&self.frames, self.current_frame);
        if frame == Frame::None {return Err(BowlingErr::NoRollsLeft)}
        self.frames[self.current_frame] = match Frame::roll(frame, pins as u8) {
            Ok(frame) => frame, 
            Err(e) => {
                match e {
                    BowlingErr::NoRollsLeft => {
                        self.current_frame += 1;
                        let nextframe = match self.frames.get(self.current_frame) {
                            None => {
                                return Err(BowlingErr::NoRollsLeft);
                            },
                            Some(nextframe) => nextframe,
                        };
                        Frame::roll(nextframe.clone(), pins as u8)?
                    },
                    BowlingErr::TooManyPins(i) => {return Err(BowlingErr::TooManyPins(i));}
                }
            }
        };
        Ok(())
    }

    pub fn score(&self) -> i32 {
        let mut score: i32 = 0;
        let frames = self.frames.clone();
        for (index, frame) in frames.into_iter().enumerate() {
            match Frame::score(&frame) {
                FrameResult::Hit(r1, r2) => score += i32::from(r1 + r2),
                FrameResult::Spare(_) => {
                    score += 10 + spare_bonus(
                        get_frame(&self.frames,index + 1)
                    );
                },
                FrameResult::Strike => {
                    score += 10 + strike_bonus(
                        &self.frames, 
                        get_frame(&self.frames, index + 1), 
                        index
                    );
                },
                FrameResult::FinalHit(r1, r2, r3) => score += i32::from(r1 + r2 + r3),
            };
        }
        score
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Frame {
    pub fn empty_simple() -> Self {
        Frame::Simple(None, None)
    }

    pub fn empty_tenth() -> Self {
        Frame::Tenth(None, None, None)
    }

    pub fn roll(mut frame: Frame, pins: u8) -> Result<Frame, BowlingErr> {
        if pins > 10 {
            return Err(BowlingErr::TooManyPins(pins));
        };
        match frame {
            Frame::Simple(r1, r2) => {
                frame = Self::roll_simple_frame(pins, r1, r2)?;
            }
            Frame::Tenth(r1, r2, r3) => {
                frame = Self::roll_tenth_frame(pins, r1, r2, r3)?;
            },
            Frame::None => return Err(BowlingErr::NoRollsLeft)
        }
        Ok(frame)
    }

    pub fn score(frame: &Frame) -> FrameResult {
        match frame {
            Frame::Simple(r1, r2) => {
                let r1pins = r1.unwrap_or(0u8);
                let r2pins = r2.unwrap_or(0u8);
                if is_strike(r1pins) {
                    return FrameResult::Strike;
                }
                if is_spare(r1pins, r2pins) {
                    return FrameResult::Spare(r1pins);
                }
                FrameResult::Hit(r1pins, r2pins)
            }
            Frame::Tenth(r1, r2, r3) => {
                let r1pins = r1.unwrap_or(0u8);
                let r2pins = r2.unwrap_or(0u8);
                let r3pins = r3.unwrap_or(0u8);
                FrameResult::FinalHit(r1pins, r2pins, r3pins)
            },
            Frame::None => FrameResult::Hit(0,0)
        }
    }

    fn roll_simple_frame(pins: u8, mut r1: Option<u8>, mut r2: Option<u8>) -> Result<Frame, BowlingErr> {
        let r1pins = match r1 {
            None => {
                r1 = Some(pins);
                return Ok(Frame::Simple(r1, None));
            }
            Some(pin) => {
                if pin == 10u8 {
                    return Err(BowlingErr::NoRollsLeft);
                }
                pin
            }
        };
        match r2 {
            None => {
                if r1pins + pins > 10u8 {
                    return Err(BowlingErr::TooManyPins(r1pins + pins));
                };
                r2 = Some(pins);
                pins
            }
            Some(_) => {
                return Err(BowlingErr::NoRollsLeft);
            }
        };
        Ok(Frame::Simple(r1, r2))
    }

    fn roll_tenth_frame(pins: u8,mut r1: Option<u8>, mut r2: Option<u8>, mut r3: Option<u8>) -> Result<Frame, BowlingErr> {
        let r1pins = match r1 {
            None => {
                r1 = Some(pins);
                return Ok(Frame::Tenth(r1, None, None));
            }
            Some(pin) => pin,
        };
        let r2pins = match r2 {
            None => {
                if r1pins + pins > 20u8 {
                    return Err(BowlingErr::TooManyPins(r1pins + pins));
                };
                r2 = Some(pins);
                return Ok(Frame::Tenth(Some(r1pins), r2, None));
            }
            Some(pin) => pin,
        };
        match r3 {
            None => {
                if r1pins + r2pins + pins > 30u8 {
                    return Err(BowlingErr::TooManyPins(r1pins + pins));
                };
                if is_strike(r2pins) || is_spare(r1pins, r2pins) {
                    r3 = Some(pins);
                    pins
                } else {
                    return Err(BowlingErr::NoRollsLeft);
                }
            }
            Some(_) => return Err(BowlingErr::NoRollsLeft),
        };
        Ok(Frame::Tenth(r1, r2, r3))
    }
}

fn is_strike(pins: u8) -> bool {
    pins == 10u8
}

fn is_spare(r1: u8, r2: u8) -> bool {
    r1 + r2 == 10u8
}

fn spare_bonus(frame: Frame) -> i32 {
    match Frame::score(&frame) {
        FrameResult::Strike => 10,
        FrameResult::Spare(r1) | FrameResult::Hit(r1, _) | FrameResult::FinalHit(r1,_,_) 
            => i32::from(r1),
    }
}

fn strike_bonus(frames: &[Frame], frame: Frame, frame_index: usize) -> i32 {
    match Frame::score(&frame) {
        FrameResult::Strike => {
            10 + spare_bonus(
                get_frame(frames, frame_index+2)
            )
        },
        FrameResult::Spare(_) => 10,
        FrameResult::Hit(r1, r2) | FrameResult::FinalHit(r1, r2, _)  
            => i32::from(r1 + r2),
    }
}

fn get_frame(frames: &[Frame],  frame_index: usize) -> Frame {
    match frames.get(frame_index) {
        None => {
            Frame::None
        },
        Some(frame) => *frame,
    }
}