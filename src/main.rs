mod ui;

mod gl_res;
mod gl;

mod pist_res;
mod pist;

fn main() {
    gl::run();
    pist::run();
}
