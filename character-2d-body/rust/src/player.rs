use godot::classes::{CharacterBody2D, ICharacterBody2D, InputEvent};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            keys: vec![Key::LEFT, Key::RIGHT, Key::UP],
            key_states: HashMap::new(),
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut velocity = self.base_mut().get_velocity();
        velocity = self.apply_gravity(velocity);
        velocity = self.apply_controls(velocity);

        let mut base = self.base_mut();
        if let Some(c) = base.move_and_collide(velocity) {
            velocity = velocity.slide(c.get_normal());
        }

        base.move_and_collide(velocity);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        self.keys.iter().for_each(|k| {
            let is_key_pressed = Input::singleton().is_key_pressed(*k);
            self.key_states.insert(*k, is_key_pressed);
        });
    }
}

impl Player {
    fn apply_gravity(&mut self, mut velocity: Vector2) -> Vector2 {
        velocity.y += 8.0;
        velocity
    }

    fn apply_controls(&mut self, mut velocity: Vector2) -> Vector2 {
        let jump_speed = 20.0;
        let move_speed = 10.0;

        self.keys.iter().for_each(|k| {
            match self.key_states.get(&k) {
                Some(true) => {}
                _ => return,
            };

            match *k {
                Key::UP => velocity.y -= jump_speed,
                Key::LEFT => velocity.x -= move_speed,
                Key::RIGHT => velocity.x += move_speed,
                _ => {
                    godot_print!("key not supported {:?}", k);
                }
            };
        });

        velocity
    }
}
