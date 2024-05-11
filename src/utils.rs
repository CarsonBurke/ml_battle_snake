use crate::Coord;

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