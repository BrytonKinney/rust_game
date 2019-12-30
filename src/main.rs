use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use std::vec;
use std::env;
use std::path;

struct MenuOption {
    item_text: String,
}

impl MenuOption {
    fn new(text: String) -> MenuOption {
        MenuOption {
            item_text: text
        }
    }
}

struct Menu {

}

struct MainState {
    background_sprite: graphics::Image,
    default_font: graphics::Fontgi
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let s = MainState {
            background_sprite: graphics::Image::new(ctx, &path::PathBuf::from("/sprites/grass_1.png")).unwrap()
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
        for y in 0..81 {
            for x in 0..81 {
                graphics::draw(ctx, &self.background_sprite, (na::Point2::new(10.0 * (x as f32), 10.0 * (y as f32)),))?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "Bryton Kinney")
                .add_resource_path(path::PathBuf::from("./resources"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}