use rand::Rng;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Key, Style},
};
use std::time::Instant;

pub mod drawer;
// mod gamelogic;
pub mod stats;

const PIXEL_SIZE: f32 = 1.0;
const PLANE_SIZE: usize = 1000;
const WINDOW_SIZE: u32 = PLANE_SIZE as u32 * PIXEL_SIZE as u32;

fn init_cells(cells: &mut [bool]) {
    let mut rng = rand::thread_rng();
    for c in cells.iter_mut() {
        *c = if rng.gen::<f64>() >= 0.5 { true } else { false };
    }
}

fn get_neighbors(i: usize, cells: &[bool]) -> u8 {
    let current_line = i / PLANE_SIZE;
    let delta_idx = [
        (-(PLANE_SIZE as i32 + 1), -1),
        (-(PLANE_SIZE as i32), -1),
        (-(PLANE_SIZE as i32 - 1), -1),
        (-1, 0),
        (1, 0),
        (PLANE_SIZE as i32 - 1, 1),
        (PLANE_SIZE as i32, 1),
        (PLANE_SIZE as i32 + 1, 1),
    ];

    let w = delta_idx
        .iter()
        .map(|(x, y)| (i as i32 + x, y)) //delta to array idx
        .filter(|&(x, y)| {
            //prune those outside the array
            //and when index is not on a "correct" line
            //and we only care about living cells
            x >= 0
                && x < cells.len() as i32
                && x / PLANE_SIZE as i32 == current_line as i32 + *y
                && cells[x as usize] == true
        })
        .count();

    // println!("w {:?}\nw1{:?}\nw2 {w2}",w,w1);
    w as u8
}

fn apply_rules(cells: &[bool]) -> Vec<bool> {
    let mut out: Vec<bool> = Vec::new();

    for (i, c) in cells.iter().enumerate() {
        let neighbors = get_neighbors(i, cells);

        let new_c = match c {
            false => match neighbors {
                3 => true,
                _ => false,
            },
            true => match neighbors {
                0..=1 => false,
                2..=3 => true,
                4.. => false,
            },
        };
        out.push(new_c);
    }
    out
}

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_SIZE, WINDOW_SIZE),
        "First Rust App",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    
    let init = [false; (PLANE_SIZE * PLANE_SIZE) as usize];
    let mut cells = Vec::from(init);
    init_cells(&mut cells);
    let now = Instant::now();

    let mut stats = stats::Stat::new();

    let drawer = drawer::Drawer::new(PIXEL_SIZE, PLANE_SIZE);

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => window.close(),
                Event::KeyReleased { code, .. } => match code {
                    Key::R => init_cells(&mut cells),
                    Key::Escape => window.close(),
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

        let new_cells = apply_rules(&cells);
        cells = new_cells;

        let s3 = now.elapsed();

        stats.add_times(s1, s2, s3);
    }
}
