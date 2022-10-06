mod server;
mod presence;
mod log;

extern crate discord_rpc_client;

use log::Tf2State;
use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {

    let mut current_state = Tf2State::Menu;

    while is_tf2_running() {
        assert!(is_tf2_running(), "Tf2 is not running, exiting.");

        let h = log::get_tf2_state();

        if h.is_none() {
            println!("No change in state")
        } else {
            current_state = h.unwrap();
        }

        match &current_state {
            Tf2State::Menu => {
                println!("Menu");
                if let Err(e) = presence::set_activity_menu() {
                    eprintln!("Unable to set menu activity: {}", e);
                }
            }
            Tf2State::Server(ip)=> {

                println!("Game on ip {}", &ip);
                match server::get_server_info(&ip.as_str()) {
                    Ok(info) => {
                        if let Err(e) = presence::set_activity_playing_from_info(&info) {
                            eprintln!("Unable to set playing activity: {}", e);
                        }
                    }
                    Err(e) =>  {
                        eprintln!("Unable to get information from server {}: {}", &ip, e);
                        if let Err(e2) = presence::set_activity_playing("Unknown", &ip.as_str(), 0, 0) {
                            eprintln!("Unable to set playing activity: {}", e2);
                        };
                    }
                };
            }
        }

        thread::sleep(time::Duration::from_secs(5))
    }

    println!("Tf2 is not running -- exiting");

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

