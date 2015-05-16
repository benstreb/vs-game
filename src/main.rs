extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use piston::input::{Input, Button, Key};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    paused: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
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

    for e in window.events() {
        match e {
            Event::Render(r) => app.render(&r),
            Event::Update(u) => app.update(&u),
            Event::Input(Input::Press(Button::Keyboard(Key::Space))) => app.pause(),
            _ => (),
        }
    }
}