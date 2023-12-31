#![macro_use]
//! `draw_function!` macro and also all the tests for it.

macro_rules! draw_function {
    ($chart:ident ($color:expr): $fun:expr$(, $ident:ident = $val:expr)*$(,)?) => {{
        draw_bounded_function!($chart ($color, [crate::NUM_POINTS_ON_DISPLAY] {crate::Y_MIN, crate::Y_MAX} {crate::Y_MIN, crate::Y_MAX}): $fun $(, $ident = $val)*);
    }};
}

/// ```
/// ($chart:ident (
///     $color:expr,
///     [$NUM_POINTS_ON_DISPLAY:expr]
///     {$X_MIN:expr, $X_MAX:expr}
///     {$Y_MIN:expr, $Y_MAX:expr}
/// ): $fun:expr$(, $ident:ident = $val:expr)*$(,)?) => {{
/// ```
macro_rules! draw_bounded_function {
    ($chart:ident ($color:expr, [$NUM_POINTS_ON_DISPLAY:expr] {$X_MIN:expr, $X_MAX:expr} {$Y_MIN:expr, $Y_MAX:expr}): $fun:expr$(, $ident:ident = $val:expr)*$(,)?) => {{
        #[allow(clippy::redundant_closure)]
        #[allow(unused_variables)]
        $chart
            .draw_series(plotters::prelude::LineSeries::new(
                    (0..=$NUM_POINTS_ON_DISPLAY).filter_map(|x| {
                        let x = (x as f64 / $NUM_POINTS_ON_DISPLAY as f64).mul_add($X_MAX - $X_MIN, $X_MIN);
                        let y = $fun(x);

                        if !($Y_MIN..$Y_MAX).contains(&y) {
                            return None;
                        }

                        Some((x, y))
                    }),
                    &$color,
            ))?
            .label(
                stringify!($fun)
                .replace("|x| ", "y = ")
                $(
                    .replace(stringify!($ident), &format!("{:?}", $val))
                )*,
            )
            .legend(move |(x, y)| plotters::prelude::PathElement::new(vec![(x, y), (x + 20, y)], &$color));
    }}
}

#[cfg(test)]
mod tests {
    //! Use of `use` should be avoided as it may lead to injection of scope in the macro and thus lead to successful compilation in cases like the following:
    //! ```rust
    //! macro_rules! do_sth {
    //!     () => {
    //!         Path::new("some/path")
    //!     }
    //! }
    //!
    //! {
    //!     use std::path::Path;
    //!     // This compiles successfully
    //!     do_sth!();
    //! }
    //!
    //! {
    //!     use plotters::prelude::Path;
    //!     // This doesn't.
    //!     do_sth!();
    //! }
    //! ```

    const TESTS_DIR: &'static str = "tests";
    static mut MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Function which will be printed with the macro in question.
    fn function(_: f64) -> f64 {
        4.0
    }

    fn function_ret_closure(n: usize) -> impl Fn(f64) -> f64 {
        move |x| x.abs() * n as f64
    }

    #[test]
    fn draw_function_draws() -> color_eyre::Result<()> {
        {
            let _guard = unsafe { MUTEX.lock().ok() };

            if !std::fs::metadata(TESTS_DIR).is_ok_and(|val| val.is_dir()) {
                std::fs::create_dir(std::path::Path::new(TESTS_DIR))?;
            }
        }

        let image_path = std::path::Path::new(TESTS_DIR).join("draw_test.png");

        let root = plotters::prelude::IntoDrawingArea::into_drawing_area(
            plotters::prelude::BitMapBackend::new(&image_path, (640, 480)),
        );

        root.fill(&plotters::style::colors::WHITE)?;

        let mut chart = plotters::prelude::ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(15)
            .y_label_area_size(30)
            .build_cartesian_2d(crate::X_MIN..crate::X_MAX, crate::Y_MIN..crate::Y_MAX)?;

        chart.configure_mesh().draw()?;

        draw_function!(chart (plotters::style::colors::BLACK): function);

        chart.configure_series_labels().draw()?;

        Ok(())
    }

    #[test]
    fn draw_function_draws_and_changes_idents() -> color_eyre::Result<()> {
        {
            let _guard = unsafe { MUTEX.lock().ok() };

            if !std::fs::metadata(TESTS_DIR).is_ok_and(|val| val.is_dir()) {
                std::fs::create_dir(std::path::Path::new(TESTS_DIR))?;
            }
        }

        let image_path = std::path::Path::new(TESTS_DIR).join("changes_idents.png");

        let root = plotters::prelude::IntoDrawingArea::into_drawing_area(
            plotters::prelude::BitMapBackend::new(&image_path, (640, 480)),
        );

        root.fill(&plotters::style::colors::WHITE)?;
        let mut chart = plotters::prelude::ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(15)
            .y_label_area_size(30)
            .build_cartesian_2d(crate::X_MIN..crate::X_MAX, crate::Y_MIN..crate::Y_MAX)?;

        chart.configure_mesh().draw()?;
        let ident = 14;

        draw_function!(chart (plotters::style::colors::BLACK): function_ret_closure(ident), ident = "Value");

        chart.configure_series_labels().draw()?;

        Ok(())
    }
}
