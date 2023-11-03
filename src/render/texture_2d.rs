use std::marker::PhantomData;
use crate::{Vec2i, Vec2s};

pub struct Texture2D<T: TextureDimension> {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    phantom: PhantomData<T>
}

pub trait TextureDimension: Clone + Copy {
    fn get_dimension() -> usize;
    fn get(buffer: &Vec<u8>, index: usize) -> Self;
    fn set(buffer: &mut Vec<u8>, index: usize, value: Self);
}
type One = u8;
type Two = (u8, u8);
type Three = (u8, u8, u8);
type Four = (u8, u8, u8, u8);
impl TextureDimension for One {
    fn get_dimension() -> usize {
        1
    }

    fn get(buffer: &Vec<u8>, index: usize) -> Self {
        buffer[index]
    }

    fn set(buffer: &mut Vec<u8>, index: usize, value: Self) {
        buffer[index] = value
    }
}

impl TextureDimension for Two {
    fn get_dimension() -> usize {
        2
    }

    fn get(buffer: &Vec<u8>, index: usize) -> Self {
        (buffer[index], buffer[index + 1])
    }

    fn set(buffer: &mut Vec<u8>, index: usize, value: Self) {
        buffer[index] = value.0;
        buffer[index + 1] = value.1;
    }
}

impl TextureDimension for Three {
    fn get_dimension() -> usize {
        3
    }

    fn get(buffer: &Vec<u8>, index: usize) -> Self {
        (buffer[index], buffer[index + 1], buffer[index + 2])
    }

    fn set(buffer: &mut Vec<u8>, index: usize, value: Self) {
        buffer[index] = value.0;
        buffer[index + 1] = value.1;
        buffer[index + 2] = value.2;
    }
}

impl TextureDimension for Four {
    fn get_dimension() -> usize {
        4
    }

    fn get(buffer: &Vec<u8>, index: usize) -> Self {
        (buffer[index], buffer[index + 1], buffer[index + 2], buffer[index + 3])
    }

    fn set(buffer: &mut Vec<u8>, index: usize, value: Self) {
        buffer[index] = value.0;
        buffer[index + 1] = value.1;
        buffer[index + 2] = value.2;
        buffer[index + 3] = value.3;
    }
}


impl<T> Texture2D<T> where T: TextureDimension + Clone + Copy {
    pub fn new(width: usize, height: usize) -> Texture2D<T> {
        Texture2D {
            width,
            height,
            buffer: vec![0; width * height * T::get_dimension()],
            phantom: PhantomData,
        }
    }

    pub fn fill(&mut self, value: T) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(Vec2s::new(x, y), value);
            }
        }
    }
    pub fn get(&self, pos: Vec2s) -> T {
        let index = pos.y * self.width + pos.x;
        T::get(&self.buffer, index)
    }
    pub fn set(&mut self, pos: Vec2s, value: T) {
        let index = pos.y * self.width + pos.x;
        T::set(&mut self.buffer, index, value)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }
}

pub enum Texture2DBuffer{
    One(Texture2D<One>),
    Two(Texture2D<Two>),
    Three(Texture2D<Three>),
    Four(Texture2D<Four>),
}
