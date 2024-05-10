use std::collections::HashMap;

use crate::{logic::get_move, Battlesnake, Board, Coord, Game, GameState};

pub struct GameWrapper {
    pub turn: i32,
    pub game: Game,
    pub board: Board,
}

pub enum GameOutcome {
    
}

impl GameWrapper {
    pub fn new(width: i32, height: u32, snakes_count: u32) -> Self {
        let mut snakes = Vec::new();

        snakes.push(Battlesnake {
            id: "0".to_string(),
            name: "Snake".to_string(),
            health: 100,
            body: Vec::new(),
            head: Coord { x: 0, y: 0 },
            latency: "0".to_string(),
            length: 0,
            shout: Some("".to_string()),
        });

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

    pub async fn play_for_outcome(&mut self) {

       for _turn in self.turn.. {
        self.turn_step()
       }
    }

    pub fn turn_step(&mut self) {
        let mut moves = Vec::new();

        for snake in &self.board.snakes {
            let chosen_move = get_move(&self.game, &self.turn, &self.board, &snake);

            moves.push(chosen_move);
        }

        self.turn += 1;
    }
}
