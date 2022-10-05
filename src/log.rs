extern crate rev_lines;

use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;
use rev_lines::RevLines;

const LOG_LOCATION: &str = "/home/zacha/.local/share/Steam/steamapps/common/Team Fortress 2/tf/console.log";

static OLD_SIZE: u64 = 0;

pub fn new_lines() -> Vec<String> {


    let mut file = File::open(LOG_LOCATION).expect("Could not find logfile.");
    let newsize = file.metadata().unwrap().len();
    file.seek(SeekFrom::Start(OLD_SIZE)).expect("Could not seek log");
    let reader = BufReader::new(file);

    let rev_lines = RevLines::new(reader).expect("Unable to read last lines");

    Vec::from_iter(rev_lines)


}