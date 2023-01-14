use macroquad::prelude::*;
use macroquad::{
    experimental::{
        collections::storage,
        coroutines::{start_coroutine,wait_seconds},
        scene::{self, Handle, RefMut},
    },
    ui::{
        hash,
        root_ui,
        widgets::{self,Group},
        Ui,
    }
};
use macroquad_tiled as tiled;
use macroquad_platformer::World as CollisionWorld;
use macroquad_platformer::Actor;
use std::borrow::BorrowMut;
use log::{debug};
use std::{thread, time};
const DIRECTION_RIGHT: bool = true;
const DIRECTION_LEFT: bool = false;

const MAX_ROTATION: f32 = 0.1;
const MOVE_SPEED: f32 = 10.0;

pub struct Player {
    collider: Actor,
    speed: Vec2,
    pos: Vec2,
    flip: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut resources = storage::get_mut::<Resources>();
        Self {
            collider: resources.collision_world.add_actor(vec2(50.0, 50.0), 30, 30),
            pos: vec2(100.0, 100.0),
            speed: vec2(0., 0.),
            flip: true,
        }
    }

}

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

pub struct Terrain {}

impl Terrain {
    pub fn new() -> Self {
        let mut resources = storage::get_mut::<Resources>();
        Self {}
    }
}

impl scene::Node for Terrain {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        resources.tiled_map.draw_tiles(
            "terrain",
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            None,
        );

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


impl scene::Node for Player {

    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        let pos = resources.collision_world.actor_pos(node.collider);
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
            if node.flip {
                scene::add_node(Bullet::new(pos.x + 100.0, pos.y + 30.0, node.flip));
            } else {
                scene::add_node(Bullet::new(pos.x - 10.0, pos.y + 30.0, node.flip));

            }
        }

        world.move_h(node.collider, node.speed.x * get_frame_time());
        world.move_v(node.collider, node.speed.y * get_frame_time());
    }
}


struct Resources {
	copter: Texture2D,
	tiled_map: tiled::Map,
        collision_world: CollisionWorld,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {

	let copter = Texture2D::from_file_with_format(
	    include_bytes!("../assets/heli.png"),
	    Some(ImageFormat::Png),
	);
        let tileset = load_texture("assets/tileset.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);
	let tiled_map_json = load_string("assets/map.json").await.unwrap();
	let tiled_map = tiled::load_map(
	    &tiled_map_json,
	    &[("tileset.png", tileset)],
	    &[],
	).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("terrain", None) {
            match tile {
                Some(t) => {static_colliders.push(t.id != 8)},
                _ => {static_colliders.push(true)},
            }
        }
        let mut collision_world = CollisionWorld::new();
        collision_world.add_static_tiled_layer(
            static_colliders,
            32.,
            32.,
            tiled_map.raw_tiled_map.width as _,
            1,
        );
        
	Ok(Resources{
            copter,
            tiled_map,
            collision_world,
	})
    }
}

fn conf() -> Conf{
    Conf {
        window_title: String::from("Quadcopter"),
        window_width: 33 * 32,
        window_height: 32 * 32,
        fullscreen: false,
        ..Default::default()
}}

#[macroquad::main(conf)]
async fn main() {
    env_logger::init();

    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while resources_loading.is_done() == false {
        clear_background(BLACK);
        draw_text(
            &format!(
                "Loading resources {}",
                ".".repeat(((get_time() * 2.0) as usize) % 4)
            ),
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0,
            40.,
            WHITE,
        );
        next_frame().await;
    }
    
    scene::add_node(Terrain::new());
    scene::add_node(Player::new());

    loop {
        clear_background(GRAY);
        next_frame().await;
    }

}
