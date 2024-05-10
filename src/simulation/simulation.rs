use super::game::GameWrapper;

pub struct Simulation {
    pub game_wrappers: Vec<GameWrapper>,
}

impl Simulation {
    pub fn new() -> Self {

     Self {
        game_wrappers: Vec::new(),
     }   
    }

    pub async fn run_games(&mut self, games_count: u32, width: i32, height: u32, snakes_count: u32) {

        for i in 0..games_count {
            self.game_wrappers.push(
                GameWrapper::new(width, height, snakes_count)
            );   
        }

        for game in &mut self.game_wrappers {
            game.play_for_outcome();
        }
    }

    pub async fn run_tournament(&self, games_count: u32) {


    }
}