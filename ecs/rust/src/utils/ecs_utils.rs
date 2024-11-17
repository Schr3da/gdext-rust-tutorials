use godot::{classes::Node, obj::*};

use crate::ecs::prelude::*;
use crate::nodes::prelude::*;

pub struct EcsUtils;

impl EcsUtils {
    pub fn subscribe_to_ecs_response(id: String, parent: Option<Gd<Node>>) -> bool {
        let parent = match parent {
            None => return false,
            Some(n) => n,
        };

        let root = match parent.get_tree() {
            None => return false,
            Some(t) => t.get_root(),
        };

        let mut ecs = match root.unwrap().try_get_node_as::<EcsNode>("EcsNode") {
            None => return false,
            Some(e) => e,
        };

        let next = match parent.try_get_node_as::<Node>(&id) {
            None => return false,
            Some(n) => n,
        };

        let cb = next.callable("handle_ecs_response");

        ecs.connect("on_received_ecs_response", &cb);

        ecs.bind_mut()
            .send_ecs_request(EcsRequest::AddNewEcsSubscriber);

        true
    }
}
