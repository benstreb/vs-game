use std::path::Path;
use std::rc::Rc;
use std::ops::{Index, IndexMut};
use uuid::Uuid;
use rand::{Rand, Rng};
use opengl_graphics::{GlGraphics, Texture};
use piston::event::Event;
use piston::input::{Input, Button, Key};
use ai_behavior::{Sequence, Action};
use sprite::{Sprite, Scene, Ease, EaseFunction, MoveBy};

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
    fn new(color: TileColor, scene: &mut Scene<Texture>) -> Tile {
        let sprite = Sprite::from_texture(color.texture());
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
        let (tile_width, tile_height) = TileColor::dims();
        let mut grid = Box::new([[None; HEIGHT as usize]; WIDTH as usize]);
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let mut tile = Tile::new(rng.gen(), scene);
                tile.set_position(scene,
                    (tile_width as f64 * (i as f64 + 0.5),
                     tile_height as f64 * (j as f64 + 0.5)));
                grid[i as usize][j as usize] = Some(tile);
            }
        }
        Grid {
            grid: grid,
        }
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
        use utils::clamp;
        let (p_x, p_y) = self.player_coords;
        let (x_delta, y_delta) = d.coord_delta();
        let (x, y) = (
            clamp(p_x + x_delta, 0, WIDTH-1),
            clamp(p_y + y_delta, 0, HEIGHT-1)
        );

        self.player_coords = (x, y);
        let (tile_width, tile_height) = TileColor::dims();
        let move_seq = Sequence(vec![
            Action(Ease(EaseFunction::ExponentialInOut,
                        Box::new(MoveBy(1.0,
                                        (tile_width*x_delta) as f64,
                                        (tile_height*y_delta) as f64))))]);
        self.scene.run(self.player_id, &move_seq);
    }

    fn attack(&mut self) {
        if let Some(tile) = self.grid[self.player_coords] {
            self.scene.remove_child(tile.sprite_id);
            self.grid[self.player_coords] = None;
        }
    }
}

impl Game for UnnamedGame {
    fn new<R: Rng>(rng: &mut R) -> Box<Self> {
        let (tile_width, tile_height) = TileColor::dims();
        let mut scene = Scene::new();
        let grid = Grid::new(rng, &mut scene);
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
                    Key::Return => self.attack(),
                    _ => (),
                },
            _ => (),
        }
    }
}