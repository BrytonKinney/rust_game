use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use std::path;

mod gui;

struct MainState {
    game_gui: gui::GameGui,
    hidpi_factor: f32,
}

impl MainState {
    fn new(ctx: &mut ggez::Context, hidpi_factor: f32) -> ggez::GameResult<MainState> {
        let sprites: Vec<path::PathBuf> = ggez::filesystem::read_dir(ctx, "/sprites").unwrap().collect();
        let resource_path = ggez::filesystem::resources_dir(ctx);
        let mut sprite_full_paths: Vec<path::PathBuf> = Vec::new();
        for sprite in sprites {
            let fp = path::PathBuf::from(resource_path.join(sprite.strip_prefix("/").unwrap()));
            println!("Path: {}", fp.to_owned().to_str().unwrap());
            sprite_full_paths.push(fp);
        }
        let s = MainState {
            game_gui: gui::GameGui::new(ctx, sprite_full_paths),
            hidpi_factor,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        {
            self.game_gui.render(ctx, self.hidpi_factor);
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.game_gui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.game_gui.update_mouse_pos(x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.game_gui.update_mouse_down((false, false, false));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            _ => (),
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("TileEditor", "Bryton Kinney")
        .add_resource_path(path::PathBuf::from("./resources"))
        .window_setup(ggez::conf::WindowSetup {
            title: "Map Editor".to_owned(),
            samples: ggez::conf::NumSamples::Two,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
            width: 800.0,
            height: 800.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: true,
        });
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(
        ctx,
        event_loop.get_primary_monitor().get_hidpi_factor() as f32,
    )?;
    event::run(ctx, event_loop, state)
}
