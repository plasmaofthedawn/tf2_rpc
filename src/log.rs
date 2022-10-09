extern crate rev_lines;
extern crate steamlocate;

use rev_lines::RevLines;
use std::fs::File;
use std::io::BufReader;
use lazy_static::lazy_static;
use steamlocate::SteamDir;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum Tf2State {
    Menu,
    Server(Server),
    Queue(QueueType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum QueueType {
    Casual,
    Competitive,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Server {
    pub ip: String,
    pub map: String,
}

lazy_static! {
    static ref CONFIG_PATH: PathBuf  = {
        let mut steamdir = SteamDir::locate().unwrap();
        let tf2 = match steamdir.app(&440) {
	        Some(app) => app,
	        None => panic!("Couldn't locate Team Fortress 2 on this computer!"),
        };
        tf2.path.join("tf/console.log")
    };
}

fn get_new_lines_from_config() -> RevLines<File> {

    // TODO: Don't let the iterator go too far
    let file = File::open(&*CONFIG_PATH).expect("Could not find logfile.");

    let reader = BufReader::new(file);

    let rev_lines = RevLines::new(reader).expect("Unable to read last lines");

    rev_lines.into_iter()
}

fn parse_config_lines<'a, I>(lines: I) -> Option<Tf2State>
where
    I: Iterator<Item = String>
{
    let mut server = Server {
        ip: String::new(),
        map: String::new(),
    };

    for line in lines {
        match line.as_str() {
            "CTFGCClientSystem::PostInitGC" | 
            _ if line.contains("PartyRemoveFromQueue successfully") ||
            line.starts_with("Disconnect") => {
                return Some(Tf2State::Menu)
            }
            _ if line.contains("Entering queue for match group 12v12 Casual Match") => {
                return Some(Tf2State::Queue(QueueType::Casual))
            }
            _ if line.contains("Entering queue for match group 12v12 Competitive Match") => {
                return Some(Tf2State::Queue(QueueType::Competitive))
            }
            _ if line.starts_with("Connected to ") => {
                server.ip = line[13..].parse().unwrap();
                return Some(Tf2State::Server(server));
            }
            _ if line.starts_with("Map: ") => {
                server.map = line[5..].parse().unwrap();
            }
            _ => continue,
        }
    }

    None

}

pub fn get_tf2_state() -> Option<Tf2State> {
    let iter = get_new_lines_from_config();
    parse_config_lines(iter)
}