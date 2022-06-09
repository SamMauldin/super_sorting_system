use std::env;
use std::fs::{remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;

use regex::Regex;

use super::process_wrapper::ProcessWrapper;

pub struct Server {
    process_wrapper: ProcessWrapper,
}

impl Server {
    pub fn start() -> Server {
        let suite_directory = env::var("CARGO_MANIFEST_DIR")
            .expect("unable to determine workspace directory: must be ran with cargo");
        let suite_path = Path::new(&suite_directory);
        let server_path = suite_path.join("server");
        let playerdata_path = server_path.join("world/playerdata");
        let spigot_path = server_path.join("spigot-1.18.2.jar");

        println!("Starting Spigot server...");

        // Best effort to clean server state
        let _ = remove_dir_all(playerdata_path);
        let _ = remove_file(server_path.join("world/session.lock"));
        let _ = remove_file(server_path.join("world_nether/session.lock"));
        let _ = remove_file(server_path.join("world_the_end/session.lock"));

        let mut command = Command::new("java");

        command
            .current_dir(server_path)
            .args(["-Xmx1G", "-Xms1G", "-jar"])
            .arg(spigot_path);

        let process_wrapper = ProcessWrapper::new(command, "spigot");
        let server = Server { process_wrapper };

        let done_regex = Regex::new(r"\[.+\] \[Server thread/INFO\]: Done \(.+\)!").unwrap();
        server.wait_for_regex(&done_regex);

        println!("Spigot started!");

        server
    }

    pub fn run_command(&mut self, command: &str, expect_regex: &str) -> String {
        self.process_wrapper.send_stdin(&format!("{}\n", command));

        self.wait_for_regex(&Regex::new(expect_regex).unwrap())
    }

    pub fn wait_for_regex(&self, regex: &Regex) -> String {
        self.process_wrapper.wait_for_regex(regex)
    }

    pub fn stop(mut self) {
        println!("Stopping Spigot...");

        self.process_wrapper.send_stdin("stop\n");
        self.process_wrapper.wait_for_stop();
    }
}
