use crate::nodes::prelude::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainMenuNode {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MainMenuNode {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MainMenuNode {
    #[func]
    pub fn new_game_pressed(&mut self) {
        let parent = match self.base().get_parent() {
            Some(p) => p,
            None => {
                godot_error!("unable to find parent");
                return;
            }
        };

        let mut node = match parent.try_cast::<EntryNode>() {
            Err(_) => {
                godot_error!("Unable to cast to Entry Node");
                return;
            }
            Ok(n) => n,
        };

        node.bind_mut().new_game();
    }
}
