use godot::engine::INode2D;
use godot::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread::JoinHandle;

use crate::ecs::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EcsNode {
    base: Base<Node2D>,
    _ecs_task: JoinHandle<()>,
    ecs_sender: EcsSender,
    node_receiver: Receiver<EcsEvents>,
}

#[godot_api]
impl INode2D for EcsNode {
    fn init(base: Base<Node2D>) -> Self {
        let (node_sender, node_receiver) = channel::<EcsEvents>();

        let (_ecs_task, ecs_sender) = Ecs::launch(node_sender.clone());

        Self {
            base,
            _ecs_task,
            ecs_sender,
            node_receiver,
        }
    }

    fn ready(&mut self) {
        self.send_ecs_request(EcsRequest::ApplicationWillInitialise);
    }

    fn process(&mut self, _delta: f64) {

        let event = match self.node_receiver.try_recv() {
            Err(_) => return,
            Ok(e) => e,
        };

        godot_error!("{:?}", event);

        match event {
            EcsEvents::Request(_) => {}
            EcsEvents::Response(_) => {}
        };
    }
}

#[godot_api]
impl EcsNode {
    pub fn send_ecs_request(&self, req: EcsRequest) {
        _ = self.ecs_sender.send(EcsEvents::Request(req));
    }
}
