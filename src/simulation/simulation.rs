use std::time::SystemTime;

use crate::simulation::game::GameStepOutcome;

use super::game::GameWrapper;

pub struct Simulation {}

impl Simulation {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run_games(
        &mut self,
        games_count: u32,
        width: i32,
        height: u32,
        snakes_count: u32,
    ) {
        for i in 0..games_count {
            #[cfg(feature = "benchmark_game")]
            let start = SystemTime::now();

            println!("Starting game {}/{}", i + 1, games_count);

            let mut game_wrapper = GameWrapper::new(width, height, snakes_count);

            let game_outcome = game_wrapper.play_for_outcome().await;

            println!("{:?}", game_outcome);

            assert_ne!(game_outcome, GameStepOutcome::None);

            #[cfg(feature = "benchmark_game")]
            let duration = SystemTime::now().duration_since(start).unwrap().as_millis();
            #[cfg(feature = "benchmark_game")]
            info!("game {} took {}ms", i + 1, duration);
        }
    }

    pub async fn run_tournament(&self, games_count: u32) {}
}
