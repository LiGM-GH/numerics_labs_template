use color_eyre::Result;
use plotters::{
    prelude::{ChartContext, CoordTranslate, DrawingBackend},
    style::colors,
};

pub fn lab_main<Backend, Coord>(chart: &mut ChartContext<Backend, Coord>) -> Result<()>
where
    Backend: DrawingBackend,
    Backend::ErrorType: 'static,
    Coord: CoordTranslate<From = (f64, f64)>,
{
    draw_function!(chart (colors::RED): original_fn);
    draw_function!(chart (colors::CYAN): nth_sum(4));

    for idx in 1..=5 {
        draw_function!(chart (colors::GREEN): compare(original_fn, nth_sum(idx)), idx = idx);
    }

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
