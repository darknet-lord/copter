use macroquad::prelude::*;
use macroquad::{
    experimental::{
        scene::{self, RefMut},
        collections::storage,
    },
};

use crate::Resources;
use crate::Player;


pub struct Bullet {
    pos: Vec2,
    speed: Vec2,
    lived: f32,
    lifetime: f32,
}

pub struct Bullets {
    player: scene::Handle<Player>,
    bullets: Vec<Bullet>,
}

impl Bullets {
    pub fn new(player: scene::Handle<Player>) -> Bullets {
        Bullets {
            player,
            bullets: Vec::with_capacity(200),
        }
    }

    pub fn spawn_bullet(&mut self, pos: Vec2, facing: bool) {
        let direction = if facing {
            vec2(1.0, 0.0)
        } else {
            vec2(-1.0, 0.0)
        };
        self.bullets.push(Bullet{
            pos: pos + vec2(16.0, 30.0) + direction * 32.0,
            speed: direction * 100.,
            lived: 0.0,
            lifetime: 10.0,
        });
    }
}

impl scene::Node for Bullets {
    fn draw(node: RefMut<Self>) {
        for bullet in &node.bullets {
            draw_circle(
                bullet.pos.x,
                bullet.pos.y,
                2.,
                Color::new(1.0, 0.2, 0.2, 1.0),
            );
        }
    }

    fn update(mut node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        let mut player = scene::get_node(node.player);
        // let mut others = scene::find_node_by_type::<RemotePlayer>();
        for bullet in &mut node.bullets {
            bullet.pos += bullet.speed * get_frame_time();
            bullet.lived += get_frame_time();
        }

        node.bullets.retain(|bullet| {
            let self_damaged = Rect::new(player.get_pos().x, player.get_pos().y, 20., 64.)
                .contains(bullet.pos);

            if self_damaged {
                // let direction = bullet.pos.x > (player.get_pos().x + 10.);
                // TODO: fix kill
                // player.kill(direction);
            }

            if resources.collision_world.solid_at(bullet.pos) || self_damaged {
                // TODO: load hit_fixes
                // resources.hit_fixes.spawn(bullet.pos);
                return false;
            }
            bullet.lived < bullet.lifetime
        });
    }
}
