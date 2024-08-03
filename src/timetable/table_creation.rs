// Tim Lobner

use std::path::Path;
// use plotters::prelude::{SVGBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, IntoSegmentedCoord, DrawingBackend, Rectangle};
// use plotters::prelude::{BLACK, WHITE};
// these uses are on purpose to reduce the function signature size when inputting a chart.
use plotters::coord::ranged1d::SegmentedCoord;
use plotters::coord::types::RangedSlice;
use chrono::prelude::{DateTime, Utc};
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
    let stages: Vec<String> = vec![
        String::from("Main Stage"),
        String::from("T Stage"),
        String::from("Wera Rebel Stage"),
        String::from("Campsite Circus Stage"),
    ];

    // TODO: naive date time not supported by plotter library
    // but UTC is. converting everything utc on the fly should work,
    // since the user will not care if the output is 8am local or 8utc
    let first_band_start = 
        NaiveDate::from_ymd_opt(2024, 8, 15)
            .unwrap()
            .and_hms_opt(6, 0, 0).
            unwrap();
    let last_band_end = 
        NaiveDate::from_ymd_opt(2024, 8, 15)
            .unwrap()
            .and_hms_opt(20, 59, 0).
            unwrap();
    let first_utc = first_band_start.and_utc();
    let last_utc = last_band_end.and_utc();

    let fleshgod = Band {
        name: String::from("Fleshgod Apocalypse"),
        selected: true,
        stage: String::from("T Stage"),
        start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(17, 20, 0).unwrap(), 
        end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(18, 30, 0).unwrap(),
    };
    let nathrakh = Band {
        name: String::from("Anaal Nathrakh"),
        selected: true,
        stage: String::from("T Stage"),
        start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(16, 30, 0).unwrap(),
        end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(17, 15, 0).unwrap(),
    };
    let meshuggah = Band {
        name: String::from("Meshuggah"),
        selected: true,
        stage: String::from("Main Stage"),
        start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(17, 30, 0).unwrap(),
        end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(19, 00, 0).unwrap(),
    };

    let bands: Vec<Band> = vec![ fleshgod, nathrakh, meshuggah ];

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
        .build_cartesian_2d(stages.into_segmented(), first_utc..last_utc)
        .unwrap();

    // draw axes labels for time on y axis (top to bottom for increasing time)
    // and stage names on the x axis

    // draw a mesh and axes labels. disable x part of mesh (only want time lines)
    chart.configure_mesh()
        .disable_x_mesh()
        .draw().unwrap();

    // draw all bands into the chart
    chart.draw_series(
        bands.iter().map(|band| {
            Rectangle::new(
                [
                    (SegmentValue::Exact(&band.stage), band.start_dt.and_utc()),
                    (SegmentValue::CenterOf(&band.stage), band.end_dt.and_utc()),
                ],
                Into::<ShapeStyle>::into(BLACK).filled()
            )
        })
    ).unwrap();
}
