extern crate rev_lines;
extern crate steamlocate;

use rev_lines::RevLines;
use std::fs::File;
use std::io::BufReader;
use lazy_static::lazy_static;
use steamlocate::SteamDir;
use std::path::PathBuf;

pub enum Tf2State {
    Menu,
    Server(String),
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
    for line in lines {
        if line == "CTFGCClientSystem::PostInitGC" {
            return Some(Tf2State::Menu);
        } else if line.starts_with("Disconnect") {
            return Some(Tf2State::Menu);
        } else if line.starts_with("Connected to ") {
            return Some(Tf2State::Server(line[13..].parse().unwrap()));
        }
    }

    None

}

pub fn get_tf2_state() -> Option<Tf2State> {
    let iter = get_new_lines_from_config();
    parse_config_lines(iter)
}