use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, WindowCanvas, TextureCreator, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};

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
}

fn render(canvas: &mut WindowCanvas, color: Color, background: &Texture, dillo: &Texture) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let dillo1 = Dillo::new(0, 0);
    // let (width, height) = canvas.output_size()?;
    // let screen_rect = Rect::from_center(Point::new(width as i32 / 2, height as i32 / 2), 63, 85);
    canvas.copy(background, None, None)?;
    canvas.copy(dillo, None, dillo1.position)?;

    canvas.present();

    Ok(())
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
    let background = texture_creator.load_texture("assets/packs/pack1/level1/lv1_1.png")?;
    let dillo = texture_creator.load_texture("assets/dillo/dwf/dwf1.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        i = (i + 1) % 255;

        render(&mut canvas, Color::RGB(i, 64, 255-i), &background, &dillo)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}