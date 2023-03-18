use macroquad::prelude::*;
use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
        state_machine::{State, StateMachine},
        coroutines::{start_coroutine, Coroutine, wait_seconds},
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
    state_machine: StateMachine<RefMut<Player>>,
    dead: bool,
}

impl Player {

    const STATE_NORMAL: usize = 0;
    const STATE_DEATH: usize = 1;
    const STATE_SHOOT: usize = 2;
    const STATE_AFTERMATCH: usize = 3;

    pub fn new() -> Self {
        let mut resources = storage::get_mut::<Resources>();

        let mut state_machine = StateMachine::new();

        state_machine.add_state(
            Self::STATE_NORMAL, State::new().update(Self::update_normal)
        );
        state_machine.add_state(
            Self::STATE_DEATH, State::new().coroutine(Self::death_coroutine)
        );
        state_machine.add_state(
            Self::STATE_SHOOT,
            State::new()
                .update(Self::update_shoot)
                .coroutine(Self::shoot_coroutine)
        );
        state_machine.add_state(
            Self::STATE_AFTERMATCH,
            State::new().update(Self::update_aftermatch)
        );

        Self {
            collider: resources.collision_world.add_actor(vec2(150.0, 150.0), 95, 32),
            pos: vec2(100.0, 100.0),
            speed: vec2(0., 0.),
            flip: true,
            state_machine: state_machine,
            dead: false,
        }
    }

    pub fn update_normal(node: &mut RefMut<Player>, _dt: f32) {}

    pub fn update_shoot(node: &mut RefMut<Player>, _dt: f32) {}

    pub fn shoot_coroutine(node: &mut RefMut<Player>) -> Coroutine {
        let handle = node.handle();
        let coroutine = async move {
            let node = &mut *scene::get_node(handle);
            // TODO: Add shooting.
            println!("POOF!");
            node.state_machine.set_state(Self::STATE_NORMAL);

        };
        start_coroutine(coroutine)
    }

    pub fn death_coroutine(node: &mut RefMut<Player>) -> Coroutine {
        let handle = node.handle();
        let coroutine = async move {
            let mut node = scene::get_node(handle);
            node.dead = true;
            // TODO: Add some actions.
            println!("You are dead");
        };
        start_coroutine(coroutine)
    }

    pub fn update_aftermatch(node: &mut RefMut<Player>, _dt: f32) {
        node.speed.x = 0.0;
    }
    
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: &Vec2) {
        self.pos = *pos;
    }

    pub fn kill(&mut self, direction: bool) {
        self.state_machine.set_state(Self::STATE_DEATH);
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

        if node.dead {
            node.state_machine.set_state(Self::STATE_AFTERMATCH);
        } else {
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

        StateMachine::update_detached(&mut node, |node| &mut node.state_machine);

    }
}