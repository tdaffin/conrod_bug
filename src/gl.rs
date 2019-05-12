use piston_window::PistonWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    input::{Event, UpdateEvent, RenderEvent},
    event_loop::{EventSettings, Events, EventLoop},
    window::{Window, WindowSettings},
};

use crate::ui;
use crate::gl_rend;

pub struct Win {
    pub window: PistonWindow,
    events: Events,
    gui: ui::Gui,
    gui_render: gl_rend::GuiRender,
    gl_graphics: GlGraphics,
}

impl Win {
    pub fn new(width: u32) -> Self {
        // window setup
        let opengl = OpenGL::V3_2;
        let settings = WindowSettings::new("opengl_graphics::GlGraphics", [width, 480])
            .opengl(opengl)
            .srgb(false)
            .samples(4);
        let window: PistonWindow = settings.build()
            .expect("Could not create window");
        let gl_graphics = GlGraphics::new(opengl);

        // conrod setup
        let size = window.size();
        let gui = ui::Gui::new(size.width, size.height);

        let gui_render = gl_rend::GuiRender::new(1024, 1024);

        // event loop setup
        let events = Events::new(EventSettings::new().swap_buffers(false));

        Self {
            window,
            events,
            gui,
            gui_render,
            gl_graphics,
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.next(&mut self.window)
    }

    pub fn do_event(&mut self, e: Event){
        let window = &mut self.window;
        let gui = &mut self.gui;
        let gl_graphics = &mut self.gl_graphics;
        let gui_render = &mut self.gui_render;
        let size = window.size();
        if let Some(e) = conrod_piston::event::convert(e.clone(), size.width, size.height) {
            gui.handle_event(e);
        }
        if let Some(_) = e.update_args() {
            gui.update();
        }
        if let Some(r) = e.render_args() {
            if let Some(primitives) = gui.draw_if_changed() {
                
                gl_graphics.draw(r.viewport(), |c, g| {
                    graphics::clear([0.0, 0.0, 0.0, 1.0], g);
                    gui_render.draw_primitives(primitives, c, g);
                });
                window.window.swap_buffers();
            }
        }
    }
}
