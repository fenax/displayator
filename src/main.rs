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
            [900, 900]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    println!("{}",serde_json::to_string(&Command::Clear).unwrap());
    println!("{}",serde_json::to_string(&Command::Show).unwrap());
    println!("{}",serde_json::to_string(&Command::Display(Visible::ImageFile("Filename.fil".to_owned()))).unwrap());
    println!("{}",serde_json::to_string(&Command::Display(Visible::Text("The text.".to_owned()))).unwrap());

    

    let mut texture_context = window.create_texture_context();
    let mut loaded_image = Texture::empty(& mut texture_context).unwrap();

    let mut child = std::process::Command::new("./start.sh").stdout(std::process::Stdio::piped())
                                                            .stdin(std::process::Stdio::piped())
                                                            .spawn().expect("failed to launch child process");
    let mut to_child = child.stdin.take().unwrap();
    let mut from_child = nonblock::NonBlockingReader::from_fd( child.stdout.take().unwrap()).unwrap();

//    window.set_lazy(true);

    let mut remainder:std::vec::Vec<u8>  = Vec::new();
    while let Some(e) = window.next() {


        let mut buf :std::vec::Vec<u8> = Vec::new();
        let red = from_child.read_available(&mut buf);
        match red {
            Ok(x) => {
                if x>0 {
                    let workbuffer = vec![remainder.as_slice(),buf.as_slice()].concat();
                    let commands:std::vec::Vec<&[u8]> = workbuffer.split(|x| *x == ('\n' as u8)).collect();
                    for i in 0..commands.len()-1{
                        let command :Command = serde_json::from_slice(commands[i]).unwrap();
                        match command{
                            Command::Clear => println!("Clearing"),
                            Command::Show  => println!("Showing"),
                            Command::Display(Visible::ImageFile(file))=> {
                                println!("displaying file {}",file);
                                loaded_image = Texture::from_path(&mut texture_context, file, Flip::None, &TextureSettings::new()).unwrap();
                            },
                            Command::Display(Visible::Text(text))=>      println!("printing {}",text),
                        }                        
                    }
                    remainder = commands[commands.len()-1].to_vec();
                } 

            },
            _ => panic!("aaaaaaa !"),
        }
        window.draw_2d(&e, |c, g, _| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            image(&loaded_image,c.transform,g);
        });
    }
}