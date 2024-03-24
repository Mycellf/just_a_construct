use macroquad::prelude::*;
use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct Object {
    display: DisplayState<()>,
}

#[derive(Clone, Debug)]
pub struct DisplayState<T: DisplayData> {
    image: Image,
    texture: Texture2D,
    data: Vec<T>,
}

impl<T: DisplayData> DisplayState<T> {
    pub fn from_size(size: Vector2<u16>) -> Self {
        Self::from_color(BLANK, size)
    }

    pub fn from_color(color: Color, size: Vector2<u16>) -> Self {
        Self::from_image(Image::gen_image_color(size.x, size.y, color))
    }

    pub fn from_image(image: Image) -> Self {
        let texture = Texture2D::from_image(&image);
        let data = (0..image.width() * image.height())
            .map(|_| T::display_default())
            .collect();
        Self {
            image,
            texture,
            data,
        }
    }

    pub fn resize(&mut self, size: Vector2<u16>, shift: Vector2<u16>) {}
}

pub trait DisplayData {
    fn display_color(&self) -> Color;
    fn display_default() -> Self;
}

impl DisplayData for () {
    fn display_color(&self) -> Color {
        BLANK
    }

    fn display_default() -> Self {}
}
