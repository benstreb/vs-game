use opengl_graphics::{GlGraphics, Texture};
use piston::event::RenderArgs;
use sprite::Scene;

trait Game {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
}

enum Tile {
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

struct UnnamedGame {
    grid: [[Tile; 5]; 5],
    scene: Box<Scene<Texture>>,
}

impl Game for UnnamedGame {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
    }
}