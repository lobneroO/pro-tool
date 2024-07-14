// Tim Lobner

use std::path::Path;
// use plotters::prelude::{SVGBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, IntoSegmentedCoord, DrawingBackend, Rectangle};
// use plotters::prelude::{BLACK, WHITE};
// TODO: use above individual use pattern...
use plotters::prelude::*;

use chrono::NaiveDate;

use crate::band::Band;

pub fn create_table(out_path: &Path){
    println!("writing image to {}", out_path.display());

    // TODO: testing variables, need to come from actual data later on
    let days = [ "Wednesday", "Thursday", "Friday", "Saturday" ];
    let current_day_index = 2;
    let current_day = days[current_day_index];
    let stages = [ "Main Stage", "T Stage", "Wera Rebel Stage", "Campsite Circus Stage" ];

    // TODO: naive date time not supported by plotter library
    // but UTC is. converting everything utc on the fly should work,
    // since the user will not care if the output is 8am local or 8utc
    let first_band_start = 
        NaiveDate::from_ymd_opt(2024, 8, 15)
            .unwrap()
            .and_hms_opt(6, 0, 0).
            unwrap().and_utc();
    let last_band_end = 
        NaiveDate::from_ymd_opt(2024, 8, 15)
            .unwrap()
            .and_hms_opt(11, 59, 0).
            unwrap().and_utc();

    let fleshgod = Band {
        name: String::from("Fleshgod Apocalypse"),
        selected: true,
        stage: String::from("T Stage"),
        start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(17, 20, 0).unwrap(), 
        end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(18, 30, 0).unwrap(),
    };

    let d1 = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(17, 20, 0).unwrap().and_utc(); 
    let d2 = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(18, 30, 0).unwrap().and_utc(); 

    let data = [d1, d2];
    let bands = [ fleshgod ];

    // use an SVG backend to have selectable text.
    // turn into a pdf for printing later
    let drawing_area = SVGBackend::new(out_path, (1024, 768))
        .into_drawing_area();
    
    drawing_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(current_day, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        // the x axis are the sages. we want the names centered
        // on an entry (i.e. it starts with half in front of a segment
        // and ends half after a segment, rather than matching segments 
        // exactle)
        // ----|---------------|-------------|-------
        //   main stage     t stage      wera stage 
        // for that behaviour, we need the into_segmented() command
        .build_cartesian_2d(stages.into_segmented(), first_band_start..last_band_end)
        .unwrap();

    // draw axes labels for time on y axis (top to bottom for increasing time)
    // and stage names on the x axis

    // draw a mesh and axes labels. disable x part of mesh (only want time lines)
    chart.configure_mesh()
        .disable_x_mesh()
        .draw().unwrap();

    let plot_area = chart.plotting_area();
    // plot_area.map_coordinate()

    let center = SegmentValue::CenterOf(&"T Stage");
    chart.draw_series(
        vec![
            Rectangle::new(
                [(center.clone(), d1), (center.clone(), d2)],
                Into::<ShapeStyle>::into(BLACK).filled()
            ),
        ]
    ).unwrap();
    chart.draw_series(
        vec![
            Circle::new((center.clone(), d1), 
                20,
                BLACK)
        ]
    ).unwrap();
    
    // let int_data = [(1.0, 3.3)];
    // chart.draw_series(int_data.map(|(x, y)| {
    //     EmptyElement::at((x, y))
    //     + Circle::new((0, 0), 10, BLACK)
    // })).unwrap();
    //
    // let temp_data = [("T Stage", d1, d2)];
    // chart.draw_series(temp_data.map(|(stage, start, end)| {
    //     EmptyElement::at((stage, start))
    //      + Circle::new((0, 0), 10, BLACK)
    // })).unwrap();


    // plot_area.draw(&Rectangle::new(
    //     [("T Stage", d1), ("T Stage", d2,)], // coordinates
    //     Into::<ShapeStyle>::into(BLACK).filled(),
    // )).unwrap();
    // plot_area.draw(Rectangle::new())
    
    // coord_trans()
    // draw based on data
    

    // draw a diagonal line
    // chart.draw_series(
    //     LineSeries::new((0..100).map(|x| (x, 100 -x)), &BLACK),
    // ).unwrap();
}
