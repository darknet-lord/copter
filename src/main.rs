use macroquad::prelude::*;
use macroquad::ui::{hash,root_ui,widgets::{self,Group},Ui};
use macroquad_tiled as tiled;
use std::borrow::BorrowMut;
use log::{debug};

const MAX_ROTATION: f32 = 0.1;


struct CopterState<'a> {
    y: &'a mut f32,
    x: &'a mut f32,
    xspeed: &'a mut f32,
    yspeed: &'a mut f32,
    rotation: &'a mut f32,
    direction: &'a mut i8,
}

struct Copter<'a> {
    state: CopterState<'a>,
}

pub trait Movement {
    fn move_left(&mut self) -> ();
    fn move_right(&mut self) -> ();
    fn ascend(&mut self) -> ();
    fn descend(&mut self) -> ();
    fn update_position(&mut self) -> ();
}

impl Movement for Copter<'_>{
    fn move_right(&mut self) -> () {
        if *self.state.xspeed < 10.0 {
            *self.state.xspeed += 0.2;
        };
        if *self.state.rotation < MAX_ROTATION {
            *self.state.rotation += 0.03;
        }
        if *self.state.direction != 1 {
            *self.state.direction = 1;
        }
    }

    fn move_left(&mut self) -> () {
        if *self.state.xspeed > -10.0 {
            *self.state.xspeed -= 0.2;
        };
        if *self.state.rotation > -MAX_ROTATION {
            *self.state.rotation -= 0.03;
        }
        if *self.state.direction != -1 {
            *self.state.direction = -1;
        }
    }
    
    fn ascend(&mut self) -> () {
        if *self.state.yspeed > -10.0 {
            *self.state.yspeed -= 0.2;
        };
    }

    fn descend(&mut self) -> () {
        if *self.state.yspeed < 10.0 {
            *self.state.yspeed += 0.2;
        };
    }

    fn update_position(&mut self) -> () {
        *self.state.x.borrow_mut() += 1.0 * *self.state.xspeed;
        *self.state.y.borrow_mut() += 1.0 * *self.state.yspeed;
    }
}


#[macroquad::main("Quadcopter")]
async fn main() {
    env_logger::init();

    let copter_texture = Texture2D::from_file_with_format(
        include_bytes!("../assets/heli.png"),
        Some(ImageFormat::Png),
    );

    let mut target = (0., 0.);
    let mut zoom = 1.0;
    let mut rotation = 0.0;
    let mut smooth_rotation: f32 = 0.0;
    let mut offset = (0., 0.);


    let initial_state = CopterState{
        y: &mut 0.0,
        x: &mut 0.0,
        xspeed: &mut 0.0,
        yspeed: &mut 0.0,
        rotation: &mut 0.1,
        direction: &mut 1,
    };

    let mut copter = Copter{state: initial_state};

    let tileset = load_texture("assets/tileset.png").await.unwrap();
    // let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset)],
        &[],
    ).unwrap();




    loop {


	/*
        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
            .label("Shop")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                for i in 0..30 {
                    Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
                        ui.label(Vec2::new(10., 10.), &format!("Item N {}", i));
                        ui.label(Vec2::new(260., 40.), "10/10");
                        ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
                        if ui.button(Vec2::new(260., 55.), "buy") {
                            println!("HELLO WORLD");
                        }
                    });
                }
            });
        */
 

        if is_key_down(KeyCode::W) {
            copter.ascend();
        }
        if is_key_down(KeyCode::S) {
            copter.descend();
        }
        if is_key_down(KeyCode::A) {
            copter.move_left();
        }
        if is_key_down(KeyCode::D) {
            copter.move_right();
        }

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }


        copter.update_position();

        clear_background(LIGHTGRAY);

        tiled_map.draw_tiles(
            "terrain",
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            None,
        );

        let copter_bottom = vec2(*copter.state.x + 95. / 2.0, *copter.state.y + 32.);
        let tile = tiled_map.get_tile(
            "terrain",
            (copter_bottom.x / screen_width() * tiled_map.raw_tiled_map.width as f32) as u32,
            (copter_bottom.y / screen_height() * tiled_map.raw_tiled_map.height as f32) as u32);
        debug!(">> {:?}", tile);

        draw_texture_ex(
            copter_texture,
            *copter.state.x,
            *copter.state.y,
            GRAY,
            DrawTextureParams{
                rotation: *copter.state.rotation,
                flip_x: *copter.state.direction == 1,
                ..Default::default()
            });

        next_frame().await
    }

}
