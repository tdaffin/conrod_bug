use piston_window::*;
//use texture::CreateTexture;
use gfx_device_gl;

//type Tex = opengl_graphics::Texture;
type Tex = Texture<gfx_device_gl::Resources>;
//type Gr = G2d;

pub struct GuiRender{
    image_map: conrod_core::image::Map<Tex>,
    glyph_cache: conrod_core::text::GlyphCache<'static>,
    glyph_cache_texture: Tex,
}

impl GuiRender {
    pub fn new(width: u32, height: u32, window: &mut PistonWindow) -> Self {
        let glyph_cache = conrod_core::text::GlyphCache::builder()
            .dimensions(width, height)
            .build();

        let glyph_cache_texture = G2dTexture::from_memory_alpha(
            &mut window.factory,
            &vec![0; (width*height*4) as usize],
            width, height,
            &texture::TextureSettings::new(),
        ).expect("failed to create texture");

        let image_map = conrod_core::image::Map::new();

        Self{
            image_map,
            glyph_cache,
            glyph_cache_texture,
        }
    }

    pub fn draw_primitives<P>(&mut self, primitives: P,
        context: graphics::context::Context,
        graphics: &mut G2d)
        where P: conrod_core::render::PrimitiveWalker
    {
        conrod_piston::draw::primitives(
                primitives,
                context,
                graphics,
                &mut self.glyph_cache_texture,
                &mut self.glyph_cache,
                &self.image_map,
                cache_glyphs,
                |t| t,
            );
    }
}

fn cache_glyphs(
    graphics: &mut G2d,
    texture: &mut Tex,
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
        graphics.encoder,
        texture::Format::Rgba8,
        &new_data,
        [rect.min.x, rect.min.y],
        [rect.width(), rect.height()],
    ).expect("Error updating glyph cache texture");
}
