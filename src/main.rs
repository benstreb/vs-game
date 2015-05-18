extern crate piston;
extern crate graphics;
extern crate ai_behavior;
extern crate sprite;
extern crate glutin_window;
extern crate opengl_graphics;

use std::path::Path;
use std::rc::Rc;
use piston::window::WindowSettings;
use piston::event::*;
use piston::input::{Input, Button, Key};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use sprite::{Sprite, Scene};
use ai_behavior::{Sequence, Behavior};

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    paused: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs, scene: &Scene<Texture>) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            scene.draw(c.transform, gl)
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if !self.paused {
            self.rotation += 2.0 * args.dt;
        }
    }

    fn pause(&mut self) {
        self.paused = !self.paused;
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    let window = Window::new(
        opengl,
        WindowSettings::new(
            "spinning-square",
            [200, 200],
        )
        .exit_on_esc(true)
    );

    let mut app = App{
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        paused: false,
    };

    let mut scene = Scene::new();
    let tex = Path::new("./bin/assets/red_box.png");
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(100.0, 100.0);

    let id = scene.add_child(sprite);
    scene.run(id, &Behavior::WaitForever);

    for e in window.events() {
        match e {
            Event::Render(r) => app.render(&r, &scene),
            Event::Update(u) => app.update(&u),
            Event::Input(Input::Press(Button::Keyboard(key))) => match key {
                Key::Space => app.pause(),
                _ => (),
            },
            _ => (),
        }
        scene.event(&e);
    }
}