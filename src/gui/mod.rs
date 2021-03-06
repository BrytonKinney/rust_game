use gfx_core::factory::Factory;
use gfx_core::{format, texture};
use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;
use ggez;
use ggez::graphics;
use ggez::Context;
use ggez::{filesystem, GameResult};
use imgui::*;
use imgui_gfx_renderer::*;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct GameGui {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
    tile_texture_ids: Vec<Option<TextureId>>,
}

impl GameGui {
    pub fn new(ctx: &mut Context, sprites: Vec<std::path::PathBuf>) -> Self {
        // Create the imgui object
        let mut imgui = imgui::Context::create();
        let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

        // Shaders
        let shaders = {
            let version = gfx_device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        };

        // Renderer
        let mut sample_textures = Vec::new();
        for tile in sprites {
            let tile_img = image::open(tile).unwrap().to_rgba();
            let tile_img_dim = tile_img.dimensions();
            let texture_kind = texture::Kind::D2(
                tile_img_dim.0 as texture::Size,
                tile_img_dim.1 as texture::Size,
                texture::AaMode::Single,
            );
            let (_, texture_view) = factory
                .create_texture_immutable_u8::<format::Srgba8>(
                    texture_kind,
                    texture::Mipmap::Provided,
                    &[tile_img.into_raw().as_slice()],
                )
                .unwrap();
            let sampler = factory.create_sampler(texture::SamplerInfo::new(
                texture::FilterMethod::Bilinear,
                texture::WrapMode::Clamp,
            ));
            sample_textures.push((texture_view, sampler));
        }
        let mut renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();
        let mut tile_imgs = Vec::new();
        for (texture_view, sample) in sample_textures {
            let texture_id = renderer.textures().insert((texture_view, sample));
            tile_imgs.push(Some(texture_id));
        }
        // Create instace
        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            tile_texture_ids: tile_imgs,
        }
    }

    pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32) {
        // Update mouse
        self.update_mouse();

        // Create new frame
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();

        // Various ui things
        {
            let tiles = &self.tile_texture_ids;
            // Window
            let height = draw_height / 25.0;
            Window::new(im_str!("Tiles"))
                .size([150.0, height], imgui::Condition::FirstUseEver)
                .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                .movable(true)
                .resizable(true)
                .build(&ui, || {
                    for tile in tiles {
                        let img = ImageButton::new(tile.unwrap(), [10.0, 10.0]);
                        if img.build(&ui) {
                            println!("Texture selected: {:?}", tile.unwrap());
                        };
                    }
                });
        }
        // Render
        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer
            .render(
                &mut *factory,
                encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data,
            )
            .unwrap();
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos =
            [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        self.mouse_state.pressed = pressed;
    }
}
