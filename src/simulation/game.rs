use std::collections::{HashMap, HashSet};

use rand::{random, Rng};

use crate::{
    logic::get_move,
    utils::{is_out_of_bounds, pack_coord, pack_xy},
    Battlesnake, Board, Coord, Game, GameState,
};

use super::constants::graphics;

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
                length: 2,
                shout: Some("".to_string()),
            });
        }

        let mut food = Vec::new();

        food.push(Coord { x: 0, y: 0 });

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

    pub async fn play_for_outcome(&mut self) -> GameStepOutcome {
        for _turn in self.turn.. {
            let step_outcome = self.turn_step();

            match step_outcome {
                GameStepOutcome::None => continue,
                _ => return step_outcome,
            }
        }

        GameStepOutcome::None
    }

    pub fn turn_step(&mut self) -> GameStepOutcome {
        let mut moves = Vec::new();
        let mut index = 0;

        for snake in &self.board.snakes {
            let chosen_move = get_move(&self.game, &self.turn, &self.board, &snake);

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

            snake.head.x += offset.x;
            snake.head.y += offset.y;

            for body_part in &mut snake.body {
                body_part.x += offset.x;
                body_part.y += offset.y;
            }
        }

        self.propagate_snakes();
        self.kill_snakes();

        #[cfg(visualize)]
        self.visualize();

        self.turn += 1;

        match self.board.snakes.len() {
            1 => GameStepOutcome::Winner(self.board.snakes[0].id.clone()),
            0 => GameStepOutcome::Tie,
            _ => GameStepOutcome::None,
        }
    }

    fn propagate_snakes(&mut self) {}

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

        // Check for head-on collisions and kill snakes when necessary

        let mut snake_head_coords = HashSet::new(); // HashSet::from_iter(self.board.snakes.iter().map(|snake| snake.head));

        for snake in &self.board.snakes {
            snake_head_coords.insert(snake.head);
        }

        // Kill both snakes that collided their heads
        self.board.snakes.retain(|snake| {
            if snake_head_coords.contains(&snake.head) {
                return false;
            }

            true
        });

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

    fn visualize(&self) {
        let mut coord_types = Vec::new();

        for x in 0..self.board.width {
            for y in 0..self.board.height {
                coord_types.push(CoordType::Empty);
            }
        }

        for food in &self.board.food {
            coord_types.insert(
                pack_coord(*food, self.board.width) as usize,
                CoordType::Food,
            );
        }

        for snake in &self.board.snakes {
            coord_types.insert(
                pack_coord(snake.head, self.board.width) as usize,
                CoordType::Food,
            );

            for body_part in &snake.body {
                coord_types.insert(
                    pack_coord(*body_part, self.board.width) as usize,
                    CoordType::Food,
                );
            }
        }

        for hazard in &self.board.hazards {
            coord_types.insert(
                pack_coord(*hazard, self.board.width) as usize,
                CoordType::Hazard,
            );
        }

        for x in 0..self.board.width {
            for y in 0..self.board.height {
                let Some(coord_type) = coord_types.get(pack_xy(x, y, self.board.width) as usize)
                else {
                    continue;
                };

                let graphic = match coord_type {
                    CoordType::Empty => graphics::EMPTY,
                    CoordType::Food => graphics::FOOD,
                    CoordType::Hazard => graphics::HAZARD,
                    CoordType::BodyUp => graphics::BODY_UP,
                    CoordType::BodyDown => graphics::BODY_DOWN,
                    CoordType::BodyLeft => graphics::BODY_LEFT,
                    CoordType::BodyRight => graphics::BODY_RIGHT,
                    CoordType::Head => graphics::HEAD,
                };

                println!("{}", graphic);
            }
        }
    }
}
