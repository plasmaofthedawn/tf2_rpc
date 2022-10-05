mod server;
mod presence;
mod log;

extern crate discord_rpc_client;

use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {

    let h = log::new_lines();
    println!("{}, {}", h.len(), h.get(0).unwrap());

    return;

    //assert!(is_tf2_running(), "tf2 is not running");

    let client = server::make_client();
    let mut drpc = presence::make_drpc();

    let info = server::get_server_info(&client, "5.188.225.147:27015");
    presence::set_activity_playing(&mut drpc, &info);

    println!("{} on {}, {}/{} players.", info.name, info.map, info.players, info.max_players);

    let info = server::get_server_info(&client, "169.254.91.154:61400");
    presence::set_activity_playing(&mut drpc, &info);

    thread::sleep(time::Duration::from_secs(100))
}

fn is_tf2_running() -> bool {

    let s = System::new_all();
    for process in s.processes_by_name("hl2_linux") {
        if process.exe().to_str().expect("Invalid tf2 path").contains("Team Fortress 2") {
            return true;
        }
    }

    false
}

