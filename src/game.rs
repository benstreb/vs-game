use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;
use rand::{Rand, Rng};
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use sprite::{Sprite, Scene};

pub trait Game {
    fn new<R: Rng>(rng: &mut R) -> Box<Self>;
    fn event(&mut self, gl: &mut GlGraphics, event: &Event);
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

    fn dims() -> (i32, i32) {
        return (16, 16);
    }
}

impl Rand for TileColor {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use self::TileColor::*;
        match rng.gen_range(1, 5) {
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
    fn set_position(&mut self, scene: &mut Scene<Texture>, pos: (f64, f64)) {
        let (x, y) = pos;
        scene.child_mut(self.sprite_id).map(|s| s.set_position(x, y));
    }
}

pub struct UnnamedGame {
    grid: Box<[[Option<Tile>; 5]; 5]>,
    scene: Box<Scene<Texture>>,
}

impl Game for UnnamedGame {
    fn new<R: Rng>(rng: &mut R) -> Box<Self> {
        let mut scene = Scene::new();
        let mut grid = [[None; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                let mut tile = Tile::new(rng.gen(), &mut scene);
                let (tile_width, tile_height) = TileColor::dims();
                tile.set_position(&mut scene,
                    ((tile_width*i+tile_width/2) as f64,
                     (tile_height*j+tile_height/2) as f64));
                grid[i as usize][j as usize] = Some(tile);

            }
        }
        Box::new(UnnamedGame {
            grid: Box::new(grid),
            scene: Box::new(scene),
        })
    }
    fn event(&mut self, gl: &mut GlGraphics, e: &Event) {
        use graphics::clear;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.scene.event(e);
        match e {
            &Event::Render(r) => gl.draw(r.viewport(), |c, gl| {
                clear(WHITE, gl);
                self.scene.draw(c.transform, gl)
            }),
            _ => (),
        }
    }
}