use macroquad::prelude::*;
use macroquad::{
    experimental::{
        collections::storage,
        coroutines::{start_coroutine},
        scene::{self, RefMut},
    },
};
use macroquad_tiled as tiled;
use macroquad_platformer::World as CollisionWorld;

mod nodes;
use nodes::Player;
use nodes::Camera;
use nodes::Bullets;


pub struct BackgroundLayer {}

impl BackgroundLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl scene::Node for BackgroundLayer {
    fn draw(_node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        draw_texture_ex(
            resources.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams{
                dest_size: Some(vec2(3200.0, 3200.0)),
                ..Default::default()
        });
    }
}

pub struct Terrain {}

impl Terrain {
    pub fn new() -> Self {
        Self {}
    }
}

impl scene::Node for Terrain {
    fn draw(_node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        resources.tiled_map.draw_tiles(
            "terrain",
            Rect::new(0.0, 0.0, 100. * 32., 100. * 32.),
            None,
        );
    }
}

struct Resources {
	copter: Texture2D,
	background: Texture2D,
	tiled_map: tiled::Map,
        collision_world: CollisionWorld,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let copter = load_texture("assets/heli.png").await?;
        copter.set_filter(FilterMode::Nearest);

        let background = load_texture("assets/bg.png").await?;
        // let background = load_texture("assets/01.png").await?;
        background.set_filter(FilterMode::Nearest);

        let tileset = load_texture("assets/tiles.png").await?;
        tileset.set_filter(FilterMode::Nearest);
	let tiled_map_json = load_string("assets/map.json").await.unwrap();
	let tiled_map = tiled::load_map(
	    &tiled_map_json,
	    &[("tiles.png", tileset)],
	    &[],
	).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("terrain", None) {
            // static_colliders.push(tile.is_some());
            match tile {
                Some(t) => {static_colliders.push(t.id != 0)},
                _ => {static_colliders.push(false)},
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
            background,
            tiled_map,
            collision_world,
	})
    }
}

fn conf() -> Conf{
    Conf {
        window_title: String::from("Quadcopter"),
        window_width: 30 * 100,
        window_height: 30 * 100,
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
    
    scene::add_node(BackgroundLayer::new());
    scene::add_node(Terrain::new());
    let player = scene::add_node(Player::new());
    scene::add_node(Bullets::new(player));
    scene::add_node(Camera::new(Rect::new(0.0, 0.0, 400.0, 400.0), 700.0, player));

    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
