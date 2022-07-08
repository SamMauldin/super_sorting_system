use std::env;
use std::path::Path;
use std::process::Command;

use regex::Regex;

use super::process_wrapper::ProcessWrapper;

pub const OPERATOR_API_KEY: &'static str = "547980bc-3194-4b08-be75-7046c5e9d21c";

pub struct Operator {
    process_wrapper: ProcessWrapper,
}

impl Operator {
    pub fn start() -> Self {
        let suite_directory = env::var("CARGO_MANIFEST_DIR")
            .expect("unable to determine workspace directory: must be ran with cargo");
        let suite_path = Path::new(&suite_directory);
        let operator_path = suite_path.parent().unwrap().join("operator");

        println!("Starting operator...");

        let mut command = Command::new("cargo");

        command
            .current_dir(operator_path)
            .arg("run")
            .env("RUST_LOG", "info")
            .env("RUST_BACKTRACE", "1")
            .env("OPERATOR_PORT", "6323")
            .env("OPERATOR_HOST", "0.0.0.0")
            .env("OPERATOR_API_KEYS", format!("[\"{}\"]", OPERATOR_API_KEY));

        let process_wrapper = ProcessWrapper::new(command, "operator");
        let server = Self { process_wrapper };

        let done_regex = Regex::new(r"Bound to ").unwrap();
        server.wait_for_regex(&done_regex);

        println!("Operator started!");

        server
    }

    pub fn wait_for_regex(&self, regex: &Regex) {
        self.process_wrapper.wait_for_regex_stderr(regex);
    }

    pub fn stop(self) {
        println!("Stopping operator...");
    }
}
