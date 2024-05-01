use bevy::prelude::*;

use crate::ecs::prelude::*;

pub struct ApplicationSystem;

impl ApplicationSystem {
    pub fn application_will_initialise() {}

    pub fn application_did_initialise(dispatcher: Res<DispatcherResource>) {
        dispatcher.dispatch(EcsEvents::Response(EcsResponse::ApplicationDidInitialise));
    }
}
