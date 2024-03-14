use macroquad::prelude::*;
use nalgebra::{point, vector, Isometry2};

pub mod material;
pub mod object;
pub mod physics_world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Just a Construct".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let polygon =
        object::PolygonCollider::new(vec![point![0, 0], point![0, 16], point![16, 16]], None);

    let mut camera = Camera2D {
        zoom: Vec2::splat(1.0 / 64.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        update_camera(&mut camera);

        polygon.draw_debug(
            Isometry2::new(vector![0.0, 0.0], std::f32::consts::FRAC_PI_6),
            0.15,
            MAGENTA,
        );

        next_frame().await;
    }
}

pub fn make_tri_pixel_material() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/vertex.glsl"),
            fragment: include_str!("shaders/fragment.glsl"),
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(miniquad::BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                    miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .expect("Shader code should be valid")
}

fn update_camera(camera: &mut Camera2D) {
    camera.zoom.x = camera.zoom.y / screen_width() * screen_height();
    set_camera(camera);
}
