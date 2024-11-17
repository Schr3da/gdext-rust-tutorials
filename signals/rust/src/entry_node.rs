use godot::classes::INode2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EntryNode {
    base: Base<Node2D>,
    main_menu_scene: Gd<PackedScene>,
    game_scene: Gd<PackedScene>,
}

#[godot_api]
impl INode2D for EntryNode {
    fn init(base: Base<Node2D>) -> Self {
        let main_menu_scene: Gd<PackedScene> = load("res://main_menu.tscn");
        let game_scene: Gd<PackedScene> = load("res://game_scene.tscn");

        Self {
            base,
            main_menu_scene,
            game_scene,
        }
    }

    fn ready(&mut self) {
        self.main_menu();
    }
}

#[godot_api]
impl EntryNode {
    pub fn main_menu(&mut self) {
        let instance = match self.main_menu_scene.instantiate() {
            Some(instance) => instance,
            None => {
                godot_error!("Unable to load scene");
                return;
            }
        };

        self.base_mut().get_children().clear();
        self.base_mut().add_child(&instance);
    }

    pub fn new_game(&mut self) {
        let instance = match self.game_scene.instantiate() {
            Some(instance) => instance,
            None => {
                godot_error!("Unable to load scene");
                return;
            }
        };

        let mut base = self.base_mut();

        base.get_children().iter_shared().for_each(|c| {
            base.remove_child(&c);
        });

        base.add_child(&instance);
    }
}
