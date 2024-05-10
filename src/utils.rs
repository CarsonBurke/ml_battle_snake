use crate::Coord;

pub fn pack_xy(x: i32, y: i32, width: i32) -> i32 {
    x * width + y
}

pub fn pack_coord(coord: Coord, width: i32) -> i32 {
    coord.x * width + coord.y
}

pub fn unpack_coord(packed_coord: i32, width: i32) -> (i32, i32) {
    (packed_coord / width, packed_coord % width)
}

pub fn bool_as_f32(boolean: bool) -> f32 {
    if boolean {
        return 1.0;
    };

    0.0
}
