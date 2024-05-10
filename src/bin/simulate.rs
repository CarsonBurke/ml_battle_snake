use ml_battle_snake::simulation::simulation::{self, Simulation};

#[tokio::main]
async fn main() {
    let mut simulation = Simulation::new();

    let games_count = 1000;
    let width = 15;
    let height = 15;
    let snakes_count = 2;

    simulation.run_games(games_count, width, height, snakes_count).await;
}