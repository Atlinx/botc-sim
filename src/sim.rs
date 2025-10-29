use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use downcast_rs::{Downcast, impl_downcast};

pub trait RoleData: Debug + Downcast {}
impl_downcast!(RoleData);

pub trait Role: Debug + Downcast {
    fn role_name(&self) -> &'static str;
    fn run_first_night(&self, world: &mut World) {
        self.run_night(world);
    }
    fn run_night(&self, world: &mut World) {
        let _ = world;
    }
    fn run_day(&self, world: &mut World) {
        let _ = world;
    }
}
impl_downcast!(Role);

pub trait Token: Debug + Downcast {}
impl_downcast!(Token);

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub role_data: Box<dyn RoleData>,
    pub tokens: Vec<Box<dyn Token>>,
}

#[derive(Debug)]
pub struct World {
    // Current step we're on
    pub step: u32,
    pub players: HashMap<String, RefCell<Player>>,
    // Current events to process
    //
    // Events of a given night include
    // - Events from that night
    // - Events from the next day
    pub events: Vec<Box<dyn Event>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            step: 0,
            players: HashMap::new(),
            events: Vec::new(),
        }
    }
}

impl World {
    pub fn new() -> World {
        Self::default()
    }
    pub fn iter_role_data_players<T: RoleData>(&self) -> impl Iterator<Item = &RefCell<Player>> {
        let res = self.players.values().filter(|x| {
            let player = x.borrow();
            player.role_data.is::<T>()
        });
        res
    }
    pub fn iter_role_data_players_mut<T: RoleData>(
        &mut self,
    ) -> impl Iterator<Item = &mut RefCell<Player>> {
        let res = self.players.values_mut().filter(|x| {
            let player = x.borrow();
            player.role_data.is::<T>()
        });
        res
    }
    pub fn process_player_event<T: RoleData + Clone>(&mut self) {
        for event in self.events.iter_mut() {
            if let Some(event) = event.downcast_mut::<PlayerEvent<T>>()
                && let Some(player) = self.players.get_mut(&event.player)
            {
                player.borrow_mut().role_data = Box::new(event.data.clone());
            }
        }
    }
}

pub trait Event: Debug + Downcast {}
impl_downcast!(Event);

#[derive(Debug)]
pub struct PlayerEvent<T>
where
    T: Debug + 'static,
{
    pub player: String,
    pub data: T,
}
impl<T: Debug> Event for PlayerEvent<T> {}

#[derive(Debug)]
pub struct Step {
    step: u32,
    events: Vec<Box<dyn Event>>,
}

#[derive(Debug)]
pub struct SimScript {
    pub first_night_order: Vec<Box<dyn Role>>,
    pub night_order: Vec<Box<dyn Role>>,
    pub day_order: Vec<Box<dyn Role>>,
    pub night_limit: u32,
    pub world: World,
    pub steps: VecDeque<Step>,
    pub script: String,
}

impl Default for SimScript {
    fn default() -> Self {
        Self {
            first_night_order: Vec::new(),
            night_order: Vec::new(),
            day_order: Vec::new(),
            night_limit: 0,
            world: World::new(),
            steps: VecDeque::new(),
            script: String::new(),
        }
    }
}

impl SimScript {
    pub fn new(script: String, night_limit: u32) -> Self {
        Self {
            script,
            night_limit,
            ..Self::default()
        }
    }

    /// Runs the simulation to completion
    pub fn run_sim(&mut self) {
        for _ in 0..self.night_limit {
            self.step_sim();
        }
    }

    /// Steps the simulation once
    pub fn step_sim(&mut self) {
        self.world.step += 1;
        if let Some(next_step) = self.steps.get(0)
            && next_step.step == self.world.step
        {
            // Fetch next step
            let next_step = self.steps.pop_front().unwrap();
            self.world.events = next_step.events;

            // Step night
            if self.world.step == 1 {
                for role in self.first_night_order.iter_mut() {
                    role.run_first_night(&mut self.world);
                }
            } else {
                for role in self.night_order.iter_mut() {
                    role.run_night(&mut self.world);
                }
            }

            // Step day (roles)
            for role in self.day_order.iter_mut() {
                role.run_day(&mut self.world);
            }

            // Step voting
        }
    }
}
