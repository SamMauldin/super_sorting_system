use crate::{config::Config, state::State};

pub trait Service {
    fn new(config: &Config) -> Self;
    fn tick(&mut self, state: &mut State);
}
