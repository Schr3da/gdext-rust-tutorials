use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self { base }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (3.14 * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
