extern crate piston;
extern crate graphics;
extern crate uuid;
extern crate ai_behavior;
extern crate sprite;
extern crate glutin_window;
extern crate opengl_graphics;

mod game;

use std::path::Path;
use std::rc::Rc;
use uuid::Uuid;
use piston::window::WindowSettings;
use piston::event::*;
use piston::input::{Input, Button, Key};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use sprite::{Sprite, Scene};
use ai_behavior::{Sequence, Behavior};

pub struct App {
    rotation: f64,
    paused: bool,
    game_scene: Box<Scene<Texture>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            self.game_scene.draw(c.transform, gl)
        });
    }

    fn update(&mut self, args: &UpdateArgs, sprite: Uuid) {
        if !self.paused {
            self.rotation += 2.0 * args.dt;
            let (sin, cos) = self.rotation.sin_cos();
            self.game_scene.child_mut(sprite).map(|s|
                s.set_position(100.0 + sin*20.0, 100.0 + cos*20.0)
            );
        }
    }

    fn pause(&mut self) {
        self.paused = !self.paused;
    }

    fn scene_event(&mut self, event: &Event) {
        self.game_scene.event(event);
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

    let mut gl = GlGraphics::new(opengl);

    let mut app = App{
        rotation: 0.0,
        paused: false,
        game_scene: Box::new(Scene::new()),
    };

    let tex = Path::new("./bin/assets/red_box.png");
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    let sprite_id = sprite.id();
    sprite.set_position(100.0, 100.0);

    let id = app.game_scene.add_child(sprite);
    app.game_scene.run(id, &Behavior::WaitForever);

    for e in window.events() {
        match e {
            Event::Render(r) => app.render(&r, &mut gl),
            Event::Update(u) => app.update(&u, sprite_id),
            Event::Input(Input::Press(Button::Keyboard(key))) => match key {
                Key::Space => app.pause(),
                _ => (),
            },
            _ => (),
        }
        app.scene_event(&e);
    }
}