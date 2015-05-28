use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;
use rand::{Rand, Rng};
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use sprite::{Sprite, Scene};

trait Game {
    fn new<R: Rng>(rng: &mut R) -> Box<Self>;
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

impl Rand for TileColor {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use game::TileColor::*;
        match rng.gen_range(1, 4) {
            1 => RED,
            2 => GREEN,
            3 => BLUE,
            4 => YELLOW,
            _ => unreachable!()
        }
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
    grid: Box<[[Option<Tile>; 5]; 5]>,
    scene: Box<Scene<Texture>>,
}

impl Game for UnnamedGame {
    fn new<R: Rng>(rng: &mut R) -> Box<Self> {
        let mut scene = Scene::new();
        let mut grid = [[None; 5]; 5];
        for i in 0..4 {
            for j in 0..4 {
                grid[i][j] = Some(
                    Tile::new(rng.gen(), &mut scene));

            }
        }
        Box::new(UnnamedGame {
            grid: Box::new(grid),
            scene: Box::new(scene),
        })
    }
    fn event(&self, gl: &mut GlGraphics, event: &Event) {
    }
}