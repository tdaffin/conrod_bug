use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    input::{UpdateEvent, RenderEvent},
    event_loop::{EventSettings, Events, EventLoop},
    window::{Window, WindowSettings},
};
use texture::CreateTexture;
use conrod_core::{
    Positionable,
    Labelable,
    Sizeable,
    Borderable,
    widget_ids,
    widget::Widget,
};

widget_ids! {
    struct Ids {
        list,
    }
}

fn main() {

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
    let font = conrod_core::text::Font::from_bytes(include_bytes!("../assets/ProggyClean.ttf") as &[u8])
        .expect("failed to load font");

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
    let mut ui = conrod_core::UiBuilder::new([size.width, size.height]).build();
    let ids = Ids::new(ui.widget_id_generator());

    ui.theme.font_id = Some(ui.fonts.insert(font));
    ui.theme.shape_color = conrod_core::color::CHARCOAL;
    ui.theme.label_color = conrod_core::color::WHITE;

    // event loop setup
    let mut events = Events::new(EventSettings::new().swap_buffers(false));
    while let Some(e) = events.next(&mut glutin_window) {

        let size = glutin_window.size();
        if let Some(e) = conrod_piston::event::convert(e.clone(), size.width, size.height) {
            ui.handle_event(e);
        }
        if let Some(_) = e.update_args() {
            set_ui(&mut ui, &ids);
        }
        if let Some(r) = e.render_args() {
            if let Some(primitives) = ui.draw_if_changed() {
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

fn set_ui(ui: &mut conrod_core::Ui, ids: &Ids) {
    let ui = &mut ui.set_widgets();

    let (mut list_items_iter, scrollbar) = conrod_core::widget::List::flow_down(20)
        .top_left_with_margins_on(ui.window, ui.win_h/2.0-50.0, 30.0)
        .item_size(35.0)
        .h(ui.win_h/2.0-30.0)
        .w(ui.win_w/2.0-60.0)
        .scrollbar_on_top()
        .set(ids.list, ui);

    scrollbar.map(|s| s.set(ui));

    let mut i = 1;
    while let Some(item) = list_items_iter.next(ui) {
        let lbl = format!("Item {}", i);
        let button = conrod_core::widget::Button::new()
            .label(&lbl)
            .border(1.0)
            .border_color(conrod_core::color::WHITE)
            .label_font_size(15);
        item.set(button, ui);
        i += 1;
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
