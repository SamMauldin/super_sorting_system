use std::env;
use std::process::Command;
use std::path::Path;

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
        let server_path = suite_path.parent().unwrap().join("server");
        let spigot_path = server_path.join("spigot-1.18.2.jar");

        println!("Starting Spigot server...");

        let mut command = Command::new("java");

            command.current_dir(server_path)
            .args(["-Xmx1G", "-Xms1G", "-jar"])
            .arg(spigot_path);

        let process_wrapper = ProcessWrapper::new(command, "spigot");
        let server = Server { process_wrapper };

        let done_regex = Regex::new(r"\[.+\] \[Server thread/INFO\]: Done \(.+\)!").unwrap();
        server.wait_for_regex(&done_regex);

        println!("Spigot started!");

        server
    }

    pub fn wait_for_regex(&self, regex: &Regex) {
        self.process_wrapper.wait_for_regex(regex);
    }

    pub fn stop(mut self) {
        println!("Stopping Spigot...");

        self.process_wrapper.send_stdin("stop\n");
        self.process_wrapper.wait_for_stop();
    }
}
