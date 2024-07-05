// Tim Lobner

use std::path::Path;
use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, BLACK, WHITE};


pub fn create_table(out_path: &Path){
    println!("writing image to {}", out_path.display());

    let drawing_area = BitMapBackend::new(out_path, (1024, 768))
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.draw_series(
        LineSeries::new((0..100).map(|x| (x, 100 -x)), &BLACK),
    ).unwrap();
}
