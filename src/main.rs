use nannou::noise::{NoiseFn, OpenSimplex};
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

const SCREEN_SIZE: u32 = 1600;
const SCREEN_THIRD: f32 = SCREEN_SIZE as f32 / 3.;

const NUM_RINGS: u32 = 40;
const OFFSET_MULTIPLIER: f64 = 0.03;
const SPEED_MULTIPLIER: f64 = 0.25;
const POINTS_PER_LINE: i32 = 250;

const NUM_COLORS: usize = 2;
const COLORS: [Srgb<u8>; NUM_COLORS] = [REBECCAPURPLE, BLUE];

const NUM_TWEEN_FRAMES: u32 = 50;

const fn color_for(i: u32) -> Srgb<u8> {
    COLORS[i as usize % NUM_COLORS]
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

#[derive(PartialEq, Debug)]
enum Mode {
    Circles,
    Noise,
}

struct Tween {
    frames: u32,
    mode_to: Mode,
}

struct Model {
    _window: window::Id,
    tween: Tween,
    lines: Vec<[Vec2; POINTS_PER_LINE as usize]>,
    completion: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .event(event)
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .build()
        .unwrap();
    Model {
        _window,
        tween: Tween {
            frames: 0,
            mode_to: Mode::Circles,
        },
        lines: vec![],
        completion: 0.,
    }
}

fn event(_: &App, model: &mut Model, event: WindowEvent) {
    if model.tween.frames == 0 {
        match event {
            KeyPressed(key) => match key {
                VirtualKeyCode::Key1 => {
                    if model.tween.mode_to != Mode::Circles {
                        model.tween.frames = NUM_TWEEN_FRAMES;
                        model.tween.mode_to = Mode::Circles;
                    }
                }
                VirtualKeyCode::Key2 => {
                    if model.tween.mode_to != Mode::Noise {
                        model.tween.frames = NUM_TWEEN_FRAMES;
                        model.tween.mode_to = Mode::Noise;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    let noisefn = OpenSimplex::new();
    model.lines.clear();
    for i in 0..NUM_RINGS {
        let mut points = [pt2(0., 0.); POINTS_PER_LINE as usize];
        let rad = SCREEN_THIRD / NUM_RINGS as f32 * (NUM_RINGS - i) as f32;

        for j in 0..POINTS_PER_LINE {
            let spatial_x = (j as f32 / 200. * TAU).cos();
            let spatial_y = (j as f32 / 200. * TAU).sin();
            let mut offset: f32 = 1.;

            let delta = (app.elapsed_frames() % 360) as f64 * PI_F64 / 180.;
            let temporal_x = delta.cos() * SPEED_MULTIPLIER;
            let temporal_y = delta.sin() * SPEED_MULTIPLIER;

            if model.tween.frames > 0 {
                let comp_offset = noisefn.get([
                    spatial_x as f64,
                    spatial_y as f64 + i as f64 * OFFSET_MULTIPLIER,
                    temporal_x,
                    temporal_y,
                ]) as f32
                    * 2.;
                match model.tween.mode_to {
                    Mode::Circles => {
                        offset =
                            map_range(model.tween.frames, 1, NUM_TWEEN_FRAMES, 1., comp_offset);
                    }
                    Mode::Noise => {
                        offset = map_range(
                            NUM_TWEEN_FRAMES - model.tween.frames,
                            1,
                            NUM_TWEEN_FRAMES,
                            1.,
                            comp_offset,
                        );
                    }
                }
            } else {
                if model.tween.mode_to == Mode::Noise {
                    offset = noisefn.get([
                        spatial_x as f64,
                        spatial_y as f64 + i as f64 * OFFSET_MULTIPLIER,
                        temporal_x,
                        temporal_y,
                    ]) as f32
                        * 2.;
                }
            }

            points[j as usize] = pt2(
                spatial_x * (offset * rad + rad),
                spatial_y * (offset * rad + rad),
            );
        }
        model.lines.push(points);
        model.completion = i as f32 / NUM_RINGS as f32;
    }
    if model.tween.frames > 0 {
        model.tween.frames -= 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    for line in &model.lines {
        draw.path()
            .stroke()
            .points(*line)
            .color(Srgba::<u8> {
                alpha: map_range(model.completion, 0., 1., 0., 255.) as u8,
                color: color_for(model.completion as u32 * NUM_RINGS as u32),
            })
            .x_y(0., 0.);
    }

    draw.to_frame(app, &frame).unwrap();
}
