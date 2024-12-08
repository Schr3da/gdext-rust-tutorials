use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct MyGodotSingleton {
    base: Base<Object>,
}

#[godot_api]
impl MyGodotSingleton {
    #[func]
    pub fn print(&self) {
        godot_warn!("Called MyGodotSigleton");
    }
}
