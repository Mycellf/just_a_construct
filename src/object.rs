use macroquad::prelude::*;
use nalgebra::{point, vector, Isometry2, Point2, Vector2};

pub struct PolygonCollider {
    pub points: Vec<Point2<i32>>,
    pub indices: Vec<[usize; 2]>,
}

impl PolygonCollider {
    pub fn new(points: Vec<Point2<i32>>, indices: Option<Vec<[usize; 2]>>) -> Self {
        let indices = match indices {
            Some(indices) => indices,
            None => (0..points.len())
                .map(|i| [i, (i + 1) % points.len()])
                .collect(),
        };

        Self { points, indices }
    }

    /// Will not work well with extremely large values in points
    pub fn draw_debug(&self, position: Isometry2<f32>, bold: f32, color: Color) {
        let points: Vec<_> = self
            .get_points_float(vector![0, 0])
            .into_iter()
            .map(|point| position.transform_point(&point))
            .collect();

        gl_use_default_material();

        for index in &*self.indices {
            let start = points[index[0]];
            let end = points[index[1]];

            draw_line(start.x, start.y, end.x, end.y, bold, color);
        }
    }

    pub fn get_points_float(&self, offset: Vector2<i32>) -> Vec<Point2<f32>> {
        (&*self.points)
            .into_iter()
            .map(|point| to_point_f32((point + offset) / 2))
            .collect()
    }
}

fn to_point_f32(value: Point2<i32>) -> Point2<f32> {
    point![value.x as f32, value.y as f32]
}
