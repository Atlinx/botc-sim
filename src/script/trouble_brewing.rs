use std::any::Any;

use crate::sim::*;
use crate::*;

#[derive(Debug)]
pub struct Washerwoman {}

#[derive(Debug, Clone)]
pub struct WasherwomanData {
    know_role: String,
    know_players: [String; 2],
}
impl RoleData for WasherwomanData {}

impl Role for Washerwoman {
    fn role_name(&self) -> &'static str {
        "washerwoman"
    }

    fn run_first_night(&self, world: &mut World) {
        world.process_player_event::<WasherwomanData>();
    }
}

pub struct Script;

impl script::Script for Script {
    fn script_name(&self) -> &'static str {
        "trouble_brewing"
    }

    fn parse_world_mut_sim(
        &self,
        sim: &mut sim::SimScript,
        world: &parser::World,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sim.first_night_order = vec![];
        sim.night_order = vec![];
        sim.day_order = vec![];
        for step in &world.steps {}
        Ok(())
    }
}
