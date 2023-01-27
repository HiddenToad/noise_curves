
use nannou::noise::{NoiseFn, OpenSimplex};
use nannou::prelude::*;


const SCREEN_SIZE: u32 = 1600;
const SCREEN_THIRD: f32 = SCREEN_SIZE as f32 / 3.;

const NUM_RINGS: u32 = 40;
const OFFSET_MULTIPLIER: f64 = 0.03;
const SPEED_MULTIPLIER: f64 = 0.25;
const POINTS_PER_LINE: i32 = 250;

const NUM_COLORS: usize = 2;
const COLORS: [Srgb<u8>; NUM_COLORS] = [REBECCAPURPLE, BLUE];

const fn color_for(i: u32) -> Srgb<u8>{
    COLORS[i as usize % NUM_COLORS]
}

fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .build()
        .unwrap();
    Model { _window }
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    let noisefn = OpenSimplex::new();

    for i in 0..NUM_RINGS {
        let mut points = [pt2(0., 0.); POINTS_PER_LINE as usize];
        let rad = SCREEN_THIRD / NUM_RINGS as f32 * (NUM_RINGS - i) as f32;
        for j in 0..POINTS_PER_LINE {
            let spatial_x = (j as f32 / 200. * TAU).cos();
            let spatial_y = (j as f32 / 200. * TAU).sin();
            let delta = (app.elapsed_frames() % 360) as f64 * PI_F64 / 180.;
            let temporal_x = delta.cos() * SPEED_MULTIPLIER;
            let temporal_y = delta.sin() * SPEED_MULTIPLIER;

            let offset = noisefn.get([
                spatial_x as f64, 
                spatial_y as f64 + i as f64 * OFFSET_MULTIPLIER, 
                temporal_x, 
                temporal_y
            ]) as f32 * 2.;

            points[j as usize] = pt2(
                spatial_x * (offset * rad + rad),
                spatial_y * (offset * rad + rad),
            );
        }

        let completion = i as f32 / NUM_RINGS as f32;
        draw.path()
            .stroke()
            .points(points)
            .color(Srgba::<u8> {
                alpha: map_range(completion, 0., 1., 0., 255.) as u8,
                color: color_for(i),
            })
            .x_y(0., 0.);
    }

    draw.to_frame(app, &frame).unwrap();
}
