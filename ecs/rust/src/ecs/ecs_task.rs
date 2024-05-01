use std::sync::mpsc::*;
use std::thread;
use std::thread::JoinHandle;

use bevy::prelude::*;

use super::prelude::*;

#[derive(Debug)]
pub enum EcsRequest {
    ApplicationWillInitialise,
}

#[derive(Debug)]
pub enum EcsResponse {
    ApplicationDidInitialise,
}

#[derive(Debug)]
pub enum EcsEvents {
    Request(EcsRequest),
    Response(EcsResponse),
}

pub type EcsSender = Sender<EcsEvents>;

pub struct Ecs {
    world: World,
    application_scheduler: ApplicationScheduler,
    node_sender: EcsSender,
}

impl Ecs {
    pub fn launch(node_sender: EcsSender) -> (JoinHandle<()>, EcsSender) {

        let (ecs_sender, ecs_receiver) = channel();

        let scoped_ecs_sender = ecs_sender.clone();
        let task = thread::spawn(move || {
            let mut ecs = Ecs::new(scoped_ecs_sender, node_sender);

            loop {
                let next = match ecs_receiver.try_recv() {
                    Err(_) => continue,
                    Ok(e) => e,
                };


                match next {
                    EcsEvents::Request(req) => ecs.handle_request(req),
                    EcsEvents::Response(res) => ecs.handle_response(res),
                };
            }
        });

        return (task, ecs_sender);
    }

    fn new(ecs_sender: EcsSender, node_sender: EcsSender) -> Self {
        let mut world = World::new();

        world.insert_resource(DispatcherResource::new(ecs_sender.clone()));

        let application_scheduler = ApplicationScheduler::new();

        Self {
            world,
            application_scheduler,
            node_sender,
        }
    }

    fn handle_request(&mut self, req: EcsRequest) {
        match req {
            EcsRequest::ApplicationWillInitialise => {
                self.application_scheduler.run(&mut self.world);
            }
        };
    }

    fn handle_response(&mut self, res: EcsResponse) {
        match res {
            EcsResponse::ApplicationDidInitialise => {
                _ = self.node_sender.send(EcsEvents::Response(res))
            }
        };
    }
}
