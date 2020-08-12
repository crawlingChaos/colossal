// Helper code so main.rs can keep as much of the structure of the original Fortran
// as possible.

use super::{data::DATA, Label};
use std::{
    fmt, io,
    process::exit,
    str::{Chars, Lines},
    time::{SystemTime, UNIX_EPOCH},
};

pub const BLANK: Char = Char { c: 32 };

// Integer to character
pub fn achar(i: i32) -> Char {
    Char { c: i as u8 }
}

// Conditional goto
pub fn goto(labels: &[Label], variable: i32) -> Option<Label> {
    let index = (variable as usize).wrapping_sub(1);
    labels.get(index).map(|l| *l)
}

// Character to integer
pub fn iachar(ch: Char) -> i32 {
    ch.c as i32
}

// Pause execution
pub fn pause(msg: &str) {
    println!("PAUSE: {}", msg);
    println!("To resume execution, type: go");
    println!("Any other input will terminate the program.");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(3) if input == "go\n" => println!("Execution resumed after PAUSE."),
        _ => exit(0),
    }
}

// Stop execution
pub fn stop() {
    exit(0);
}

// Fortran arrays can begin at any index (including negative), but default to one.
// All the arrays in the original source use the default of one.

// One-based array
#[derive(Debug)]
pub struct Array<T: Copy + Default> {
    array: Vec<T>,
}

impl<T: Copy + Default> Array<T> {
    pub fn new(size: usize) -> Self {
        Self {
            array: vec![T::default(); size],
        }
    }

    pub fn get(&self, index: i32) -> &T {
        &self.array[index as usize - 1]
    }

    pub fn get_mut(&mut self, index: i32) -> &mut T {
        &mut self.array[index as usize - 1]
    }

    pub fn set(&mut self, index: i32, value: T) {
        self.array[index as usize - 1] = value;
    }

    pub fn copy_from_slice(&mut self, slice: &[T]) {
        for (i, value) in slice.iter().enumerate() {
            self.array[i] = *value;
        }
    }
}

// One-based 2D array
#[derive(Debug)]
pub struct Array2D<T: Copy + Default> {
    array: Vec<Vec<T>>,
}

impl<T: Copy + Default> Array2D<T> {
    pub fn new(size_1: usize, size_2: usize) -> Self {
        Self {
            array: vec![vec![T::default(); size_2]; size_1],
        }
    }

    pub fn get(&self, index_1: i32, index_2: i32) -> &T {
        &self.array[index_1 as usize - 1][index_2 as usize - 1]
    }

    pub fn get_mut(&mut self, index_1: i32, index_2: i32) -> &mut T {
        &mut self.array[index_1 as usize - 1][index_2 as usize - 1]
    }

    pub fn set(&mut self, index_1: i32, index_2: i32, value: T) {
        self.array[index_1 as usize - 1][index_2 as usize - 1] = value;
    }
}

// Character
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Char {
    c: u8,
}

impl Char {
    pub fn new(c: u8) -> Self {
        Char { c }
    }
}

impl Default for Char {
    fn default() -> Self {
        BLANK
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.c as char)
    }
}

// Internal file
#[derive(Debug)]
pub struct File<'a> {
    lines: Lines<'a>,
}

impl<'a> File<'_> {
    pub fn open() -> File<'a> {
        File {
            lines: DATA.lines(),
        }
    }

    pub fn read(&mut self) -> Line {
        match self.lines.next() {
            Some(line) => Line {
                chars: line.chars(),
            },
            None => Line { chars: "".chars() },
        }
    }
}

// Keyboard
#[derive(Debug)]
pub struct Keyboard {
    input: String,
}

impl Keyboard {
    pub fn open() -> Keyboard {
        Keyboard {
            input: String::new(),
        }
    }

    pub fn read<'a>(&'a mut self) -> Line<'a> {
        self.input.truncate(0);
        io::stdin().read_line(&mut self.input).unwrap();
        Line {
            chars: self.input.chars(),
        }
    }
}

// Line of text
#[derive(Debug)]
pub struct Line<'a> {
    chars: Chars<'a>,
}

impl<'a> Line<'_> {
    pub fn read_char(&mut self, var: &mut Char) {
        if let Some(c) = self.chars.next() {
            var.c = c as u8;
        }
    }

    pub fn read_i7(&mut self, var: &mut i32) {
        let mut value = String::with_capacity(7);
        for _ in 0..7 {
            if let Some(c) = self.chars.next() {
                value.push(c);
            }
        }
        *var = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }

    pub fn read_word(&mut self, var: &mut Word) {
        for i in 1..=5 {
            if let Some(c) = self.chars.next() {
                if c != '\n' {
                    var.get_mut(i).c = c as u8;
                }
            }
        }
    }
}

// A crude Blum Blum Shub psuedo-random generator
#[derive(Debug)]
pub struct Rand {
    seed: u32,
}

impl Rand {
    const P: u32 = 2879;
    const Q: u32 = 2903;
    pub fn new() -> Rand {
        Rand {
            seed: Self::reseed(),
        }
    }

    // Generate a number between 0.0 and 1.0
    pub fn gen(&mut self) -> f32 {
        let n = (Self::P * Self::Q) as u64;
        loop {
            let mut num = 0;
            for _ in 1..8 {
                let seed_squared = (self.seed as u64).pow(2);
                self.seed = (seed_squared % n) as u32;
                num <<= 1;
                num += self.seed & 1;
            }
            if num < 100 {
                return num as f32 / 100.0;
            }
        }
    }

    fn reseed() -> u32 {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();
        duration.subsec_nanos() % (Self::P * Self::Q - 2) + 2
    }
}

// Five character word with one-based indexing
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Word {
    w: [Char; 5],
}

impl Word {
    pub fn new() -> Self {
        Word {
            w: [Char::default(); 5],
        }
    }

    pub fn eq(&self, s: &str) -> bool {
        let mut word = Word::new();
        for (i, c) in s.chars().enumerate() {
            if i > 4 {
                return false;
            }
            word.set(i + 1, Char { c: c as u8 });
        }
        *self == word
    }

    pub fn get(&self, index: usize) -> &Char {
        &self.w[index - 1]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Char {
        &mut self.w[index - 1]
    }

    pub fn set(&mut self, index: usize, value: Char) {
        self.w[index - 1] = value;
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.w.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
