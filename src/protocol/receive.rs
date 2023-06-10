use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::Split;
use std::time::Instant;

use grid::Grid;
use log::{info, log, trace, warn};
use crate::algo::flood::flood_fill;

use crate::protocol::connection::{ReceiveCommandable, SendCommandable};
use crate::protocol::send::{Join, Move};
use crate::util::{Direction, GameData, GridMeta, PLAYER_ID_NONE, PlayerId};

pub struct Motd {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl ReceiveCommandable for Motd {
    fn execute(&self, mut args: Split<char>, _game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        println!("MOTD: {}", args.next().unwrap());
        vec![Box::new(Join::new(self.username.clone(), self.password.clone()))]
    }
}

pub struct ErrorCmd {}

impl ReceiveCommandable for ErrorCmd {
    fn execute(&self, mut args: Split<char>, _game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        println!("ERROR: {}", args.next().unwrap());
        vec![]
    }
}

pub struct LoseCmd {}

impl ReceiveCommandable for LoseCmd {
    fn execute(&self, mut args: Split<char>, _game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        info!("LOSE.");
        vec![]
    }
}

pub struct GameCmd {}

impl ReceiveCommandable for GameCmd {
    fn execute(&self, mut args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        let width = args.next().unwrap().parse::<u8>().unwrap();
        let height = args.next().unwrap().parse::<u8>().unwrap();
        let own_id = args.next().unwrap().parse::<PlayerId>().unwrap();
        game_data.grid_meta = GridMeta::new(width, height);
        game_data.own_id = own_id;
        game_data.grid = Grid::init(width as usize, height as usize, PLAYER_ID_NONE);
        game_data.player_heads.clear();
        game_data.current_tick = 0;
        vec![]
    }
}

pub struct PosCmd {}

impl ReceiveCommandable for PosCmd {
    fn execute(&self, mut args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        let player_id_str = args.next().expect("Unexpected argument length!");
        let player_id = player_id_str.parse::<u8>().expect("Unexpected player id!");
        let pos_x = args.next()
            .expect("Unexpected argument length!")
            .parse::<u8>()
            .expect("Unexpected X position!");
        let pos_y = args.next()
            .expect("Unexpected argument length!")
            .parse::<u8>()
            .expect("Unexpected Y position!");

        game_data.update_player_pos(player_id, pos_x, pos_y);
        vec![]
    }
}

struct PlayerCmd {}

impl ReceiveCommandable for PlayerCmd {
    fn execute(&self, mut args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        let id = args.next().unwrap();
        let username = args.next().unwrap();

        info!("Registered player {} as ID {}", username, id);
        vec![]
    }
}

struct Tick {}

impl ReceiveCommandable for Tick {
    fn execute(&self, _args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        let start = Instant::now();

        game_data.current_tick += 1;
        let my_position = game_data.player_heads[&game_data.own_id];
        let mut max_fields = 0u32;
        let mut max_direction = Direction::UP;
        let mut max_players = 0u8;

        for direction in Direction::iterator() {
            let pos = direction.get_relative_width_height(my_position, game_data.grid_meta.width, game_data.grid_meta.height);
            let (field_amount, player_amount) = flood_fill(game_data, pos);
            trace!("Direction {:?} has {} fields and {} players", direction, field_amount, player_amount);

            if field_amount > max_fields {
                max_direction = direction;
                max_fields = field_amount;
                max_players = player_amount;
            }
            if max_fields > 1000 {
                break;
            }
        }

        if max_fields == 0 {
            warn!("OUCHIES!!!")
        }

        let duration = start.elapsed();
        info!("Tick took {}ms", duration.as_millis());
        vec![Box::new(Move {
            direction: max_direction
        })]
    }
}

pub struct Die {
}

impl ReceiveCommandable for Die {
    fn execute(&self, args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>> {
        args.map(|arg| {arg.parse::<u8>().unwrap()})
            .for_each(|player_id| {
                game_data.player_heads.remove(&player_id);
                game_data.grid.iter_mut().for_each(|cell| {
                    if *cell == player_id {
                        *cell = PLAYER_ID_NONE;
                    }
                });
            });
        vec![]
    }
}

pub fn init_game_data() -> GameData {
    let game_data: GameData = GameData {
        grid_meta: GridMeta::new(0, 0),
        own_id: 0,
        grid: Grid::init(0, 0, PLAYER_ID_NONE),
        player_heads: HashMap::new(),
        current_tick: 0,
    };
    game_data
}

pub fn get_mappers(username: String, password: String) -> HashMap<String, Box<dyn ReceiveCommandable>> {
    let mut map: HashMap<String, Box<dyn ReceiveCommandable>> = HashMap::new();
    map.insert("motd".to_string(), Box::new(Motd { username, password }));
    map.insert("error".to_string(), Box::new(ErrorCmd {}));
    map.insert("lose".to_string(), Box::new(LoseCmd {}));
    map.insert("pos".to_string(), Box::new(PosCmd {}));
    map.insert("player".to_string(), Box::new(PlayerCmd {}));
    map.insert("game".to_string(), Box::new(GameCmd {}));
    map.insert("tick".to_string(), Box::new(Tick {}));
    map.insert("die".to_string(), Box::new(Die {}));
    map
}
