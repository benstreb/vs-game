use std::path::Path;
use std::rc::Rc;
use std::ops::Add;
use uuid::Uuid;
use rand::{Rand, Rng};
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use piston::input::{Input, Button, Key};
use sprite::{Sprite, Scene};

pub trait Game {
    fn new<R: Rng>(rng: &mut R) -> Box<Self>;
    fn event(&mut self, gl: &mut GlGraphics, event: &Event);
    fn move_player(&mut self, d: Direction);
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
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn coord_delta(self) -> Coord {
        use self::Direction::*;
        match self {
            Up => Coord{x: 0, y: -1},
            Down => Coord{x: 0, y: 1},
            Left => Coord{x: -1, y: 0},
            Right => Coord{x: 1, y: 0},
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
    player_coords: Coord,
    player_id: Uuid,
}

impl Game for UnnamedGame {
    fn new<R: Rng>(rng: &mut R) -> Box<Self> {
        let (tile_width, tile_height) = TileColor::dims();
        let mut scene = Scene::new();
        let mut grid = [[None; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                let mut tile = Tile::new(rng.gen(), &mut scene);
                tile.set_position(&mut scene,
                    ((tile_width*i+tile_width/2) as f64,
                     (tile_height*j+tile_height/2) as f64));
                grid[i as usize][j as usize] = Some(tile);

            }
        }
        let mut player = Sprite::from_texture(Rc::new(Texture::from_path(
            Path::new("./bin/assets/player.png")).unwrap()));
        player.set_position((tile_width/2) as f64, (tile_height/2) as f64);
        let player_id = player.id();
        scene.add_child(player);
        Box::new(UnnamedGame {
            grid: Box::new(grid),
            scene: Box::new(scene),
            player_coords: Coord{x: 0, y: 0},
            player_id: player_id,
        })
    }

    fn move_player(&mut self, d: Direction) {
        self.player_coords = self.player_coords + d.coord_delta();
        let (tile_width, tile_height) = TileColor::dims();
        let Coord{x, y} = self.player_coords;
        self.scene.child_mut(self.player_id).map(|p| p.set_position(
            tile_width as f64 * (x as f64 + 0.5),
            tile_height as f64 * (y as f64 + 0.5),
        ));
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
            &Event::Input(Input::Press(Button::Keyboard(k))) => 
                match k {
                    Key::Up => self.move_player(Direction::Up),
                    Key::Down => self.move_player(Direction::Down),
                    Key::Left => self.move_player(Direction::Left),
                    Key::Right => self.move_player(Direction::Right),
                    _ => (),
                },
            _ => (),
        }
    }
}