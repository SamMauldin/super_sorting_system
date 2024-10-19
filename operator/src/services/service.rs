use crate::{config::Config, state::State};

pub trait Service {
    fn new(config: &Config) -> Self
    where
        Self: Sized;
    fn tick(&mut self, state: &mut State);
    fn get_name(&self) -> &'static str;
}
