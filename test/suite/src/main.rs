mod agent;
mod operator;
mod process_wrapper;
mod scenario;
mod server;
mod session;

use clap::{Parser, Subcommand};
use scenario::{Barrel, Item, Sign, Vec3};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a sandbox server for manual testing
    Sandbox {
        #[clap(long)]
        run_sss: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sandbox { run_sss } => {
            let mut server = server::Server::start();
            let mut session = session::Session::start(&mut server);

            if *run_sss {
                session.start_operator();
                session.start_agent();
            }

            session.load_scenario(scenario::Scenario {
                barrels: vec![Barrel {
                    location: Vec3 { x: 5, y: 1, z: 5 },
                    items: vec![Item {
                        name: String::from("stone"),
                        count: 5,
                        slot: 1,
                    }],
                }],
                signs: vec![Sign {
                    location: Vec3 { x: 5, y: 2, z: 5 },
                    text: [
                        String::from("among us"),
                        String::from(""),
                        String::from(""),
                        String::from(""),
                    ],
                }],
            });

            println!("Sandbox server started at 0.0.0.0:25585");

            loop {
                std::thread::park();
            }
        }
    };
}
