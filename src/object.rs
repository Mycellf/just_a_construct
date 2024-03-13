use macroquad::prelude::*;
use nalgebra::{vector, Vector2};
use std::num::NonZeroU16;

#[derive(Clone, Debug)]
pub struct MaterialVolume {
    pub size: Vector2<u32>,
    pub volume: Vec<Option<Material>>,
    pub update_handler: UpdateHandler,
    pub image: Image,
    pub texture: Texture2D,
}

impl MaterialVolume {
    pub fn new(size: Vector2<u32>) -> Self {
        let image_size = size * 2;
        let elements = (image_size.x * image_size.y) as usize;

        let image = Image::gen_image_color(image_size.x as u16, image_size.y as u16, BLANK);
        let texture = Texture2D::from_image(&image);

        Self {
            size,
            volume: (0..elements).map(|_| None).collect(),
            update_handler: UpdateHandler::from_elements(elements),
            image,
            texture,
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
        if in_bounds_of(self.size * 2, index) {
            self.volume[self.index_1d(index)]
        } else {
            None
        }
    }

    pub fn set(&mut self, index: Vector2<u32>, value: Option<Material>) -> Option<()> {
        if in_bounds_of(self.size * 2, index) {
            let index_1d = self.index_1d(index);
            println!("{index_1d}");
            self.image.get_image_data_mut()[index_1d] = match value {
                Some(material) => material.base_color,
                None => [0, 0, 0, 0],
            };
            self.volume[index_1d] = value;
            self.update_handler.register_update(index);

            Some(())
        } else {
            None
        }
    }

    pub fn index_1d(&self, index: Vector2<u32>) -> usize {
        (index.x + index.y * self.size.x * 2) as usize
    }

    pub fn update_texture(&mut self) {
        match self.update_handler {
            UpdateHandler::Full(true) => {
                self.texture.update(&self.image);
            }
            UpdateHandler::Square(Some((lower, upper))) => {
                let update_size = upper - lower + vector![1, 1];
                let sub_image = self.image.sub_image(Rect::new(
                    lower.x as f32,
                    lower.y as f32,
                    update_size.x as f32,
                    update_size.y as f32,
                ));

                self.texture.update_part(
                    &sub_image,
                    lower.x as i32,
                    lower.y as i32,
                    update_size.x as i32,
                    update_size.y as i32,
                );
            }
            _ => (),
        }

        self.update_handler.clear_updates();
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

#[derive(Clone, Copy, Debug)]
pub enum UpdateHandler {
    Full(bool),
    Square(Option<(Vector2<u32>, Vector2<u32>)>),
}

impl UpdateHandler {
    pub const MAX_ELEMENTS_FULL: usize = 128;

    pub fn from_elements(elements: usize) -> Self {
        if elements > Self::MAX_ELEMENTS_FULL {
            Self::Square(None)
        } else {
            Self::Full(false)
        }
    }

    /// Will void existing updates
    pub fn update_from_elements(&mut self, elements: usize) {
        let needs_update = (elements > Self::MAX_ELEMENTS_FULL)
            ^ match self {
                Self::Full(_) => false,
                Self::Square(_) => true,
            };

        if needs_update {
            *self = Self::from_elements(elements);
        }
    }

    pub fn register_update(&mut self, update: Vector2<u32>) {
        match self {
            Self::Full(update) => {
                *update = true;
            }
            Self::Square(shape) => match shape {
                Some((lower, upper)) => {
                    if update.x < lower.x {
                        lower.x = update.x;
                    } else if update.x > upper.x {
                        upper.x = update.x;
                    }

                    if update.y < lower.y {
                        lower.y = update.y;
                    } else if update.y > upper.y {
                        upper.y = update.y;
                    }
                }
                None => {
                    *shape = Some((update, update));
                }
            },
        }
    }

    pub fn clear_updates(&mut self) {
        match self {
            Self::Full(update) => {
                *update = false;
            }
            Self::Square(shape) => {
                *shape = None;
            }
        }
    }
}
