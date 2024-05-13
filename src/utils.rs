use rand::Rng;

use crate::{neural_network::{Input, NeuralNetwork, Output}, Coord};

pub fn pack_xy(x: i32, y: u32, width: i32) -> i32 {
    x * width + y as i32
}

pub fn pack_coord(coord: &Coord, width: i32) -> i32 {
    coord.x * width + coord.y
}

pub fn unpack_coord(packed_coord: i32, width: i32) -> Coord {

    Coord {
        x: packed_coord / width,
        y: packed_coord % width,
    }
}

pub fn bool_as_f32(boolean: bool) -> f32 {
    if boolean {
        return 1.0;
    };

    0.0
}

pub fn is_out_of_bounds(x: i32, y: i32, width: i32, height: u32) -> bool {
    x < 0 || x >= width || y < 0 || y >= height as i32
}

pub fn get_direction<'a>(front: Coord, back: Coord) -> &'a str {
    
    // vertical
    if front.x == back.x {
        if front.y < back.y {
            return "up"
        }

        return "down"
    }

    // horizontal
    if front.y == back.y {
        if front.x < back.x {
            return "left"
        }

        return "right"
    }

    "unknown"
}

pub fn random_coord(width: i32, height: u32) -> Coord {
    let mut rng = rand::thread_rng();

    Coord {
        x: rng.gen_range(0..width),
        y: rng.gen_range(0..height) as i32,
    }
}

pub fn build_neural_network(neural_network: &mut NeuralNetwork, width: i32, height: u32) {
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

    for x in 0..width {
        for y in 0..height {

            inputs.push(Input::new(
                "coord".to_string(),
                vec![
                    x as f32,
                    y as f32,
                    0.,
                    0.,
                    0.,
                    0.,
                    0.,
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
    }

    // Outputs

    let outputs: Vec<Output> = vec![
        Output::new("up".to_string()),
        Output::new("down".to_string()),
        Output::new("left".to_string()),
        Output::new("right".to_string()),
    ];

    neural_network.build(&inputs, outputs.len());
}