use macroquad::prelude::Color;
use nalgebra::{vector, Vector2};
use std::num::NonZeroU16;

#[derive(Clone, Debug)]
pub struct MaterialVolume {
    pub capacity: Vector2<u32>,
    pub size: Vector2<u32>,
    pub volume: Vec<Option<Material>>,
    pub update_handler: UpdateHandler,
}

impl MaterialVolume {
    pub fn new(size: Vector2<u32>, update_handler: UpdateHandlerType) -> Self {
        let elements = (size.x * size.y * 4) as usize;

        Self {
            capacity: size,
            size,
            volume: (0..elements).map(|_| None).collect(),
            update_handler: update_handler.get_empty(),
        }
    }

    pub fn get_pixel_at(&self, position: Vector2<f32>) -> Option<Material> {
        self.get(self.get_index_of_pixel(position))
    }

    pub fn set_pixel_at(&mut self, position: Vector2<f32>, value: Option<Material>) -> Option<()> {
        self.set(self.get_index_of_pixel(position), value)
    }

    pub fn get_index_of_pixel(&self, position: Vector2<f32>) -> Vector2<u32> {
        to_vector_u32(position * 2.0) + offset_of(position)
    }

    pub fn get(&self, index: Vector2<u32>) -> Option<Material> {
        if in_bounds_of(self.size, index) {
            self.volume[self.index_1d(index)]
        } else {
            None
        }
    }

    pub fn set(&mut self, index: Vector2<u32>, value: Option<Material>) -> Option<()> {
        if in_bounds_of(self.size, index) {
            let index_1d = self.index_1d(index);
            self.volume[index_1d] = value;

            Some(())
        } else {
            None
        }
    }

    pub fn index_1d(&self, index: Vector2<u32>) -> usize {
        (index.x + index.y * self.capacity.x) as usize
    }
}

fn offset_of(position: Vector2<f32>) -> Vector2<u32> {
    let position = vector![position.x % 1.0, position.y % 1.0];

    vector![
        (position.y < position.x) as u32,
        (position.y > 1.0 - position.x) as u32,
    ]
}

fn to_vector_u32(input: Vector2<f32>) -> Vector2<u32> {
    vector![input.x as u32, input.y as u32]
}

fn in_bounds_of(lhs: Vector2<u32>, rhs: Vector2<u32>) -> bool {
    (0..lhs.x).contains(&rhs.x) && (0..lhs.y).contains(&rhs.y)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    pub base_color: [u8; 4],
    pub integrity: u16,
    pub max_integrity: NonZeroU16, // Used non-zero type for memory usage with Option<Material>
    pub collision_layers: u8,
    pub temputature: i8, // May be replaced in the future
}

impl Material {
    pub fn get_base_color(&self) -> Color {
        let mut channels = [0.0; 4];

        for i in 0..4 {
            channels[i] = self.base_color[i] as f32 / u8::MAX as f32;
        }

        Color::new(channels[0], channels[1], channels[2], channels[3])
    }
}

#[derive(Clone, Debug)]
pub enum UpdateHandler {
    Full(bool),
    Square(Option<(Vector2<u32>, Vector2<u32>)>),
    PointSet(Vec<(Vector2<u32>, Vector2<u32>)>),
}

#[derive(Clone, Copy, Debug)]
pub enum UpdateHandlerType {
    Full,
    Square,
    PointSet,
}

impl UpdateHandlerType {
    pub fn get_empty(self) -> UpdateHandler {
        match self {
            Self::Full => UpdateHandler::Full(false),
            Self::Square => UpdateHandler::Square(None),
            Self::PointSet => UpdateHandler::PointSet(Vec::new()),
        }
    }
}
