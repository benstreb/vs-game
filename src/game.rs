use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use sprite::{Sprite, Scene};

trait Game {
    fn event(&self, gl: &mut GlGraphics, event: &Event);
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
    grid: Box<[[Tile; 5]; 5]>,
    scene: Box<Scene<Texture>>,
}

impl UnnamedGame {
    fn new() -> Box<Game> {
        let mut scene = Scene::new();
        let mut grid = [[Tile::new(TileColor::RED, &mut scene); 5]; 5];
        Box::new(UnnamedGame {
            grid: Box::new(grid),
            scene: Box::new(scene),
        })
    }
}

impl Game for UnnamedGame {
    fn event(&self, gl: &mut GlGraphics, event: &Event) {
    }
}