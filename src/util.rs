use std::collections::HashMap;
use grid::Grid;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub type PlayerId = u8;
pub type Coordinate = u8;

pub const PLAYER_ID_NONE: PlayerId = 255;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector2 {
    pub(crate) x: Coordinate,
    pub(crate) y: Coordinate
}

pub struct GridMeta {
    pub(crate) width: Coordinate,
    pub(crate) height: Coordinate
}

impl GridMeta {
    pub(crate) fn new(width: Coordinate, height: Coordinate) -> GridMeta {
        GridMeta {
            width,
            height
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point<'a> {
    pub(crate) x: Coordinate,
    pub(crate) y: Coordinate,
    pub(crate) grid_meta: &'a GridMeta
}

pub struct GameData {
    pub(crate) grid_meta: GridMeta,
    pub(crate) grid: Grid<PlayerId>,
    pub(crate) player_heads: HashMap<PlayerId, Vector2>,
    pub(crate) own_id: PlayerId,
    pub(crate) current_tick: u64
}

impl GameData {
    pub fn update_player_pos(&mut self, player_id: PlayerId, x: Coordinate, y: Coordinate) {
        self.grid[x as usize][y as usize] = player_id;
        self.player_heads.insert(player_id, Vector2{x, y});
    }
}

#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        vec![Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT].into_iter()
    }

    pub fn iterator_random() -> impl Iterator<Item = Direction> {
        let mut list = vec![Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT];
        list.shuffle(&mut thread_rng());
        list.into_iter()
    }
}

impl Direction {
    pub fn get_relative_width_height(&self, position: Vector2, width: Coordinate, height: Coordinate) -> Vector2 {
        let mut new_pos = position.clone();
        match self {
            Direction::UP => new_pos.y = (new_pos.y + height - 1) % height,
            Direction::DOWN => new_pos.y = (new_pos.y + 1) % height,
            Direction::LEFT => new_pos.x = (new_pos.x + height - 1) % width,
            Direction::RIGHT => new_pos.x = (new_pos.x + 1) % width
        };
        return new_pos;
    }

    pub fn get_relative<'a>(&self, pos: Point<'a>) -> Point<'a> {
        let mut point = pos.clone();
        match self {
            Direction::UP => point.y = (point.y - 1) % point.grid_meta.height,
            Direction::DOWN => point.y = (point.y + 1) % point.grid_meta.height,
            Direction::LEFT => point.x = (point.x - 1) % point.grid_meta.width,
            Direction::RIGHT => point.x = (point.x + 1) % point.grid_meta.width
        };
        return point;
    }
}
