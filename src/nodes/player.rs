use macroquad::prelude::*;
use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
};
use macroquad_platformer::Actor;
use crate::{Resources};


const MOVE_SPEED: f32 = 10.0;

pub struct Player {
    collider: Actor,
    speed: Vec2,
    flip: bool,
    pos: Vec2,
}

impl Player {
    pub fn new() -> Self {
        let mut resources = storage::get_mut::<Resources>();
        Self {
            collider: resources.collision_world.add_actor(vec2(150.0, 150.0), 95, 32),
            pos: vec2(100.0, 100.0),
            speed: vec2(0., 0.),
            flip: true,
        }
    }
    
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }


    pub fn set_pos(&mut self, pos: &Vec2) {
        self.pos = *pos;
    }
}

impl scene::Node for Player {

    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        let pos = resources.collision_world.actor_pos(node.collider);

        // Debug
        // draw_rectangle(pos.x, pos.y, 95.0, 32.0, BLACK);

        let rotation: f32;
        if node.flip {
            rotation = 0.2;
        } else {
            rotation = -0.2;
        }

        draw_texture_ex(
            resources.copter,
            pos.x,
            pos.y,
            BLACK,
            DrawTextureParams{
                source: Some(Rect::new(0.0, 0.0, 95.0, 32.0)),
                rotation: rotation,
                flip_x: node.flip,
                ..Default::default()
            });
    }

    fn update(mut node: RefMut<Self>) {
        let world = &mut storage::get_mut::<Resources>().collision_world;
        let pos = world.actor_pos(node.collider);

        node.set_pos(&pos);

        let collides_top = world.collide_check(node.collider, pos + vec2(0., -1.));
        let collides_bottom = world.collide_check(node.collider, pos + vec2(0., 1.));
        let collides_right = world.collide_check(node.collider, pos + vec2(1., 0.));
        let collides_left = world.collide_check(node.collider, pos + vec2(-1., 0.));

        if collides_bottom || collides_top {
            node.speed.y = 0.0;
        }

        if collides_left || collides_right {
            node.speed.x = 0.0;
        }

        if is_key_down(KeyCode::W){
            if !collides_top {
                node.speed.y -= MOVE_SPEED;
            }
        }

        if is_key_down(KeyCode::S) {
            if !collides_bottom {
                node.speed.y += MOVE_SPEED;
            }
        }

        if is_key_down(KeyCode::A) {
            node.flip = false;
            if !collides_left {
                node.speed.x -= MOVE_SPEED;
            } 
        }

        if is_key_down(KeyCode::D) {
            node.flip = true;
            if !collides_right {
                node.speed.x += MOVE_SPEED;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let mut bullets = scene::find_node_by_type::<crate::nodes::Bullets>().unwrap();
            bullets.spawn_bullet(node.pos, node.flip);
        }

        world.move_h(node.collider, node.speed.x * get_frame_time());
        world.move_v(node.collider, node.speed.y * get_frame_time());
    }
}

