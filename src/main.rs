use gamelogic::GameLogic;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Time,
    window::{ContextSettings, Event, Key, Style},
};
use std::time::Instant;

mod drawer;
mod gamelogic;
mod stats;

const PIXEL_SIZE: f32 = 1.0;
const PLANE_SIZE: usize = 1000;
const WINDOW_SIZE: u32 = PLANE_SIZE as u32 * PIXEL_SIZE as u32;

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_SIZE, WINDOW_SIZE),
        "First Rust App",
        Style::DEFAULT,
        &ContextSettings::default(),
    );

    let init = [false; (PLANE_SIZE * PLANE_SIZE) as usize];
    let mut cells = Vec::from(init);
    let mut gl = GameLogic::new(PLANE_SIZE);
    gl.init_cells(&mut cells);
    // println!("init {:?}", cells);
    let now = Instant::now();

    let mut stats = stats::Stat::new();

    let drawer = drawer::Drawer::new(PIXEL_SIZE, PLANE_SIZE);

    // let mut cnt = 0;
    let mut sleep_time = Time::milliseconds(0);

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => window.close(),
                Event::KeyReleased { code, .. } => match code {
                    Key::R => gl.init_cells(&mut cells),
                    Key::Escape => window.close(),
                    Key::Add => {
                        gl.init_thresh += 0.1;
                        println!("Thresh {}", gl.init_thresh);
                    }
                    Key::Subtract => {
                        gl.init_thresh -= 0.1;
                        println!("Thresh {}", gl.init_thresh);
                    }
                    Key::Num1 => {
                        sleep_time -= Time::milliseconds(100);
                        println!("Sleep {}", sleep_time.as_milliseconds());
                    }
                    Key::Num2 => {
                        sleep_time += Time::milliseconds(100);
                        println!("Sleep {}", sleep_time.as_milliseconds());
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let s1 = now.elapsed(); //TODO add this to stats
                                // maybe stats.elapsed() or stats.count()
                                // and then at the end
                                // stats.collect()/calculate()/summary()

        {
            window.clear(Color::BLACK);
            // cells_to_pixels(&cells, &mut window);
            drawer.vertex_pixels(&cells, &mut window);
            window.display();
        }

        let s2 = now.elapsed();

        let new_cells = gl.apply_rules(&cells);
        cells = new_cells;
        // println!("{:?}", cells);

        let s3 = now.elapsed();

        stats.add_times(s1, s2, s3);
        // cnt += 1;
        // if cnt == 10 {
        //     window.close();
        // }
        sfml::system::sleep(sleep_time);
    }
}
