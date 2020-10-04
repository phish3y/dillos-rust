use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use std::time::Duration;

struct Dillo {
  position: Rect,
  x_speed: i32,
  y_speed: i32,
  collision_rect: Option<Rect>
}

impl Dillo {
  fn new(x: i32, y: i32) -> Dillo {
    Dillo {
      position: Rect::from((x, y, 63, 85)),
      x_speed: 0,
      y_speed: 0,
      collision_rect: None
    }
  }

  fn update_position(& mut self) {
      self.position.set_x(self.position.x + self.x_speed);
      self.position.set_y(self.position.y + self.y_speed);
  }
}

trait Collider {
    fn new(x: i32, y: i32, width: u32, heigh: u32) -> Self;
    fn get_rect(&self) -> Rect;
}

struct Ground {
    rect: Rect
}

impl Collider for Ground {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Ground {
        Ground {
            rect: Rect::new(x, y, width, height)      
        }
    }

    fn get_rect(&self) -> Rect {
        return self.rect
    }
}

fn collide<T: Collider>(colliding_objects: &Vec<&T>, dillo: &mut Dillo) {
    let mut colliding_objects_count = 0;
    for colliding_object in colliding_objects {
        let collision: Option<Rect> = colliding_object.get_rect().intersection(dillo.position);
        if let Some(current_collision) = collision {
            if let Some(previous_collision) = dillo.collision_rect {
                // there's a previous intersect, see if the height or width has changed
                // if height changed, set y_speed to 0, if width changed, reverse direction
                if current_collision.height() != previous_collision.height() {
                    // we must be colliding on the y axis
                    dillo.y_speed = 0;
                    break
                }

            } else {
                // no previous intersect, just updated the intersect on the ground object
                dillo.collision_rect = Some(current_collision);
            }
        }
        colliding_objects_count += 1;
    }

    if colliding_objects_count == colliding_objects.len() {
        // nothing collided
        dillo.y_speed = 5;
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
    let dillo_texture: Texture = texture_creator.load_texture("assets/dillo/dwf/dwf1.png")?;
    let mut dillo1 = Dillo::new(0, 0);

    // x, y, width, height
    let g1 = Ground::new(0, 700, 650, 200);
    let g2 = Ground::new(250, 800, 650, 200);

    let colliders = vec!(&g1, &g2);

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    let mut start_game: bool = false;
    let mut start_speed_set: bool = false;
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

        if start_game {
            if !start_speed_set {
                dillo1.y_speed = 5;
                dillo1.x_speed = 3;
                start_speed_set = true;
            }

            dillo1.update_position(); 

            collide(&colliders, &mut dillo1);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(g1.rect)?;
        canvas.draw_rect(g2.rect)?;

        canvas.copy(&dillo_texture, None, dillo1.position)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}