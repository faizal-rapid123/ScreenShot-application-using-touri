#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate  scrap;
extern crate repng;
use scrap::{Capturer, Display};

use std::fs::File;




fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![Screen_shot])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



//for code refer to :  https://github.com/quadrupleslap/scrap/blob/master/examples/screenshot.rs
#[tauri::command]
fn Screen_shot(var:String)
{
  println!("function called");
 

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

   

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
              panic!("something went wrong!! ----- {}",error)
            }
        };

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

       
       let filename = format!("screenshot{}.png",var);
        repng::encode(
            File::create(filename).unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        ).unwrap();

        println!("Image saved to `screenshot.png`.");
      
    }

