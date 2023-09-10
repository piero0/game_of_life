use std::{time::Duration, fmt, default::Default};

struct Avg {
    min: u32,
    max: u32,
    avg: u32,
    sum: u64,
    cnt: u64,
}

impl Default for Avg {
    fn default() -> Self {
        Self { 
            min: u32::MAX,
            max: 0,
            avg: 0,
            sum: 0,
            cnt: 0
        }
    }
}

impl Avg {
    fn add(&mut self, value: u32) {
        if value < self.min && value > 0 {
            self.min = value;
        }

        if value > self.max {
            self.max = value;
        }

        self.sum += value as u64;
        self.cnt += 1;

        self.avg = (self.sum/self.cnt) as u32;
    }
}

impl fmt::Display for Avg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "avg: {} min: {} max: {}", self.avg, self.min, self.max)
    }
}

#[derive(Default)]
pub struct Stat {
    draw: Avg,
    logic: Avg,
    frame: Avg,
    frame_num: u32,
}

impl Stat {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_times(&mut self, frame_start: Duration, draw: Duration, logic: Duration) {
        let draw_time = (draw-frame_start).as_micros();
        let logic_time = (logic-draw).as_micros();
        let frame_time = (logic-frame_start).as_micros();

        self.draw.add(draw_time as u32);
        self.logic.add(logic_time as u32);
        self.frame.add(frame_time as u32);

        self.frame_num += 1;

        if self.frame_num % 100 == 0 {
            println!("=== {} ===", self.frame_num);
            println!("draw {}", self.draw);
            println!("logic {}", self.logic);
            println!("frame {}", self.frame);
        }
    }
}
