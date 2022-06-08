use signal_child::Signalable;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{channel, Receiver};

use regex::Regex;

pub struct ProcessWrapper {
    child: Child,

    recv_stdout: Receiver<String>,
    recv_stderr: Receiver<String>,
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
        let (send_stderr, recv_stderr) = channel::<String>();

        std::thread::spawn(move || {
            for line in stdout.lines() {
                let line = line.unwrap();

                println!("[{}] {}", name, line);
                match send_stdout.send(line) {
                    Ok(_) => {}
                    Err(_) => continue,
                }
            }
        });

        std::thread::spawn(move || {
            for line in stderr.lines() {
                let line = line.unwrap();

                println!("[{} err] {}", name, line);
                match send_stderr.send(line) {
                    Ok(_) => {}
                    Err(_) => continue,
                }
            }
        });

        ProcessWrapper {
            child,
            recv_stdout,
            recv_stderr,
            stdin,
        }
    }

    pub fn wait_for_regex(&self, regex: &Regex) -> String {
        loop {
            let line = self.recv_stdout.recv().unwrap();

            if regex.is_match(&line) {
                return line;
            }
        }
    }

    pub fn wait_for_regex_stderr(&self, regex: &Regex) {
        loop {
            let line = self.recv_stderr.recv().unwrap();

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

impl Drop for ProcessWrapper {
    fn drop(&mut self) {
        let _ = self.child.term();
        self.child.wait().unwrap();
    }
}
