use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    input::{UpdateEvent, RenderEvent},
    event_loop::{EventSettings, Events, EventLoop},
    window::{Window, WindowSettings},
};


use crate::ui;
use crate::gl_res;

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

    let mut gui_res = gl_res::GuiResources::new(1024, 1024);

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
                    gui_res.draw_primitives(primitives, c, gl);
                });
                glutin_window.swap_buffers();
            }
        }
    }
}

