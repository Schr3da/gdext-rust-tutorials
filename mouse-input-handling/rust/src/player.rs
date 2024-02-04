use godot::engine::{ISprite2D, InputEvent, InputEventMouse, Sprite2D};
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

    fn input(&mut self, event: Gd<InputEvent>) {
        // mouse events
        match event.try_cast::<InputEventMouse>() {
            Ok(e) => {
                let mouse_position = e.get_position();
                self.base_mut().set_position(mouse_position);
                return;
            }
            Err(_) => {}
        };
    }
}
