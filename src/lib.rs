use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    points: u32,
    increment: u32,
    time_multiplier: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            points: 0,
            increment: 1,
            time_multiplier: 1,
        }
    }

    pub fn click(&mut self) {
        self.points += self.increment;
    }

    pub fn get_points(&self) -> u32 {
        self.points
    }

    pub fn upgrade_increment(&mut self) {
        self.points -= 10;  // Assume upgrade costs 10 points
        self.increment += 1;
    }

    pub fn upgrade_time(&mut self) {
        self.points -= 20;  // Assume upgrade costs 20 points
        self.time_multiplier += 1;
    }
}