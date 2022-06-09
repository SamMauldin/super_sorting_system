use super::agent::Agent;
use super::operator::Operator;
use super::scenario::Scenario;
use super::server::Server;

pub struct Session<'a> {
    pub server: &'a mut Server,
    operator: Option<Operator>,
    agents: Vec<Agent>,
}

impl<'a> Session<'a> {
    pub fn start(server: &'a mut Server) -> Self {
        let mut session = Self {
            server,
            operator: None,
            agents: Vec::new(),
        };

        session.reset_area();

        session
    }

    pub fn reset_area(&mut self) {
        let server = &mut self.server;
        server.run_command("/world world", r"Set the world override to world\.");

        server.run_command("/pos1 0,0,0", "First position set");
        server.run_command("/pos2 15,0,15", "Second position set");
        server.run_command("/set stone", "Operation completed");

        server.run_command("/pos1 0,1,0", "First position set");
        server.run_command("/pos2 15,15,15", "Second position set");
        server.run_command("/set air", "Operation completed");

        server.run_command("setworldspawn 1 1 1", "Set the world spawn point");

        server.run_command(
            "kill @e[type=item]",
            r"(Killed \d+ entities)|(No entity was found)",
        );
        self.server
            .run_command("gamerule spawnRadius 0", r"spawnRadius is now set to");
    }

    pub fn start_operator(&mut self) {
        if self.operator.is_some() {
            panic!("Operator was already started!");
        }

        self.operator = Some(Operator::start());
    }

    pub fn start_agent(&mut self) {
        self.agents
            .push(Agent::start(&format!("sort_{}", self.agents.len())));
    }

    pub fn stop_agents(&mut self) {
        self.agents.clear();
    }

    pub fn stop_operator(&mut self) {
        self.operator.take();
    }

    pub fn load_scenario(&mut self, scenario: Scenario) {
        let command = scenario.to_command();

        command.map(|command| self.server.run_command(&command, "."));
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        self.stop_operator();
    }
}
