// Tim Lobner

use std::path::Path;
use std::collections::HashMap;
use plotters::prelude::*;
use chrono::NaiveDate;

use crate::band::Band;

/// Creates an SVG timetable with stages on the x-axis and bands as rectangles on the y-axis.
/// The number of stages is determined dynamically from the bands' data.
pub fn create_table(out_path: &Path, bands: &[Band], day_label: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("writing image to {}", out_path.display());

    if bands.is_empty() {
        return Err("No bands provided".into());
    }

    // Collect unique stage names
    let mut stage_names: Vec<String> = bands.iter()
        .map(|b| b.stage.clone())
        .collect();
    stage_names.sort();
    stage_names.dedup();

    // Create a mapping from stage name to index
    let stage_to_index: HashMap<String, usize> = stage_names
        .iter()
        .enumerate()
        .map(|(i, name)| (name.clone(), i))
        .collect();

    // Find the earliest start and latest end time among all bands
    let first_band_start = bands.iter().map(|b| b.start_dt).min().unwrap();
    let last_band_end = bands.iter().map(|b| b.end_dt).max().unwrap();
    
    let first_utc = first_band_start.and_utc();
    let last_utc = last_band_end.and_utc();

    // SVG backend for selectable text
    let drawing_area = SVGBackend::new(out_path, (1024, 768))
        .into_drawing_area();

    drawing_area.fill(&WHITE)?;
    
    // Ensure rectangles are fully visible by providing enough margin
    // Rectangle width is 0.8, so we need at least 0.4 padding on each side
    let chart_width = stage_names.len() as f32;
    let margin = 0.5; // Slightly more than half the rectangle width for safety
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(day_label, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .build_cartesian_2d(-margin..(chart_width - 1.0 + margin), first_utc..last_utc)?;

    // Configure the mesh - position labels at rectangle centers
    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("Time")
        .y_max_light_lines(1)
        .x_desc("Stages")
        .x_labels(stage_names.len())
        .x_label_formatter(&|x| {
            let idx = x.round() as usize;
            if idx < stage_names.len() {
                stage_names[idx].clone()
            } else {
                String::new()
            }
        })
        .draw()?;

    // Draw rectangles and text for each band
    for band in bands {
        if let Some(&stage_idx) = stage_to_index.get(&band.stage) {
            // Position rectangles so their centers align with the integer tick marks
            let x_center = stage_idx as f32;
            let rect_width = 0.8;
            let x_start = x_center - rect_width / 2.0;
            let x_end = x_center + rect_width / 2.0;
            
            let start_time = band.start_dt.and_utc();
            let end_time = band.end_dt.and_utc();
            
            // Draw the band rectangle centered on the tick mark
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x_start, start_time), (x_end, end_time)],
                BLUE.mix(0.7).filled(),
            )))?;
            
            // Calculate middle time for text
            let duration_seconds = (end_time.timestamp() - start_time.timestamp()) / 2;
            let middle_time = start_time + chrono::Duration::seconds(duration_seconds);
            
            // Draw band name at the center of the rectangle
            chart.draw_series(std::iter::once(Text::new(
                band.name.clone(),
                (x_center, middle_time),
                ("Arial", 10).into_font().color(&WHITE),
            )))?;
        }
    }

    // Ensure the chart is properly finalized
    drawing_area.present()?;
    
    Ok(())
}
