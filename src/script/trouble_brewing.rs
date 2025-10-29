use std::any::Any;

use crate::sim::*;
use crate::*;

#[derive(Debug)]
pub struct Washerwoman {}

#[derive(Debug)]
pub struct WasherwomanData {
    know_role: String,
    know_players: [String; 2],
}

impl Role for Washerwoman {
    fn role_name() -> &'static str {
        "washerwoman"
    }

    fn run_first_night(&self, world: &mut World) {
        for player in world.players.values_mut() {
            let player = player.borrow_mut();
            let role = player.role as Box<dyn Any>;
            player.role
        }
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
        Ok(())
    }
}
