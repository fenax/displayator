extern crate piston_window;
use piston_window::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
enum Visible{
    ImageFile(String),
    Text(String)
}

#[derive(Serialize, Deserialize)]
enum Command{
    Clear,
    Show,
    Display(Visible),
}


fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "piston: hello_world",
            [200, 200]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();
    let mut texture_context = window.create_texture_context();
    let mut loaded_image = Texture::empty(& mut texture_context).unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            image(&loaded_image,c.transform,g);
        });

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {

        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {

        }
    }
}