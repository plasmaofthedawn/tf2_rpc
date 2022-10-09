mod server;
mod presence;
mod log;

extern crate discord_rpc_client;

use log::Tf2State;
use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut old_state: Tf2State = Tf2State::Menu;
    
    thread::sleep(time::Duration::from_secs(10));

    while is_tf2_running() {
        thread::sleep(time::Duration::from_secs(5));

        let current_state = log::get_tf2_state().unwrap();

        if old_state.eq(&current_state) {
            println!("No change in state");
            continue
        } else {
            old_state = current_state;
        }

        match &old_state {
            Tf2State::Menu => {
                println!("Menu");
                presence::set_activity_menu();
            }
            Tf2State::Queue(queue_type) => {
                println!("Queue Casual");
                presence::set_activity_queue(queue_type);
            }
            Tf2State::Server(server) => {

                println!("Game on ip {}", &server.ip);
                match server::get_server_info(&server.ip) {
                    Ok(info) => {
                        presence::set_activity_playing_from_info(&info);
                    }
                    Err(e) =>  {
                        eprintln!("Unable to get information from server {}: {}", &server.ip, e);
                        presence::set_activity_playing(&server.map, "Valve casual server", 0, 0)
                    }
                };
            }
        }
    }

    eprintln!("Tf2 is not running -- exiting");

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
