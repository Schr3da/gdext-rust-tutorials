use bevy::prelude::*;

use crate::ecs::prelude::*;

#[derive(Resource)]
pub struct DispatcherResource {
    inner: EcsSender,
}

impl DispatcherResource {

    pub fn new(ecs_sender: EcsSender) -> Self {
        Self {
            inner: ecs_sender,
        }
    }

    pub fn dispatch(&self, event: EcsEvents) {
        _ = self.inner.send(event);
    }
}
