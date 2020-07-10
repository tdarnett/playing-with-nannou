use nannou::prelude::*;
fn main() {
    nannou::app(model).simple_window(view).run()
}

struct Model {}

const FRAMES: usize = 251;
const TIMES: u32 = 2;

fn model(app: &App) -> Model {
    app.main_window().set_inner_size_pixels(1920, 1200);
    app.set_loop_mode(LoopMode::NTimes {
        number_of_updates: FRAMES + 1,
    });
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let main_colour = rgb(213u8, 202u8, 184u8);
    let seconday_colour = rgb(27u8, 32u8, 43u8);
    // draw.background().rgb(r: 192, 185, 173)/;
    draw.background().color(seconday_colour);

    let n = 100;

    let offset: f32 = (TIMES as f32) * 2. * PI * (frame.nth() as f32) / (FRAMES as f32 / 1.2);

    let points = (0..n).map(|i| {
        let x = i as f32 - (n as f32 / 1.5);
        let point = pt2(x, ((x / 3.) - (offset - 2.)).cos()) * 10.0;
        point
    });

    for i in 0..200 {
        draw.polyline()
            .weight(1.5)
            .color(main_colour)
            .points(points.clone())
            // .rotate(i as f32 / 1.); // pretty cool actually
            .rotate(i as f32 / 50.); // pretty cool actually
    }
    draw.ellipse().color(main_colour);

    draw.to_frame(app, &frame).unwrap();
    app.main_window()
        .capture_frame(format!("output/ninth/{:03}.png", frame.nth()));
    // to turn into mp4
}
