use plotters::prelude::*;

/// Creates a plot.
pub fn plot(lijnen: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Kogelsnelheid", ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..1.0, 0.0..4.5)?;

    chart
        .configure_mesh()
        .x_desc("Hoogteverschil in m")
        .y_desc("Snelheid in m/s^2")
        .draw()?;

    chart
        .draw_series(LineSeries::new(lijnen, &RED))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    root.present()?;

    Ok(())
}

/// Calculates kogel snelheid
pub fn kogelSnelheidBerekenen(
    mut hoogte: f32,
    deltaH: f32,
    massa: f32,
    valversnelling: f32,
) -> f64 {
    hoogte -= deltaH;
    let zwaarte_energie = massa * valversnelling * hoogte;
    let kinetische_energie = 19620.0 - zwaarte_energie;
    (2f32 * kinetische_energie / massa).sqrt() as f64
}
