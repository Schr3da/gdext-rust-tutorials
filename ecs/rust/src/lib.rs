mod nodes;
mod ecs;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
