use crate::*;

pub struct Script;

impl script::Script for Script {
    fn script_name(&self) -> &'static str {
        "bad_moon_rising"
    }

    fn parse_world_mut_sim(
        &self,
        sim: &mut sim::SimScript,
        world: &parser::World,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
