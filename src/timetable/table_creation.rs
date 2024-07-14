// Tim Lobner

use std::path::Path;
use plotters::prelude::{SVGBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, IntoSegmentedCoord, LineSeries, BLACK, WHITE};


pub fn create_table(out_path: &Path){
    println!("writing image to {}", out_path.display());

    // TODO: testing variables, need to come from actual data later on
    let days = [ "Wednesday", "Thursday", "Friday", "Saturday" ];
    let current_day_index = 2;
    let current_day = days[current_day_index];
    let num_stages = 4;

    let first_band_start = 12;
    let last_band_end = 27;

    // use an SVG backend to have selectable text.
    // turn into a pdf for printing later
    let drawing_area = SVGBackend::new(out_path, (1024, 768))
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(current_day, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        // the x axis are the sages. we want the names centered
        // on an entry (i.e. it starts with half in front of a segment
        // and ends half after a segment, rather than matching segments 
        // exactle)
        // ----|---------------|-------------|-------
        //   main stage     t stage      wera stage 
        // for that behaviour, we need the into_segmented() command
        .build_cartesian_2d((0..num_stages).into_segmented(), last_band_end..first_band_start)
        .unwrap();

    // draw axes labels for time on y axis (top to bottom for increasing time)
    // and stage names on the x axis

    // draw a mesh and axes labels. disable x part of mesh (only want time lines)
    chart.configure_mesh()
        .disable_x_mesh()
        .draw().unwrap();

    // draw a diagonal line
    // chart.draw_series(
    //     LineSeries::new((0..100).map(|x| (x, 100 -x)), &BLACK),
    // ).unwrap();
}
