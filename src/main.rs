use plotters::prelude::*;

const Y_MAX: f32 = 1.0;
const Y_MIN: f32 = -1.0;

fn main() -> color_eyre::Result<()> {
    draw(|x| x + 0.4)
}

fn draw(fun: impl Fn(f32) -> f32) -> color_eyre::Result<()> {
    let root = BitMapBackend::new("images/0.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y = x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, Y_MIN..Y_MAX)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).filter_map(|x| {
                let y = fun(x);
                if y > Y_MAX || y < Y_MIN {
                    None
                } else {
                    Some((x, y))
                }
            }),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
