use core::{net, panic};
use std::{collections::HashSet, ops::Range, time::SystemTime};

use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    ml_snake::snake,
    neural_network::{NeuralNetwork, NeuralNetworkManager},
    simulation::game::GameStepOutcome, utils::build_neural_network,
};

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
        round: u32,
        network_manager: &mut NeuralNetworkManager,
        networks: &mut Vec<NeuralNetwork>,
    ) {

        let mut winning_network_indexes = HashSet::new();

        println!("Starting nets count {}", networks.len());

        for i in 0..games_count {
            #[cfg(feature = "benchmark_game")]
            let start = SystemTime::now();

            println!("Starting game {}/{} of round {}", i + 1, games_count, round);

            let Some(networks_for_game) = &mut networks.get_mut(
                (i as usize * snakes_count as usize)
                    ..(i as usize * snakes_count as usize + snakes_count as usize),
            ) else {
                panic!("invalid networks splice");
            };

            #[cfg(debug_game)]
            println!("nets len {}", networks_for_game.len());

            let mut game_wrapper = GameWrapper::new(width, height, snakes_count);

            let game_outcome = game_wrapper
                .play_for_outcome(&mut networks_for_game.to_vec())
                .await;

            println!("{:?}", game_outcome);

            match game_outcome {
                GameStepOutcome::Winner(snake_id) => {
                    let Ok(relative_id) = snake_id.parse::<usize>() else {
                        panic!("invalid snake id");
                    };

                    let absolute_id = relative_id + i as usize;

                    winning_network_indexes.insert(absolute_id);

                    // let Some(network) = &mut networks.get_mut(id) else {
                    //     panic!("invalid network");
                    // };
                }
                GameStepOutcome::Tie => {}
                GameStepOutcome::None => {
                    panic!("invalid game outcome");
                }
            }

            #[cfg(feature = "benchmark_game")]
            let duration = SystemTime::now().duration_since(start).unwrap().as_millis();
            #[cfg(feature = "benchmark_game")]
            info!("game {} took {}ms", i + 1, duration);
        }

        #[cfg(debug_simulation)]
        println!("before retain {}", networks.len());
        #[cfg(debug_simulation)]
        println!("{:?}", winning_network_indexes);

        let mut i = 0;
        networks.retain(|_| {
            i += 1;

            winning_network_indexes.contains(&i)
        });

        #[cfg(debug_simulation)]
        println!("after retain {}", networks.len());
    }

    pub async fn run_tournament(
        &mut self,
        games_count: u32,
        width: i32,
        height: u32,
        snakes_count: u32,
        rounds_count: u32,
    ) {
        let mut network_manager = NeuralNetworkManager::new();
        let mut networks: Vec<NeuralNetwork> = Vec::new();

        for _ in 0..games_count {
            for _ in 0..snakes_count {
                let mut network = NeuralNetwork::new(&mut network_manager);
                build_neural_network(&mut network, width, height);
                networks.push(network);
            }
        }

        for round in 0..rounds_count {
            self.run_games(
                games_count,
                width,
                height,
                snakes_count,
                round,
                &mut network_manager,
                &mut networks,
            )
            .await;

            #[cfg(debug_simulation)]
            println!("remaining networks count {}", networks.len());

            self.reproduce_networks(&mut network_manager, &mut networks, games_count, snakes_count);
            self.train_networks(&mut networks);
        }
    }

    fn reproduce_networks(
        &self,
        network_manager: &mut NeuralNetworkManager,
        networks: &mut Vec<NeuralNetwork>,
        games_count: u32,
        snakes_count: u32,
    ) {
        let mut new_networks = Vec::new();
        let mut i = networks.len() as u32;

        while i < games_count * snakes_count {
            let index = i as usize % networks.len();
            let Some(network) = networks.get(index) else {
                panic!("invalid network");
            };

            new_networks.push(network.clone(network_manager));

            i += 1;
        }

        #[cfg(debug_simulation)]
        println!("addtional networks count {}", new_networks.len());

        networks.extend(new_networks);
        networks.shuffle(&mut thread_rng());
    }

    fn train_networks(&self, networks: &mut Vec<NeuralNetwork>) {
        for network in networks {
            network.mutate();
        }
    }
}
