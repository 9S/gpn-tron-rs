use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::Split;
use log::{trace, warn};
use crate::util::GameData;

pub struct Connection<'a> {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    game_data: GameData,
    mappers: &'a HashMap<String, Box<dyn ReceiveCommandable>>
}

pub trait ReceiveCommandable {
    fn execute(&self, args: Split<char>, game_data: &mut GameData) -> Vec<Box<dyn SendCommandable>>;
}

pub trait SendCommandable {
    fn send(&self) -> Vec<String>;
}

impl Connection<'_> {
    pub fn new(game_data: GameData, mappers: &HashMap<String, Box<dyn ReceiveCommandable>>) -> Result<Connection, Box<dyn Error>> {
        let stream = TcpStream::connect("gpn-tron.duckdns.org:4000")?;
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Connection {
            stream,
            reader,
            game_data,
            mappers
        })
    }

    pub fn process_incoming_command(&mut self) -> Result<(), Box<dyn Error>> {
        let mut line: String = String::new();
        let size = self.reader.read_line(&mut line)?;

        if size == 0 {
            //warn!("Received an empty message");
            return Ok(());
        }

        let mut splitted = line
            .strip_suffix("\n")
            .unwrap()
            .split('|');

        let code = splitted.next().unwrap();
        let commandable = self.mappers.get(code);
        match commandable {
            Some(commandable) => {
                if code != "pos" && code != "message" {
                    trace!("Executing command {} with Commandable", code);
                }
                let send_commands = commandable.execute(splitted, &mut self.game_data);
                for send_command in send_commands {
                    let data = send_command.send();
                    let mut answer_command_str = data.join("|");
                    trace!("Sending Answer: {}", answer_command_str);
                    answer_command_str += "\n";
                    self.stream.write_all(answer_command_str.as_bytes()).expect("Data writing error");
                    self.stream.flush().expect("Data flushing error");
                }
            },
            None => {
                warn!("Command for type {} not found", code);
            }
        }
        Ok(())
    }
}
