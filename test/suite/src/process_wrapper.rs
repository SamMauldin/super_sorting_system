use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio, ChildStdin};
use std::sync::mpsc::{channel, Receiver};

use regex::Regex;

pub struct ProcessWrapper {
    child: Child,

    recv_stdout: Receiver<String>,
    stdin: ChildStdin,
}

impl ProcessWrapper {
    pub fn new(mut command: Command, name: &'static str) -> ProcessWrapper {
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect(&format!("failed to execute command for {}", name));

        let stdout = child.stdout.take().unwrap();
        let stdout = BufReader::new(stdout);
        let stderr = child.stderr.take().unwrap();
        let stderr = BufReader::new(stderr);
        let stdin = child.stdin.take().unwrap();

        let (send_stdout, recv_stdout) = channel::<String>();

        std::thread::spawn(move || {
            for line in stdout.lines() {
                let line = line.unwrap();

                println!("[{}] {}", name, line);
                send_stdout.send(line).unwrap();
            }
        });

        std::thread::spawn(move || {
            for line in stderr.lines() {
                let line = line.unwrap();

                println!("[{} err] {}", name, line);
            }
        });

        ProcessWrapper { child, recv_stdout, stdin}
    }

    pub fn wait_for_regex(&self, regex: &Regex) {
        loop {
            let line = self.recv_stdout.recv().unwrap();

            if regex.is_match(&line) {
                return;
            }
        }
    }

    pub fn wait_for_stop(mut self) {
        self.child.wait().unwrap();
    }

    pub fn send_stdin(&mut self, data: &str) {
        self.stdin.write_all(data.as_bytes()).unwrap();
    }
}
