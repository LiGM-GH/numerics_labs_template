#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use color_eyre::{eyre::anyhow, Result};
use plotters::prelude::*;

mod draw_function_macro;
mod lab_main;

use lab_main::lab_main;

const Y_MAX: f64 = 100.0;
const Y_MIN: f64 = 0.001;
const X_MAX: f64 = 0.0;
const X_MIN: f64 = -4.0;
const NUM_POINTS_ON_DISPLAY: i32 = 15000;

fn main() -> Result<()> {
    // Program can still execute if this fails, so we'll just ignore failure if it occurs.
    color_eyre::install().ok();

    let args = std::env::args().nth(1);

    let Some(image_path) = args else {
        return Err(anyhow!("Couldn't get filename to write JPG to. Provide it as an command-line argument"));
    };

    let root = BitMapBackend::new(&image_path, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(15)
        .y_label_area_size(30)
        .build_cartesian_2d(X_MIN..X_MAX, (Y_MIN..Y_MAX).log_scale())?;

    chart.configure_mesh().draw()?;

    lab_main(&mut chart)?;

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
