mod godot_singleton;
mod player;
mod singleton;

use godot::classes::Engine;
use godot::prelude::*;
use godot_singleton::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        Engine::singleton().register_singleton("MyGodotSingleton", &MyGodotSingleton::new_alloc());
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        let mut engine = Engine::singleton();
        match Engine::singleton().get_singleton("MyGodotSingleton") {
            None => godot_error!("Failed to get singleton"),
            Some(s) => {
                engine.unregister_singleton("MyGodotSingleton");
                s.free();
            }
        }
    }
}
