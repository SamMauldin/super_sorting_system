use std::env;
use std::path::Path;
use std::process::Command;

use regex::Regex;

use super::operator::OPERATOR_API_KEY;
use super::process_wrapper::ProcessWrapper;

pub struct Agent {
    process_wrapper: ProcessWrapper,
}

impl Agent {
    pub fn start(username: &str) -> Self {
        let suite_directory = env::var("CARGO_MANIFEST_DIR")
            .expect("unable to determine workspace directory: must be ran with cargo");
        let suite_path = Path::new(&suite_directory);
        let agent_path = suite_path.parent().unwrap().parent().unwrap().join("agent");

        println!("Starting agent...");

        let mut command = Command::new("npm");

        command
            .current_dir(agent_path)
            .arg("start")
            .env("AGENT_MC_SERVER_HOST", "127.0.0.1")
            .env("AGENT_MC_SERVER_PORT", "25585")
            .env("AGENT_USERNAME", username)
            .env("AGENT_PASSWORD", "")
            .env("AGENT_ENDPOINT", "http://127.0.0.1:6323")
            .env("AGENT_API_KEY", OPERATOR_API_KEY);

        let process_wrapper = ProcessWrapper::new(command, "agent");
        let server = Self { process_wrapper };

        let done_regex = Regex::new(r"Received spawn event").unwrap();
        server.wait_for_regex(&done_regex);

        println!("Agent started!");

        server
    }

    pub fn wait_for_regex(&self, regex: &Regex) {
        self.process_wrapper.wait_for_regex(regex);
    }

    pub fn stop(self) {
        println!("Stopping agent...");
    }
}
