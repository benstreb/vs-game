use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use sprite::{Sprite, Scene};

trait Game {
    fn event(&self, gl: &mut GlGraphics, event: &Event);
}

enum TileColor {
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

impl TileColor {
    fn texture(&self) -> Rc<Texture> {
        Rc::new(Texture::from_path(Path::new(match self {
            &TileColor::RED => "./bin/assets/red_box.png",
            &TileColor::GREEN => "./bin/assets/green_box.png",
            &TileColor::BLUE => "./bin/assets/blue_box.png",
            &TileColor::YELLOW => "./bin/assets/yellow_box.png",
        })).unwrap())
    }
}

struct Tile {
    color: TileColor,
    sprite_id: Uuid,
}

impl Tile {
    fn new(color: TileColor, scene: &mut Scene<Texture>) -> Tile {
        let mut sprite = Sprite::from_texture(color.texture());
        let sprite_id = sprite.id();
        scene.add_child(sprite);
        Tile {
            color: color,
            sprite_id: sprite_id,
        }
    }
}

struct UnnamedGame {
    grid: [[Tile; 5]; 5],
    scene: Box<Scene<Texture>>,
}

impl Game for UnnamedGame {
    fn event(&self, gl: &mut GlGraphics, event: &Event) {
    }
}