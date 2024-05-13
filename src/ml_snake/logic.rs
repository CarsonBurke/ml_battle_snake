// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::{collections::HashMap, time::SystemTime};

use crate::{
    neural_network::{Input, NeuralNetwork, NeuralNetworkManager, Output},
    utils::{bool_as_f32, build_neural_network, pack_coord},
    Battlesnake, Board, Game,
};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "MarvinTMB", // TODO: Your Battlesnake Username
        "color": "#926ee4", // TODO: Choose color
        "head": "safe", // TODO: Choose head
        "tail": "bolt", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

#[derive(Default, Clone)]
pub struct GameInfo {
    pub my_health: i32,
    pub opponent_healths: Vec<i32>,
    pub my_length: i32,
    pub opponent_lengths: Vec<i32>,
}

#[derive(Default, Clone, Copy)]
pub struct CoordInfo {
    pub x: i32,
    pub y: i32,
    pub food: bool,
    pub my_head: bool,
    pub my_body: bool,
    pub opponent_head: bool,
    pub opponent_body: bool,
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move<'a>(game: &Game, turn: &i32, board: &Board, me: &Battlesnake) -> &'a str {
    let mut neural_network_manager = NeuralNetworkManager::new();
    let mut neural_network = NeuralNetwork::new(&mut neural_network_manager);

    build_neural_network(&mut neural_network, board.width, board.height);
    neural_network.mutate();

    choose_move(game, turn, board, me, &mut neural_network)
}

pub fn choose_move<'a>(_game: &Game, _turn: &i32, board: &Board, me: &Battlesnake, neural_network: &mut NeuralNetwork) -> &'a str {
    #[cfg(feature = "benchmark")]
    let start = SystemTime::now();

    let mut game_info = GameInfo {
        my_health: me.health,
        my_length: me.length,
        opponent_healths: Vec::new(),
        opponent_lengths: Vec::new(),
    };

    let mut grid: Vec<CoordInfo> = Vec::new();

    // let board_width = &board.width;
    // let board_height = &board.height;

    for x in 0..board.width {
        for y in 0..board.height {
            grid.push(CoordInfo {
                x,
                y: y as i32,
                ..Default::default()
            });
        }
    }

    for any_snake in &board.snakes {
        game_info.opponent_healths.push(any_snake.health);
        game_info.opponent_lengths.push(any_snake.length);

        if let Some(coord_info) = grid.get_mut(pack_coord(&any_snake.head, board.width) as usize) {
            if any_snake.id == me.id {
                coord_info.my_head = true;
            } else {
                coord_info.opponent_head = true;
            }
        };

        for body_part in &any_snake.body {
            if let Some(coord_info) = grid.get_mut(pack_coord(body_part, board.width) as usize) {
                if any_snake.id == me.id {
                    coord_info.my_body = true;
                } else {
                    coord_info.opponent_body = true;
                }
            };
        }
    }

    for food_coord in &board.food {
        if let Some(coord_info) = grid.get_mut(pack_coord(&food_coord, board.width) as usize) {
            coord_info.food = true;
        };
    }

    // neural network

    let mut inputs: Vec<Input> = vec![Input::new(
        "game".to_string(),
        vec![0., 0., 0., 0.],
        vec![
            "g0".to_string(),
            "g1".to_string(),
            "g2".to_string(),
            "g3".to_string(),
        ],
    )];

    for coord_info in &grid {
        inputs.push(Input::new(
            "coord".to_string(),
            vec![
                coord_info.x as f32,
                coord_info.y as f32,
                bool_as_f32(coord_info.food),
                bool_as_f32(coord_info.my_head),
                bool_as_f32(coord_info.my_body),
                bool_as_f32(coord_info.opponent_head),
                bool_as_f32(coord_info.opponent_body),
            ],
            vec![
                "c0".to_string(),
                "c1".to_string(),
                "c2".to_string(),
                "c3".to_string(),
                "c4".to_string(),
                "c5".to_string(),
                "c6".to_string(),
            ],
        ))
    }

    let outputs: Vec<Output> = vec![
        Output::new("up".to_string()),
        Output::new("down".to_string()),
        Output::new("left".to_string()),
        Output::new("right".to_string()),
    ];

    neural_network.forward_propagate(&inputs);
    let outputs = neural_network.get_outputs();

    let move_options = vec![
        ("up", outputs[0]),
        ("down", outputs[1]),
        ("left", outputs[2]),
        ("right", outputs[3]),
    ];

    let mut chosen_move: Option<&str> = None;
    let mut best_score = -1.0;

    for (move_name, move_score) in move_options {

        if move_score <= best_score {
            continue;
        }

        chosen_move = Some(move_name);
        best_score = move_score;
    }

    let Some(chosen_move) = chosen_move else {
        return "up"
    };

    #[cfg(feature = "snake_logs")]
    println!("MOVE {} with score {}", chosen_move, best_score);

    //

    // let mut is_move_safe: HashMap<_, _> = vec![
    //     ("up", true),
    //     ("down", true),
    //     ("left", true),
    //     ("right", true),
    // ]
    // .into_iter()
    // .collect();

    // // We've included code to prevent your Battlesnake from moving backwards
    // let my_head = &me.body[0]; // Coordinates of your head
    // let my_neck = &me.body[1]; // Coordinates of your "neck"

    // if my_neck.x < my_head.x {
    //     // Neck is left of head, don't move left
    //     is_move_safe.insert("left", false);
    // } else if my_neck.x > my_head.x {
    //     // Neck is right of head, don't move right
    //     is_move_safe.insert("right", false);
    // } else if my_neck.y < my_head.y {
    //     // Neck is below head, don't move down
    //     is_move_safe.insert("down", false);
    // } else if my_neck.y > my_head.y {
    //     // Neck is above head, don't move up
    //     is_move_safe.insert("up", false);
    // }

    // // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // // let board_width = &board.width;
    // // let board_height = &board.height;

    // // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    // // let my_body = &you.body;

    // // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // // let opponents = &board.snakes;

    // // Are there any safe moves left?
    // let safe_moves = is_move_safe
    //     .into_iter()
    //     .filter(|&(_, v)| v)
    //     .map(|(k, _)| k)
    //     .collect::<Vec<_>>();

    // // Choose a random move from the safe ones
    // let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // // let food = &board.food;

    // info!("MOVE {}: {}", turn, chosen);

    #[cfg(feature = "benchmark")]
    let duration = SystemTime::now().duration_since(start).unwrap().as_millis();
    #[cfg(feature = "benchmark")]
    info!("took {}ms", duration);

    return chosen_move;
}