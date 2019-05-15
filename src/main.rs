mod ui;

mod gl_rend;
mod pist_rend;

use piston_window::*;
use gfx_graphics::Gfx2d;

use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let window_width = 640;

    // window setup
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("opengl_graphics::GlGraphics vs piston_window::G2d",
        [window_width, 480])
        .opengl(opengl)
        .srgb(false)
        .samples(4);
    let mut window: PistonWindow = settings.build()
        .expect("Could not create window");

    // conrod setup
    let size = window.size();
    let mut ui = conrod_core::UiBuilder::new([size.width, size.height]).build();
    let font = conrod_core::text::Font::from_bytes(include_bytes!("../assets/ProggyClean.ttf") as &[u8])
        .expect("failed to load font");

    ui.theme.font_id = Some(ui.fonts.insert(font));
    ui.theme.shape_color = conrod_core::color::CHARCOAL;
    ui.theme.label_color = conrod_core::color::WHITE;

    let gui = ui::Gui::new(&mut ui);
    let gui_width = size.width/2.0 - 60.0;

    // event loop setup
    let mut events = Events::new(EventSettings::new().swap_buffers(false));

    // Setup similar to all_piston_window from conrod examples
    let mut g2d = Gfx2d::new(opengl, &mut window.factory);
    let mut pist_render = pist_rend::GuiRender::new(1024, 1024, &mut window);

    // Setup similar to example given in image in piston2d-graphics
    // https://github.com/PistonDevelopers/graphics/blob/master/src/image.rs
    let mut gl_graphics = GlGraphics::new(opengl);
    let mut gl_render = gl_rend::GuiRender::new(1024, 1024);

    let mut state = ui::GuiState { toggle: true };

    // Main event loop
    while let Some(e) = events.next(&mut window){
        let window = &mut window;

        let g2d = &mut g2d;
        let pist_render = &mut pist_render;

        let gl_graphics = &mut gl_graphics;
        let gl_render = &mut gl_render;

        let size = window.size();
        if let Some(e) = conrod_piston::event::convert(e.clone(), size.width, size.height) {
            ui.handle_event(e);
        }
        if let Some(_) = e.update_args() {
            gui.update(&mut ui, &mut state, 0.0, gui_width);
        }

        if let Some(r) = e.render_args() {
            if let Some(primitives) = ui.draw_if_changed() {
                if state.toggle {
                    gl_graphics.draw(r.viewport(), |c, g| {
                        graphics::clear([0.0, 0.0, 0.0, 1.0], g);
                        gl_render.draw_primitives(primitives, c, g);
                    });
                } else {
                    //g2d.draw(r.viewport(), |c, g| {
                    window.window.make_current();
                    //window.g2d.draw(
                    g2d.draw(
                        &mut window.encoder,
                        &window.output_color,
                        &window.output_stencil,
                        r.viewport(),|c, g|{
                    //window.draw_2d(&e, |c, g|{
                        graphics::clear([0.0, 0.0, 0.0, 1.0], g);
                        pist_render.draw_primitives(primitives, c, g);
                    });
                    window.encoder.flush(&mut window.device);
                }
                window.window.swap_buffers();
            }
        }

    }
    
}
