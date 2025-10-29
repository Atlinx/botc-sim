use crate::{
    parser,
    sim::{self, SimScript},
};

pub mod bad_moon_rising;
pub mod sects_and_violets;
pub mod travellers_and_fabled;
pub mod trouble_brewing;

pub trait Script {
    fn script_name(&self) -> &'static str;
    fn parse_world_to_sim(
        &self,
        world: &parser::World,
    ) -> Result<sim::SimScript, Box<dyn std::error::Error>> {
        let mut sim = sim::SimScript::default();
        sim.script = world.script.clone();
        self.parse_world_mut_sim(&mut sim, world)?;
        Ok(sim)
    }
    fn parse_world_mut_sim(
        &self,
        sim: &mut sim::SimScript,
        world: &parser::World,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

macro_rules! scripts_vec {
    ($($script:expr),*) => {
        {
            let mut script_vec: Vec<Box<dyn Script>> = Vec::new();
            $(
                script_vec.push(Box::new($script));
            )*
            script_vec
        }
    };
}

pub fn get_scripts() -> Vec<Box<dyn Script>> {
    scripts_vec!(
        bad_moon_rising::Script,
        sects_and_violets::Script,
        travellers_and_fabled::Script,
        trouble_brewing::Script
    )
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("error generating script: {0}")]
    ScriptError(#[from] Box<dyn std::error::Error>),
    #[error("invalid script: {0}")]
    InvalidScript(String),
}

pub fn parse_world_to_sim(world: &parser::World) -> Result<sim::SimScript, ParseError> {
    let scripts = get_scripts();
    let script = scripts
        .iter()
        .find(|x| x.script_name() == world.script)
        .ok_or(ParseError::InvalidScript(world.script.clone()))?;

    let sim = script.parse_world_to_sim(&world)?;
    Ok(sim)
}
