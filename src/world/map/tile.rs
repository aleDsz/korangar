use procedural::ByteConvertable;

use crate::loaders::{ByteConvertable, ByteStream};

const NONE: u8 = 0b00000000;
const WALKABLE: u8 = 0b00000001;
const WATER: u8 = 0b00000010;
const SNIPABLE: u8 = 0b00000100;
const CLIFF: u8 = 0b00001000;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TileType(pub u8);

impl ByteConvertable for TileType {
    fn from_bytes(byte_stream: &mut ByteStream, _: Option<usize>) -> Self {
        Self::new(byte_stream.next())
    }
}

impl TileType {
    pub fn new(type_index: u8) -> Self {
        match type_index {
            0 => Self(WALKABLE),
            1 => Self(NONE),
            2 => Self(WATER),
            3 => Self(WATER | WALKABLE),
            4 => Self(WATER | SNIPABLE),
            5 => Self(CLIFF | SNIPABLE),
            6 => Self(CLIFF),
            invalid => panic!("invalid tile type {invalid}"),
        }
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub fn is_walkable(&self) -> bool {
        self.0 & WALKABLE != 0
    }
}

#[allow(dead_code)]
#[derive(ByteConvertable, Debug)]
pub struct Tile {
    pub upper_left_height: f32,
    pub upper_right_height: f32,
    pub lower_left_height: f32,
    pub lower_right_height: f32,
    pub tile_type: TileType,
    _skip: [u8; 3],
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        self.tile_type.is_walkable()
    }

    pub fn average_height(&self) -> f32 {
        (self.upper_left_height + self.upper_right_height + self.lower_left_height + self.lower_right_height) / 4.0
    }
}
