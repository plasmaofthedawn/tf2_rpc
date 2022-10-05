extern crate rev_lines;
extern crate steamlocate;

use rev_lines::RevLines;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use steamlocate::SteamDir;

static OLD_SIZE: u64 = 0;

pub fn new_lines() -> Vec<String> {
    let mut steamdir = SteamDir::locate().unwrap();
    let tf2 = match steamdir.app(&440) {
	    Some(app) => app,
	    None => panic!("Couldn't locate Team Fortress 2 on this computer!"),
    };
    let mut file = File::open(&tf2.path.join("tf/console.log")).expect("Could not find logfile.");
    let newsize = file.metadata().unwrap().len();
    file.seek(SeekFrom::Start(OLD_SIZE))
        .expect("Could not seek log");
    let reader = BufReader::new(file);

    let rev_lines = RevLines::new(reader).expect("Unable to read last lines");

    Vec::from_iter(rev_lines)
}
