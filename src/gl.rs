use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    input::{UpdateEvent, RenderEvent},
    event_loop::{EventSettings, Events, EventLoop},
    window::{Window, WindowSettings},
};
use texture::CreateTexture;

use crate::ui;

pub fn run(){
    // window setup
    let opengl = OpenGL::V3_2;
    let mut glutin_window: GlutinWindow = WindowSettings::new("test", [640, 480])
        .opengl(opengl)
        .srgb(false)
        .samples(4)
        .build()
        .expect("Could not create window");
    let mut gl_graphics = GlGraphics::new(opengl);

    // conrod setup
    let size = glutin_window.size();
    let mut gui = ui::Gui::new(size.width, size.height);

    let mut glyph_cache = conrod_core::text::GlyphCache::builder()
        .dimensions(1024, 1024)
        .build();

    let mut glyph_cache_texture = opengl_graphics::Texture::create(
        &mut (),
        texture::Format::Rgba8,
        &vec![0; 1024*1024*4],
        [1024, 1024],
        &texture::TextureSettings::new(),
    ).expect("failed to create texture");

    let image_map = conrod_core::image::Map::new();
    let size = glutin_window.size();


    // event loop setup
    let mut events = Events::new(EventSettings::new().swap_buffers(false));
    while let Some(e) = events.next(&mut glutin_window) {

        let size = glutin_window.size();
        if let Some(e) = conrod_piston::event::convert(e.clone(), size.width, size.height) {
            gui.handle_event(e);
        }
        if let Some(_) = e.update_args() {
            gui.update();
        }
        if let Some(r) = e.render_args() {
            if let Some(primitives) = gui.draw_if_changed() {
                gl_graphics.draw(r.viewport(), |c, gl| {
                    graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                    conrod_piston::draw::primitives(
                        primitives,
                        c,
                        gl,
                        &mut glyph_cache_texture,
                        &mut glyph_cache,
                        &image_map,
                        cache_glyphs,
                        |t| t,
                    );
                });
                glutin_window.swap_buffers();
            }
        }
    }
}



fn cache_glyphs(
    _graphics: &mut opengl_graphics::GlGraphics,
    texture: &mut opengl_graphics::Texture,
    rect: conrod_core::text::rt::Rect<u32>,
    data: &[u8]
) {
    let mut new_data = Vec::with_capacity((rect.width() * rect.height() * 4) as usize);
    for &a in data {
        new_data.push(255);
        new_data.push(255);
        new_data.push(255);
        new_data.push(a);
    }
    texture::UpdateTexture::update(
        texture,
        &mut (),
        texture::Format::Rgba8,
        &new_data,
        [rect.min.x, rect.min.y],
        [rect.width(), rect.height()],
    ).expect("Error updating glyph cache texture");
}
