use ml_battle_snake::simulation::simulation::{self, Simulation};

#[tokio::main]
async fn main() {
    let mut simulation = Simulation::new();

    let games_count = 20;
    let width = 11;
    let height = 11;
    let snakes_count = 2;
    let rounds_count = 10000;

    simulation.run_tournament(games_count, width, height, snakes_count, rounds_count).await;
}