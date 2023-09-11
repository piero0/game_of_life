use std::cmp::{min, max};

use rand::Rng;
// trait Algorithm {
    
// }

pub struct GameLogic {
    line_size: usize,
    pub init_thresh: f64
}

impl GameLogic {
    pub fn new(line_size: usize) -> Self {
        GameLogic { line_size, init_thresh: 0.5 }
    }

    pub fn init_cells(&self, cells: &mut [bool]) {
        let mut rng = rand::thread_rng();
        for c in cells.iter_mut() {
            *c = if rng.gen::<f64>() >= self.init_thresh { true } else { false };
        }
    }
    
    fn get_neighbors(&self, i: usize, cells: &[bool]) -> u8 {
        let current_line = i / self.line_size as usize;
        let delta_idx = [
            (-(self.line_size as i32 + 1), -1),
            (-(self.line_size as i32), -1),
            (-(self.line_size as i32 - 1), -1),
            (-1, 0),
            (1, 0),
            (self.line_size as i32 - 1, 1),
            (self.line_size as i32, 1),
            (self.line_size as i32 + 1, 1),
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
                    && x / self.line_size as i32 == current_line as i32 + *y
                    && cells[x as usize] == true
            })
            .count();
    
        // println!("w {:?}\nw1{:?}\nw2 {w2}",w,w1);
        w as u8
    }

    pub fn apply_rules(&self, cells: &[bool]) -> Vec<bool> {
        let mut out: Vec<bool> = Vec::new();

        let mut part_sum = Vec::<u8>::new();

        part_sum.extend(vec![0; self.line_size]);

        for chunk in cells.chunks(self.line_size) {
            let v = self.line_sum(chunk);
            part_sum.extend_from_slice(&v);
        }

        part_sum.extend(vec![0; self.line_size]);

        // println!("part {:?}", part_sum);

        // assert!(part_sum.len() == self.line_size*self.line_size+2*self.line_size);

        for i in 1..(part_sum.len()/self.line_size)-1 {
            for j in 0..self.line_size {
                let idx: usize = i*self.line_size+j;
                let s1 = part_sum[idx-self.line_size];
                let s2 = self.current_line_sum(cells, idx, j);
                let s3 = part_sum[idx+self.line_size];

                let s = s1+s2+s3;

                let new_val = match s {
                    0..=1 => false,
                    2 => cells[idx-self.line_size],
                    3 => true,
                    4.. => false,
                };

                out.push(new_val);
            }
        }
        out
    }

    fn current_line_sum(&self, cells: &[bool], base_idx: usize, pos: usize) -> u8 {
        let ret: u8;
        let cbase = base_idx - self.line_size;
        if pos == 0 {
            ret = cells[cbase+1]  as u8
        } else if pos == self.line_size-1 {
            ret = cells[cbase-1] as u8
        } else {
            ret = cells[cbase-1] as u8 + cells[cbase+1] as u8
        }
        ret
    }

    fn line_sum(&self, line: &[bool]) -> Vec<u8> {
        let mut out = Vec::<u8>::new();
        out.push(line[0] as u8 + line[1] as u8);
        for i in 1..line.len()-1 {
            out.push(line[i-1] as u8 + line[i] as u8 + line[i+1] as u8);
        }
        out.push(line[line.len()-2] as u8 + line[line.len()-1] as u8);
        // assert!(out.len() == self.line_size);
        out
    }
}
