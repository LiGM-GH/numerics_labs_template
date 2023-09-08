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
use plotters::style::colors;

mod draw_function_macro;

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

    /* ===============< THIS IS WHERE THE MAIN STARTS >=============== */
    {
        draw_function!(chart (colors::RED): original_fn);
        draw_function!(chart (colors::CYAN): nth_sum(4));

        for idx in 1..=5 {
            draw_function!(chart (colors::GREEN): compare(original_fn, nth_sum(idx)), idx = idx);
        }
    }
    /* ===============<  THIS IS WHERE THE MAIN ENDS  >=============== */

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn compare(
    mut f1: impl FnMut(f64) -> f64,
    mut f2: impl FnMut(f64) -> f64,
) -> impl FnMut(f64) -> f64 {
    move |x| {
        let f1_val = f1(x);
        let f2_val = f2(x);
        let val = f1_val - f2_val;
        let val = val / f1_val;
        val
    }
}
/// f(x) = x^2 * (e^x - x - 1)
fn original_fn(x: f64) -> f64 {
    x.powi(2) * f64::exp(x) - x.powi(3) - x.powi(2)
}

fn nth_sum(n: usize) -> impl Fn(f64) -> f64 {
    move |x| sum(n, x)
}

/// f(x) = x^4/2! + x^5/3! + x^6/4! + …
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[inline]
fn sum(n: usize, x: f64) -> f64 {
    let mut sum = x.powi(4) / 2.0;
    let mut current_summed = sum;

    for i in 1..n {
        current_summed *= x / (i as f64 + 2.0);
        sum += current_summed;
    }

    sum
}
