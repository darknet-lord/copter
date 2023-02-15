use macroquad::prelude::*;
use macroquad::{
    experimental::{
        scene::{self, Handle, RefMut},
    },
};

use crate::nodes;
use nodes::Player;

pub struct Camera {
    bounds: Rect,
    player: Handle<Player>,
    viewport_height: f32,
    macroquad_camera: Camera2D,
}

impl Camera {
    pub fn new(bounds: Rect, viewport_height: f32, player: Handle<Player>) -> Camera {
        Camera{
            bounds,
            player,
            viewport_height,
            macroquad_camera: Camera2D::default(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.macroquad_camera.target
    }

    pub fn macroquad_camera(&self) -> &Camera2D {
        &self.macroquad_camera
    }
}

impl scene::Node for Camera {
    fn update(mut node: RefMut<Self>) {
        if let Some(player) = scene::try_get_node::<Player>(node.player) {
            let aspect = screen_width() / screen_height();
            let viewport_width = node.viewport_height * aspect;
            node.macroquad_camera = Camera2D {
                zoom: vec2(
                      1.0 / viewport_width as f32 * 2.,
                      -1.0 / node.viewport_height as f32 * 2.,
                  ),
                  target: vec2(node.pos().x, node.pos().y),
                  ..Default::default()
            }
        }
        scene::set_camera(*node.macroquad_camera());
    }
}
