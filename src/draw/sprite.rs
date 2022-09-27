use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::LoadTexture;

struct Sprite<'a>{
    texture: Texture<'a>
}

impl Sprite<'_>{
    fn new(&mut self, canvas: &WindowCanvas, image: &str){
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(image);
        self.texture = texture.unwrap();
    }

    fn render(&self, mut canvas: WindowCanvas){
        canvas.copy(&self.texture, None, None);
    }
}