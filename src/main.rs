#![feature(is_sorted)]
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use palette::{Gradient, Hsv, LinSrgb, Srgb};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod sorts;

use sorts::{
    bogo::BogoSort, bubble::BubbleSort, insertion::InsertionSort, selection::SelectionSort,
};

const OPENGL: OpenGL = OpenGL::V4_5;

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 1250;

trait SortingAlgorithm {
    fn checked_step(&mut self, _args: &UpdateArgs) -> bool {
        if self.is_locked() {
            return true;
        }

        return self.step(_args);
    }
    fn step(&mut self, _args: &UpdateArgs) -> bool;
    fn members(&self) -> &Vec<usize>;
    fn is_locked(&self) -> bool;
}

fn render_alg(
    sort: &Box<dyn SortingAlgorithm>,
    arg: &RenderArgs,
    gl: &mut GlGraphics,
    x_offset: usize,
    total_sorts: usize,
) {
    // let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    // let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    let scale = (**sort).members();
    let data_len = scale.len();
    let x_size_mult = (WINDOW_WIDTH as f64 / total_sorts as f64) as f64;
    let y_size_mult = (WINDOW_HEIGHT as f64 / scale.len() as f64) as f64;
    let sorted = scale.is_sorted();

    let scale = scale
        .iter()
        .zip((0..scale.len()).fold(Vec::new(), |mut accum, itm| {
            accum.push(graphics::rectangle::rectangle_by_corners(
                (x_offset as f64) * x_size_mult,
                (itm as f64) * y_size_mult,
                (1.0 + x_offset as f64) * x_size_mult,
                (1.0 + itm as f64) * y_size_mult,
            ));
            accum
        }));

    gl.draw(arg.viewport(), |c, gl| {
        let color_grad = Gradient::new(vec![
            Hsv::from(LinSrgb::new(1.0, 0.0, 0.0)),           // red
            Hsv::from(LinSrgb::new(1.0, 165.0 / 255.0, 0.0)), // orange
            Hsv::from(LinSrgb::new(1.0, 1.0, 0.0)),           // yellow
            Hsv::from(LinSrgb::new(0.0, 0.5, 0.0)),           // green
            Hsv::from(LinSrgb::new(0.0, 0.0, 1.0)),           // blue
            Hsv::from(LinSrgb::new(0.5, 0.0, 0.5)),           // purple
        ]);
        let transform = c.transform;
        scale.for_each(|(&val, rectangle)| {
            let color = if !sorted {
                let hsv = color_grad.get(val as f32 / data_len as f32);
                let rgb = LinSrgb::from(hsv);
                let rgb = rgb.into_components();
                [rgb.0, rgb.1, rgb.2, 1.0_f32]
            } else {
                [0.0, val as f32 / data_len as f32, 0.0, 1.0]
            };
            graphics::rectangle(color, rectangle, transform, gl);
        })
    });
}

fn game_loop(
    event_settings: EventSettings,
    mut window: Window,
    mut algs: Vec<Box<dyn SortingAlgorithm>>,
) -> ! {
    let mut graphics = GlGraphics::new(OPENGL);
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            for (idx, alg) in algs.iter().enumerate() {
                render_alg(&alg, &r, &mut graphics, idx, algs.len());
            }
        }

        if let Some(u) = e.update_args() {
            for alg in algs.iter_mut() {
                alg.checked_step(&u);
            }
        }
    }

    std::process::exit(0);
}

fn get_window(width: u32, height: u32) -> Window {
    WindowSettings::new("sort-visualizer", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|err| {
            panic!(
                "Failed to build piston window. You may need to disable HDR. Full error: {}",
                err
            )
        })
}

fn main() {
    let window = get_window(WINDOW_WIDTH, WINDOW_HEIGHT);
    let event_settings = get_event_settings();

    let sorts: Vec<Box<dyn SortingAlgorithm>> = vec![
        // Box::new(BogoSort::new(5)),
        // Box::new(BogoSort::new(10)),
        // Box::new(BogoSort::new(25)),
        // Box::new(BogoSort::new(50)),
        // Box::new(InsertionSort::new(1250)),
        Box::new(BubbleSort::new(1250)),
        // Box::new(SelectionSort::new(1250)),
    ];

    game_loop(event_settings, window, sorts);
}

fn get_event_settings() -> EventSettings {
    let mut event_settings = EventSettings::new();
    event_settings.ups = 100;
    event_settings.max_fps = 250;

    event_settings
}
