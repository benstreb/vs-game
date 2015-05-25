use opengl_graphics::{GlGraphics, Texture};
use piston::event::RenderArgs;
use sprite::Scene;

trait Game {
    fn event(&self, gl: &mut GlGraphics, event: &Event);
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
    fn event(&self, gl: &mut GlGraphics, event: &Event) {
    }
}