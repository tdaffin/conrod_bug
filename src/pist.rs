use piston_window::*;

use crate::ui;
use crate::pist_rend;

pub fn run(){
    // window setup
    let settings = WindowSettings::new("test", [640, 480])
        .srgb(false)
        .samples(4);
        //.exit_on_esc(true).samples(1);
    let mut window: PistonWindow = settings.build()
        .expect("Could not create window");
    //let mut gl_graphics = GlGraphics::new(opengl);

    // conrod setup
    let size = window.size();
    let mut gui = ui::Gui::new(size.width, size.height);

    let mut gui_res = pist_rend::GuiRender::new(1024, 1024, &mut window);

    // event loop setup
    let mut events = Events::new(EventSettings::new());//.swap_buffers(false));
    while let Some(e) = events.next(&mut window) {

        let size = window.size();
        if let Some(e) = conrod_piston::event::convert(e.clone(), size.width, size.height) {
            gui.handle_event(e);
        }
        if let Some(_) = e.update_args() {
            gui.update();
        }
        if let Some(r) = e.render_args() {
            if let Some(primitives) = gui.draw_if_changed() {
                //gl_graphics.draw(r.viewport(), |c, gl| {
                window.window.make_current();
                window.g2d.draw(
                    &mut window.encoder,
                    &window.output_color,
                    &window.output_stencil,
                    r.viewport(),|c, g|{
                //window.draw_2d(&e, |c, g|{
                    graphics::clear([0.0, 0.0, 0.0, 1.0], g);
                    gui_res.draw_primitives(primitives, c, g);
                });
                window.encoder.flush(&mut window.device);
                //window.swap_buffers();
            }
        }
    }
}

