use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;

use std::time::Duration;

struct Dillo {
  position: Rect
}

impl Dillo {
  fn new(x: i32, y: i32) -> Dillo {
    Dillo {
      position: Rect::from((x, y, 63, 85))
    }
  }

  fn update_position(& mut self, x: i32, y: i32) {
      self.position.set_x(self.position.x + x);
      self.position.set_y(self.position.y + y);
  }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window: Window = video_subsystem.window("Dillos", 2436, 1125)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let _image_context = image::init(InitFlag::PNG)?;

    let mut canvas: Canvas<Window> = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let dillo_texture = texture_creator.load_texture("assets/dillo/dwf/dwf1.png")?;
    let mut dillo1 = Dillo::new(0, 0);

    // TODO
    // let g1 = Rect::from((x, y, width, height): (i32, i32, u32, u32))

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    let mut start_game: bool = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown {..} |
                Event::FingerDown {..} => { 
                    start_game = true;
                }
                _ => {}
            }
        }

        canvas.clear();

        if start_game {
            dillo1.update_position(0, 5);
        }

        canvas.copy(&dillo_texture, None, dillo1.position)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}