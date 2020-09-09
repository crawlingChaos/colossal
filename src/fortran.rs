// Helper code so main.rs can keep as much of the structure of the original Fortran
// as possible.

use super::{data::DATA, Label};
use std::{
    io, ops,
    process::exit,
    str::{Chars, Lines},
    time::{SystemTime, UNIX_EPOCH},
};

// Conditional goto
pub fn goto(labels: &[Label], index: i64) -> Option<Label> {
    if index > 0 {
        let index = index as usize - 1;
        labels.get(index).map(|l| *l)
    } else {
        None
    }
}

// Pause execution
pub fn pause(msg: &str) {
    println!("{}", msg);
    loop {
        println!();
        println!("TYPE G TO CONTINUE,X TO EXIT.");
        print!(" ");
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if let Some(reply) = input.lines().next() {
                if reply == "G" {
                    break;
                } else if reply == "X" {
                    stop();
                }
            }
        }
    }
}

// Stop execution
pub fn stop() {
    exit(0);
}

// Type formatted output
pub fn type_format(format: &str, end_slash: EndSlash) {
    let mut chars = format.chars();
    let first = chars.next();
    match first {
        // Fortran format - skip a line
        Some(' ') => {
            println!();
        }
        // Fortran format - skip 1/6 page (Most implementations seem to just double space.)
        Some('/') => {
            println!();
            println!();
        }
        // Fortran format - other formats unused and unsupported
        _ => panic!(),
    }
    if chars.next() != None {
        println!("{}", &format[1..]);
    }
    if let EndSlash::Yes = end_slash {
        println!();
    }
}

// Used to indicate presence of trailing '/' format character
#[derive(Clone, Copy, Debug)]
pub enum EndSlash {
    Yes,
    No,
}

// Fortran arrays can begin at any index (including negative), but default to one.
// All the arrays in the original source use the default of one.

// One-based array
#[derive(Debug)]
pub struct Array {
    array: Vec<i64>,
}

impl Array {
    pub fn new(size: usize) -> Self {
        Self {
            array: vec![0; size],
        }
    }

    pub fn data(&mut self, slice: &[i64]) {
        self.array[..slice.len()].copy_from_slice(slice);
    }
}

impl ops::Index<i64> for Array {
    type Output = i64;

    fn index(&self, index: i64) -> &Self::Output {
        &self.array[index as usize - 1]
    }
}

impl ops::IndexMut<i64> for Array {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        &mut self.array[index as usize - 1]
    }
}

// One-based 2D array
#[derive(Debug)]
pub struct Array2D {
    array: Vec<Vec<i64>>,
}

impl Array2D {
    pub fn new(size1: usize, size2: usize) -> Self {
        Self {
            array: vec![vec![0; size2]; size1],
        }
    }
}

impl ops::Index<(i64, i64)> for Array2D {
    type Output = i64;

    fn index(&self, (index1, index2): (i64, i64)) -> &Self::Output {
        &self.array[index1 as usize - 1][index2 as usize - 1]
    }
}

impl ops::IndexMut<(i64, i64)> for Array2D {
    fn index_mut(&mut self, (index1, index2): (i64, i64)) -> &mut Self::Output {
        &mut self.array[index1 as usize - 1][index2 as usize - 1]
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
        loop {
            match self.lines.next() {
                Some(line) if line.len() > 0 => {
                    return Line {
                        chars: line.chars(),
                    }
                }
                Some(_) => continue,
                None => panic!(),
            }
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
        if let Ok(_) = io::stdin().read_line(&mut self.input) {
            if let Some(reply) = self.input.lines().next() {
                self.input = reply.to_string();
            }
        }
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
    pub fn read_g(&mut self, var: &mut i64) {
        let mut value = String::new();
        loop {
            match self.chars.next() {
                Some(c) if c != '\t' && c != '\n' => value.push(c as char),
                _ => break,
            }
        }
        *var = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    pub fn read_a5(&mut self, var: &mut i64) {
        let mut word = String::with_capacity(5);
        for _ in 0..5 {
            match self.chars.next() {
                Some(c) if c != '\t' && c != '\n' => word.push(c),
                _ => break,
            }
        }
        *var = i64::from_word(&word)
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
            if num > 0 && num < 100 {
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

pub trait Word {
    fn from_word(word: &str) -> i64;
    fn equals(&self, rhs: &str) -> bool;
    fn to_word(&self) -> String;
}

impl Word for i64 {
    fn from_word(word: &str) -> i64 {
        let mut result = 0;
        let mut chars = word.chars();
        for i in 0..5 {
            result |= i64::from(match chars.next() {
                Some(c) => (c as u8) & 0b_0111_1111,
                None => 0b_0010_0000,
            }) << 1 + 7 * (4 - i);
        }
        if result & 0b_1000000_0000000_0000000_0000000_0000000_0 != 0 {
            result |= -0b_1_0000000_0000000_0000000_0000000_0000000_0;
        }
        result
    }

    fn equals(&self, rhs: &str) -> bool {
        let rhs = i64::from_word(rhs);
        *self == rhs
    }

    fn to_word(&self) -> String {
        let mut output = String::with_capacity(5);
        for i in 0..5 {
            let c = ((self >> 1 + 7 * (4 - i)) as u8 & 0b_0111_1111) as char;
            output.push(c);
        }
        output
    }
}
