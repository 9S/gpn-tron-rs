use std::collections::VecDeque;
use crate::util::{Direction, GameData, PLAYER_ID_NONE, Vector2};

pub fn flood_fill(game_state: &GameData, start_pos: Vector2) -> (u32, u8) {
    let mut queue: VecDeque<Vector2> = VecDeque::new();
    queue.push_back(start_pos);

    let mut num_fields = 0u32;
    let mut player_count = 0u8;
    let mut already_checked: Vec<Vector2> = Vec::new();

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        if game_state.grid[n.x as usize][n.y as usize] == PLAYER_ID_NONE {
            num_fields += 1;
            Direction::iterator().for_each(|dir| {
                let new_pos = dir.get_relative_width_height(n, game_state.grid_meta.width, game_state.grid_meta.height);
                if !already_checked.contains(&new_pos) {
                    queue.push_back(new_pos);
                    already_checked.push(new_pos)
                }
            });
        } else {
            let position = game_state.player_heads.values().find(|&pos| *pos == n);
            if position.is_some() && game_state.player_heads[&game_state.own_id] != *position.unwrap() {
                player_count += 1;
            }
        }
    }
    (num_fields, player_count)
}