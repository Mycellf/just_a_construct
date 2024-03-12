use macroquad::prelude::*;

pub mod object;
pub mod physics_world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Madness 404".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let material = make_tri_pixel_material();
    let texture = load_texture("assets/wall.png").await.unwrap();

    let mut camera = Camera2D {
        zoom: Vec2::splat(1.0 / 64.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        update_camera(&mut camera);

        gl_use_material(&material);

        draw_texture(&texture, 0.0, 0.0, WHITE);

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