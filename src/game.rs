use std::path::Path;
use std::rc::Rc;
use std::ops::{Index, IndexMut};
use std::borrow::BorrowMut;
use uuid::Uuid;
use rand::{Rand, Rng};
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use piston::input::{Input, Button, Key};
use ai_behavior::{Sequence, Action};
use sprite::{Sprite, Scene, Ease, EaseFunction, MoveBy};

pub trait Game<R: Rng> {
    fn new(rng: &mut R) -> Box<Self>;
    fn event(&mut self, rng: &mut R, gl: &mut GlGraphics, event: &Event);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileColor {
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

impl TileColor {
    fn color(&self) -> (f32, f32, f32) {
        match self {
            &TileColor::RED => (1.0, 0.0, 0.0),
            &TileColor::GREEN => (0.0, 1.0, 0.0),
            &TileColor::BLUE => (0.0, 0.0, 1.0),
            &TileColor::YELLOW => (1.0, 1.0, 0.0),
        }
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn coord_delta(self) -> (i32, i32) {
        use self::Direction::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    color: TileColor,
    sprite_id: Uuid,
}

impl Tile {
    fn new(color: TileColor, scene: &mut Scene<Texture>, pos: (f64, f64)) -> Tile {
        let mut sprite =
            Sprite::from_texture(Rc::new(Texture::from_path(Path::new("./bin/assets/box.png")).unwrap()));
        let (r, g, b) = color.color();
        sprite.set_color(r, g, b);
        let (x, y) = pos;
        sprite.set_position(x, y);
        let sprite_id = sprite.id();
        scene.add_child(sprite);
        Tile {
            color: color,
            sprite_id: sprite_id,
        }
    }
}

const WIDTH: i32 = 5;
const HEIGHT: i32 = 5;

struct Grid {
    grid: Box<[[Option<Tile>; HEIGHT as usize]; WIDTH as usize]>,
}

impl Index<(i32, i32)> for Grid{
    type Output = Option<Tile>;

    fn index(&self, indexes: (i32, i32)) -> &Option<Tile> {
        let (x, y) = indexes;
        &self.grid[x as usize][y as usize]
    }
}

impl IndexMut<(i32, i32)> for Grid{
    fn index_mut(&mut self, indexes: (i32, i32)) -> &mut Option<Tile> {
        let (x, y) = indexes;
        &mut self.grid[x as usize][y as usize]
    }
}

impl Grid {
    fn new<R: Rng>(rng: &mut R, scene: &mut Scene<Texture>) -> Self {
        let mut grid = Box::new([[None; HEIGHT as usize]; WIDTH as usize]);
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let tile = Tile::new(rng.gen(), scene, Grid::to_coords(i, j));
                grid[i as usize][j as usize] = Some(tile);
            }
        }
        Grid {
            grid: grid,
        }
    }

    fn transform(&mut self, f: &mut FnMut(&mut Option<Tile>)) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                f(&mut self.grid[i as usize][j as usize]);
            }
        }
    }

    fn to_coords(x: i32, y: i32) -> (f64, f64) {
        let (tile_width, tile_height) = TileColor::dims();
        (tile_width as f64 * (x as f64 + 0.5),
            tile_height as f64 * (y as f64 + 0.5))
    }
}

pub struct UnnamedGame {
    grid: Grid,
    scene: Box<Scene<Texture>>,
    player_coords: (i32, i32),
    player_id: Uuid,
}

impl UnnamedGame {
    fn move_player(&mut self, d: Direction) {
        if self.scene.running() == 0 {
            use utils::move_clamp;
            let (x_delta, y_delta) = move_clamp(
                d.coord_delta(),
                self.player_coords,
                (WIDTH, HEIGHT),
            );
            let (p_x, p_y) = self.player_coords;
            self.player_coords = (
                p_x + x_delta,
                p_y + y_delta,
            );
            let (tile_width, tile_height) = TileColor::dims();
            let move_seq = Sequence(vec![Action(
                Ease(EaseFunction::ExponentialInOut,
                     Box::new(MoveBy(0.5,
                                     (tile_width*x_delta) as f64,
                                     (tile_height*y_delta) as f64))))]);
            self.scene.run(self.player_id, &move_seq);
        }
    }

    fn attack<R: Rng>(&mut self, rng: &mut R) {
        if let Some(tile) = self.grid[self.player_coords] {
            self.grid.transform(&mut |t: &mut Option<Tile>| {
                if let Some(&mut other_tile) = t.as_mut() {
                    if tile.color == other_tile.color {
                        let (r, g, b) = rng.gen::<TileColor>().color();
                    }
                }
            });
        }
    }
}

impl<R: Rng> Game<R> for UnnamedGame {
    fn new(mut rng: &mut R) -> Box<Self> {
        let (tile_width, tile_height) = TileColor::dims();
        let mut scene = Scene::new();
        let grid = Grid::new::<R>(rng.borrow_mut(), &mut scene);
        let mut player = Sprite::from_texture(Rc::new(Texture::from_path(
            Path::new("./bin/assets/player.png")).unwrap()));
        player.set_position((tile_width/2) as f64, (tile_height/2) as f64);
        let player_id = player.id();
        scene.add_child(player);
        Box::new(UnnamedGame {
            grid: grid,
            scene: Box::new(scene),
            player_coords: (0, 0),
            player_id: player_id,
        })
    }

    fn event(&mut self, rng: &mut R, gl: &mut GlGraphics, e: &Event) {
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
                    Key::Return | Key::Space => self.attack(rng),
                    _ => (),
                },
            _ => (),
        }
    }
}