mod ui;

mod gl_rend;
mod gl;

mod pist_rend;
mod pist;

fn main() {
    gl::run();
    pist::run();
}
