use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use std::io::{stdout, Write};
use std::vec::Vec;
use std::{thread, time};
use rand_distr::Distribution;

struct Ring<T> {
    vec: Vec<T>,
    ptr: usize
}

impl<T: Clone> Ring<T> {
    fn new(len: usize, def: T) -> Self {
        Ring {
            vec: vec![def; len],
            ptr: 0
        }
    }
    fn push(&mut self, input: T) {
        self.vec[self.ptr] = input;
        self.ptr += 1;
        if self.ptr >= self.vec.len() {
            self.ptr = 0;
        }
    }
}

struct RingIter<'a, T> {
    ptr: usize,
    ring: &'a Ring<T>
}

impl<'a, T: Copy> Iterator for RingIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        if self.ptr == self.ring.vec.len() {
            self.ptr = 0;
        }
        if self.ptr == self.ring.ptr {
            return Option::None;
        }
        Option::Some(self.ring.vec[self.ptr])
    }
}

impl<T> Ring<T> {
    fn iter(&self) -> RingIter<T> {
        RingIter { ptr : self.ptr, ring : self }
    }
}


fn draw_graph(ring: &Ring<f64>, height: i16, std: &mut std::io::Stdout) {
    std.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let min_val = ring.iter().fold(f64::MAX, f64::min);
    let max_val = ring.iter().fold(f64::MIN, f64::max);
    let range = max_val - min_val;
    for (x, val) in ring.iter().enumerate() {
        let val = val - min_val;
        let y = val * (height as f64) / range;
        std
            .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
            .queue(style::PrintStyledContent("█".magenta())).unwrap();
    }
    stdout().flush().unwrap();
}

fn main() {
    let mut stdout = stdout();
    let (width, height) = {
        let (w, h) = term_size::dimensions().unwrap();
        (w as i16, h as i16)
    };
    let mut ring: Ring<f64> = Ring::new(width as usize, 0.0);
    let mut value: f64 = 0.0;
    let normal = rand_distr::Normal::new(0.0, 1.0).unwrap();
    loop {
        thread::sleep(time::Duration::from_millis(300));

        value += normal.sample(&mut rand::thread_rng());
        ring.push(value);

        draw_graph(&ring, height, &mut stdout);
    }
}
