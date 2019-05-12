use piston_window::*;
use gfx_graphics::Gfx2d;

use crate::ui;
use crate::pist_rend;

pub struct Win {
    pub window: PistonWindow,
    events: Events,
    gui: ui::Gui,
    gui_render: pist_rend::GuiRender,
    g2d: Gfx2d<gfx_device_gl::Resources>,
}

impl Win {
    pub fn new(width: u32) -> Self {
        // window setup
        let opengl = OpenGL::V3_2;
        let settings = WindowSettings::new("piston_window::G2d", [width, 480])
            .opengl(opengl)
            .srgb(false)
            .samples(4);
        let mut window: PistonWindow = settings.build()
            .expect("Could not create window");
        let g2d = Gfx2d::new(opengl, &mut window.factory);

        // conrod setup
        let size = window.size();
        let gui = ui::Gui::new(size.width, size.height);

        let gui_render = pist_rend::GuiRender::new(1024, 1024, &mut window);

        // event loop setup
        let events = Events::new(EventSettings::new().swap_buffers(false));

        Self {
            window,
            events,
            gui,
            gui_render,
            g2d,
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.next(&mut self.window)
    }

    pub fn do_event(&mut self, e: Event){
        let window = &mut self.window;
        let gui = &mut self.gui;
        let g2d = &mut self.g2d;
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
                    gui_render.draw_primitives(primitives, c, g);
                });
                window.encoder.flush(&mut window.device);
                window.window.swap_buffers();
            }
        }
    }
}
