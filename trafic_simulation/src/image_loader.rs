use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub fn load_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(path)
}
