mod ui;

mod gl_rend;
mod gl;

mod pist_rend;
mod pist;

use piston_window::AdvancedWindow;

fn main() {
    let width = 320;

    let mut gl = gl::Win::new(width);
    let mut pist = pist::Win::new(width);

    while let Some(e) = gl.next_event(){
        gl.do_event(e);
    }

    if let Some(glpos) = gl.window.get_position() {
        let mut pos = glpos;
        pos.x += width as i32 + 10;
        pist.window.set_position(pos);
    }
    while let Some(e) = pist.next_event(){
        pist.do_event(e);
    }
    
}
