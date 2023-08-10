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

const Y_MAX: f64 = 50.0;
const Y_MIN: f64 = -10.0;
const X_MAX: f64 = 10.0;
const X_MIN: f64 = -10.0;
const NUM_POINTS_ON_DISPLAY: i32 = 1500;

macro_rules! draw_function {
    ($chart:ident ($color:expr): $fun:expr) => {
        #[allow(clippy::redundant_closure)]
        #[allow(unused_variables)]
        $chart
            .draw_series(LineSeries::new(
                (0..=NUM_POINTS_ON_DISPLAY).filter_map(|x| {
                    let x = (x as f64 / NUM_POINTS_ON_DISPLAY as f64).mul_add(X_MAX - X_MIN, X_MIN);
                    let y = $fun(x);

                    if !(Y_MIN..Y_MAX).contains(&y) {
                        return None;
                    }

                    Some((x, y))
                }),
                &$color,
            ))?
            .label(stringify!($fun).replace("|x| ", "y = "))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &$color));
    };
}

fn main() -> Result<()> {
    // Program can still execute if this fails, so we'll just ignore failure if it occurs.
    color_eyre::install().ok();

    let args = std::env::args().collect::<Vec<_>>();
    let Some(image_path) = args.get(1) else {
        return Err(anyhow!("Couldn't get filename to write JPG to. Provide it as an command-line argument"));
    };

    let root = BitMapBackend::new(image_path, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y = f(x)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(X_MIN..X_MAX, Y_MIN..Y_MAX)?;

    chart.configure_mesh().draw()?;

    draw_function!(chart (plotters::style::colors::BLACK): |x| 0.0);
    draw_function!(chart (plotters::style::colors::full_palette::DEEPPURPLE_900): |x| given_f(x));
    draw_function!(chart (plotters::style::colors::full_palette::ORANGE): |x| x + 0.2);
    draw_function!(chart (plotters::style::colors::full_palette::GREEN): |x| f64::ln(x));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn given_f(x: f64) -> f64 {
    10.45f64.mul_add(-x, 1.6f64.mul_add(x.powi(3), -1.7 * x.powi(2)))
}
