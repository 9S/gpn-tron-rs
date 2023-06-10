use crate::protocol::connection::SendCommandable;
use crate::util::Direction;

pub struct Join {
    username: String,
    password: String,
}

impl Join {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
}

impl SendCommandable for Join {
    fn send(&self) -> Vec<String> {
        vec![
            "join".to_string(),
            self.username.to_string(),
            self.password.to_string(),
        ]
    }
}

pub struct Move {
    pub(crate) direction: Direction
}

impl SendCommandable for Move {
    fn send(&self) -> Vec<String> {
        let dir = match self.direction {
            Direction::UP => {"up"}
            Direction::DOWN => {"down"}
            Direction::LEFT => {"left"}
            Direction::RIGHT => {"right"}
        }.to_string();

        vec![
            "move".to_string(),
            dir
        ]
    }
}