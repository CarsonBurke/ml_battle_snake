use std::{
    collections::{HashMap, HashSet},
    usize,
};

use colored::Colorize;
use rand::{random, Rng};

use crate::{
    ml_snake::logic::{choose_move, get_move},
    neural_network::NeuralNetwork,
    utils::{get_direction, is_out_of_bounds, pack_coord, pack_xy, random_coord, unpack_coord},
    Battlesnake, Board, Coord, Game, GameState,
};

use super::constants::{graphics, SNAKE_STARTING_LENGTH};

pub struct GameWrapper {
    pub turn: i32,
    pub game: Game,
    pub board: Board,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameStepOutcome {
    Winner(String),
    Tie,
    None,
}

#[derive(Debug)]
pub enum CoordType {
    Head,
    BodyUp,
    BodyDown,
    BodyLeft,
    BodyRight,
    Food,
    Hazard,
    Empty,
}

impl GameWrapper {
    pub fn new(width: i32, height: u32, snakes_count: u32) -> Self {
        let mut snakes = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..snakes_count {
            snakes.push(Battlesnake {
                id: i.to_string(),
                name: format!("snake_{}", i),
                health: 100,
                body: Vec::new(),
                head: Coord {
                    x: rng.gen_range(0..width),
                    y: rng.gen_range(0..height as i32),
                },
                latency: "0".to_string(),
                length: SNAKE_STARTING_LENGTH,
                shout: Some("".to_string()),
            });
        }

        let mut food = Vec::new();

        for i in 0..=snakes_count {
            food.push(random_coord(width, height));
        }

        Self {
            turn: 0,
            game: Game {
                id: "0".to_string(),
                ruleset: HashMap::new(),
                timeout: 1000,
            },
            board: Board {
                height,
                width,
                food,
                snakes,
                hazards: Vec::new(),
            },
        }
    }

    pub async fn play_for_outcome(&mut self, networks: &mut Vec<NeuralNetwork>) -> GameStepOutcome {
        #[cfg(feature = "visualize_game")]
        self.visualize();
        #[cfg(debug_game)]
        println!("Snakes: {:?}", self.board.snakes);

        for _turn in self.turn.. {
            let step_outcome = self.turn_step(networks);

            match step_outcome {
                GameStepOutcome::None => continue,
                _ => return step_outcome,
            }
        }

        GameStepOutcome::None
    }

    pub fn turn_step(&mut self, networks: &mut Vec<NeuralNetwork>) -> GameStepOutcome {
        #[cfg(feature = "turn_logs")]
        println!("Running turn {}:", self.turn);

        self.age_snakes();
        self.feed_snakes();
        self.propagate_snakes(networks);
        self.kill_snakes();

        #[cfg(feature = "visualize_game")]
        self.visualize();
        #[cfg(debug_game)]
        println!("Snakes: {:?}", self.board.snakes);

        self.turn += 1;

        match self.board.snakes.len() {
            1 => GameStepOutcome::Winner(self.board.snakes[0].id.clone()),
            0 => GameStepOutcome::Tie,
            _ => GameStepOutcome::None,
        }
    }

    fn feed_snakes(&mut self) {
        let mut snake_ids_by_head: HashMap<i32, String> = HashMap::new();

        for snake in &self.board.snakes {
            let packed_coord = pack_coord(&snake.head, self.board.width);
            snake_ids_by_head.insert(packed_coord, snake.id.clone());
        }

        for food in &self.board.food {
            let packed_coord = pack_coord(food, self.board.width);
            let Some(snake_id) = snake_ids_by_head.get(&packed_coord) else {
                continue;
            };

            for snake in &mut self.board.snakes {
                if &snake.id != snake_id {
                    continue;
                }

                snake.length += 1;
            }
        }
    }

    fn propagate_snakes(&mut self, networks: &mut Vec<NeuralNetwork>) {
        let mut moves = Vec::new();
        let mut index = 0;

        for snake in &self.board.snakes {
            let Ok(id) = snake.id.parse::<usize>() else {
                panic!("invalid snake id");
            };

            let Some(network) = &mut networks.get_mut(id) else {
                panic!("invalid network");
            };

            let chosen_move = choose_move(&self.game, &self.turn, &self.board, &snake, network);
            moves.push((index, chosen_move));

            index += 1;
        }

        // Move all snakes and grow if necessary

        for (index, chosen_move) in moves {
            let snake = &mut self.board.snakes[index];

            let offset = match chosen_move {
                "up" => Coord { x: 0, y: -1 },
                "down" => Coord { x: 0, y: 1 },
                "left" => Coord { x: -1, y: 0 },
                "right" => Coord { x: 1, y: 0 },
                _ => Coord { x: 0, y: 0 },
            };

            assert_ne!(offset, Coord { x: 0, y: 0 }, "invalid move");

            let mut previous = snake.head.clone();

            snake.head.x += offset.x;
            snake.head.y += offset.y;

            for body_part in &mut snake.body {
                let new_previous = body_part.clone();

                body_part.x = previous.x;
                body_part.y = previous.y;

                previous = new_previous;
            }

            if (snake.body.len() as i32 + 1) < snake.length {
                snake.body.push(previous.clone());
            }
        }
    }

    fn kill_snakes(&mut self) {
        // Check out of bounds and kill snakes when necessary

        self.board.snakes.retain(|snake| {
            if is_out_of_bounds(
                snake.head.x,
                snake.head.y,
                self.board.width,
                self.board.height,
            ) {
                return false;
            }

            true
        });

        // check if the snake has collided with any snake's head that is not its own

        // Check for head-on collisions and kill snakes when necessary

        let mut snakes_to_kill = HashSet::new();
        let mut snake_ids_by_head: HashMap<i32, String> = HashMap::new();

        for snake in &self.board.snakes {
            let packed_coord = pack_coord(&snake.head, self.board.width);

            if let Some(other_snake_id) = snake_ids_by_head.get(&packed_coord) {
                snakes_to_kill.insert(snake.id.clone());
                snakes_to_kill.insert(other_snake_id.to_string());

                continue;
            }

            snake_ids_by_head.insert(packed_coord, snake.id.clone());
        }

        // let mut snake_head_coords = HashSet::new(); // HashSet::from_iter(self.board.snakes.iter().map(|snake| snake.head));

        // for snake in &self.board.snakes {
        //     snake_head_coords.insert(snake.head);
        // }

        // // Kill both snakes that collided their heads
        // self.board.snakes.retain(|snake| {
        //     if snake_head_coords.contains(&snake.head) {
        //         return false;
        //     }

        //     true
        // });

        // Check for body collisions and kill snakes when necessary

        let mut snake_body_coords = HashSet::new();

        for snake in &self.board.snakes {
            snake_body_coords.extend(snake.body.clone());
        }

        self.board.snakes.retain(|snake| {
            if snake_body_coords.contains(&snake.head) {
                return false;
            };

            true
        });
    }

    fn age_snakes(&mut self) {
        for snake in &mut self.board.snakes {
            snake.health -= 1;
        }

        self.board.snakes.retain(|snake| snake.health > 0);
    }

    fn visualize(&self) {
        let mut coord_types = Vec::new();

        for x in 0..self.board.width {
            for y in 0..self.board.height {
                coord_types.push(CoordType::Empty);
            }
        }

        for food in &self.board.food {
            coord_types[pack_coord(food, self.board.width) as usize] = CoordType::Food;
        }

        for snake in &self.board.snakes {
            coord_types[pack_coord(&snake.head, self.board.width) as usize] = CoordType::Head;

            let mut previous = snake.head;

            for body_part in &snake.body {
                let direction = get_direction(*body_part, previous);

                coord_types[pack_coord(body_part, self.board.width) as usize] = match direction {
                    "up" => CoordType::BodyUp,
                    "down" => CoordType::BodyDown,
                    "left" => CoordType::BodyLeft,
                    "right" => CoordType::BodyRight,
                    _ => {
                        panic!("invalid direction, coordinates are possibly equal");
                    }
                };

                previous = *body_part;
            }
        }

        for hazard in &self.board.hazards {
            coord_types[pack_coord(hazard, self.board.width) as usize] = CoordType::Hazard;
        }

        println!("End of turn {}", self.turn);

        for y in 0..self.board.height {
            let mut print_line = String::new();

            for x in 0..self.board.width {
                let Some(coord_type) = coord_types.get(pack_xy(x, y, self.board.width) as usize)
                else {
                    panic!("Out of bounds search");
                };

                let graphic = match coord_type {
                    CoordType::Empty => graphics::EMPTY.white(),
                    CoordType::Food => graphics::FOOD.red(),
                    CoordType::Hazard => graphics::HAZARD.white(),
                    CoordType::BodyUp => graphics::BODY_UP.white(),
                    CoordType::BodyDown => graphics::BODY_DOWN.white(),
                    CoordType::BodyLeft => graphics::BODY_LEFT.white(),
                    CoordType::BodyRight => graphics::BODY_RIGHT.white(),
                    CoordType::Head => graphics::HEAD.white(),
                };

                print_line += format!(" {} ", graphic).as_str();
            }

            println!("{}", print_line);
        }
    }
}
