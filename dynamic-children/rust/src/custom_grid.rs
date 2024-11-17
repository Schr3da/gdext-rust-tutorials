use godot::classes::Label;
use godot::obj::{Base, NewAlloc};
use godot::prelude::*;
use godot::{
    classes::{GridContainer, IGridContainer},
    register::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(base=GridContainer)]
pub struct CustomGrid {
    base: Base<GridContainer>,
}

#[godot_api]
impl IGridContainer for CustomGrid {
    fn init(base: Base<GridContainer>) -> Self {
        let mut instance = Self { base };

        instance.init_children();

        instance
    }
}

#[godot_api]
impl CustomGrid {
    pub fn init_children(&mut self) {
        for x in 0..10 {
            let mut label = Label::new_alloc();
            let text = format!("Item: {}", x).to_godot();
            label.set_text(&text);
            self.base.to_gd().add_child(&label.upcast::<Node>());
        }
    }
}
