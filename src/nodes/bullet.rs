use macroquad::prelude::*;
use macroquad::{
    experimental::{
        scene::{self, RefMut},
    },
};

const DIRECTION_RIGHT: bool = true;
const DIRECTION_LEFT: bool = false;



pub struct Bullet {
    pos: Vec2,
    direction: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: bool) -> Self {
        // let mut resources = storage::get_mut::<Resources>();
        Self {
            // collider: resources.collision_world.add_actor(vec2(50.0, 50.0), 10, 10),
            pos: vec2(x, y),
            direction: direction,
        }
    }

}


impl scene::Node for Bullet {

    fn draw(node: RefMut<Self>) {
        draw_circle(node.pos.x, node.pos.y, 4.0, BLACK);
    }
    fn update(mut node: RefMut<Self>) {
        if node.direction == DIRECTION_RIGHT {
            node.pos.x  += 10.0;
        } else if node.direction == DIRECTION_LEFT {
            node.pos.x  -= 10.0;
        }
    }
}

