use godot::classes::{CharacterBody2D, ICharacterBody2D, InputEvent};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;

use crate::nodes::prelude::*;
use crate::utils::prelude::*;

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

    fn ready(&mut self) {
        let id = self.base_mut().get_name();
        let parent = self.base_mut().get_parent();

        match EcsUtils::subscribe_to_ecs_response(id.to_string(), parent) {
            false => godot_error!("Unable to subscribe node to ecs responses"),
            true => godot_error!("Did subscribe node to ecs responses"),
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut velocity = self.base_mut().get_velocity();
        velocity = self.apply_gravity(velocity);
        velocity = self.apply_controls(velocity);

        let mut base = self.base_mut();
        if let Some(c) = base.move_and_collide(velocity) {
            velocity = velocity.slide(c.get_normal());
            base.emit_signal("on_player_did_collide", &[]);
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

#[godot_api]
impl Player {
    #[signal]
    fn on_player_did_collide();

    #[func]
    pub fn handle_ecs_response(&mut self, response: GodotEcsResponse) {
        godot_error!("received response for player: {:?}", response);
    }

    fn apply_gravity(&mut self, mut velocity: Vector2) -> Vector2 {
        velocity.y += 1.0;
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
