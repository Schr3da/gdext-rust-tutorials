use godot::classes::{Engine, ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::singleton::EXAMPLE_SINGLETON;
use crate::MyGodotSingleton;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        match EXAMPLE_SINGLETON.lock() {
            Err(e) => godot_error!("{:?}", e),
            Ok(mut s) => {
                s.add("EXAMPLE_SINGLETON: Not a registered Godot Singleton".to_string());
                s.print();
            }
        };

        match Engine::singleton().get_singleton("MyGodotSingleton") {
            None => godot_error!("Failed to get singleton"),
            Some(s) => s.try_cast::<MyGodotSingleton>().unwrap().bind().print(),
        };

        Self { base }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (3.14 * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
