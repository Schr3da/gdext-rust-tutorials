use godot::engine::global::Key;
use godot::engine::{IArea2D, InputEvent, Area2D};
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    base: Base<Area2D>,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            keys: vec![Key::LEFT, Key::RIGHT, Key::UP, Key::DOWN],
            key_states: HashMap::new(),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (3.14 * delta) as f32;
        self.base_mut().rotate(radians);
        let mut current_position = self.base_mut().get_position();

        self.keys.iter().for_each(|k| {
            match self.key_states.get(&k) {
                Some(true) => {}
                _ => return,
            };

            match *k {
                Key::UP => current_position.y -= 1.0 * 1000.0 * delta as f32,
                Key::DOWN => current_position.y += 1.0 * 1000.0 * delta as f32,
                Key::LEFT => current_position.x -= 1.0 * 1000.0 * delta as f32,
                Key::RIGHT => current_position.x += 1.0 * 1000.0 * delta as f32,
                _ => {
                    godot_print!("key not supported {:?}", k);
                }
            };
        });

        self.base_mut().set_position(current_position);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        self.keys.iter().for_each(|k| {
            let is_key_pressed = Input::singleton().is_key_pressed(*k);
            self.key_states.insert(*k, is_key_pressed);
        });
    }
}

#[godot_api]
impl Player {

    #[func]
    fn on_collision_started(&mut self, _item: Gd<Area2D>) {
        godot_print!("Collision started");
    }

    #[func]
    fn on_collision_end(&mut self, _item: Gd<Area2D>) {
        godot_print!("Collision ended");
    }


}
