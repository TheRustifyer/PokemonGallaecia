use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[derive(Debug)]
pub struct Map;


#[methods]
impl Map {
    fn new(_owner: &Node2D) -> Self {       
        Self
    }

    // TODO Handle map resources from within the initializers of this object
}