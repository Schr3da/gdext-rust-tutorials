use bevy::prelude::*;

use crate::ecs::prelude::*;

#[derive(Clone, SystemSet, Eq, PartialEq, Debug, Hash)]
pub enum ApplicationSystemSet {
    Init,
    Compete,
}

pub struct ApplicationScheduler {
    inner: Schedule,
}

impl ApplicationScheduler {
    pub fn new() -> Self {
        let mut inner = Schedule::default();

        inner.add_systems((
            ApplicationSystem::application_will_initialise.in_set(ApplicationSystemSet::Init),
            ApplicationSystem::application_did_initialise
                .in_set(ApplicationSystemSet::Compete)
                .after(ApplicationSystemSet::Init),
        ));

        Self { inner }
    }

    pub fn run(&mut self, world: &mut World) {
        self.inner.run(world);
    }
}
