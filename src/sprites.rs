use ggez::graphics::Image;

pub type SpriteId = usize;

pub const SMILEY: usize = 0;

// We load all the sprites into ggez and return a reference to all of them
pub fn load_sprites(ctx: &mut ggez::Context) -> Vec<Image> {
    let mut sprites = Vec::new();

    // 0
    sprites.push(Image::new(ctx, "/smiley.png").unwrap());

    sprites
}