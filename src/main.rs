extern crate piston;
extern crate graphics;
extern crate uuid;
extern crate ai_behavior;
extern crate sprite;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod game;
mod utils;

use rand::thread_rng;
use piston::window::WindowSettings;
use piston::event::Events;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use game::{Game, UnnamedGame};

fn main() {
    let opengl = OpenGL::_3_2;

    let window = Window::new(
        opengl,
        WindowSettings::new(
            "vs-game",
            [200, 200],
        )
        .exit_on_esc(true)
    );

    let mut gl = GlGraphics::new(opengl);

    let mut rand = rand::thread_rng();

    let mut game = game::UnnamedGame::new(&mut rand);

    for e in window.events() {
        game.event(&mut gl, &e);
    }
}