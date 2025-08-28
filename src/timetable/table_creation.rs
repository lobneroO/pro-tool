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
    
    // Function to transform time coordinates for visual inversion
    let transform_time = |time: chrono::DateTime<chrono::Utc>| -> chrono::DateTime<chrono::Utc> {
        let total_duration = last_utc.timestamp() - first_utc.timestamp();
        let time_offset = time.timestamp() - first_utc.timestamp();
        let inverted_offset = total_duration - time_offset;
        first_utc + chrono::Duration::seconds(inverted_offset)
    };
    
    // Use NORMAL ascending range for coordinate system - this allows automatic labels to work
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(day_label, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .build_cartesian_2d(-margin..(chart_width - 1.0 + margin), first_utc..last_utc)?;

    // Configure the mesh with automatic y-labels that show the logical times
    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("Time")
        .y_max_light_lines(1)
        .y_labels(8) // This will now work properly with ascending range
        .y_label_formatter(&|y| {
            // Apply inverse transformation to show the actual logical time
            // The coordinate y represents a position in the coordinate system
            // We need to transform it back to get the actual time that should be displayed
            let actual_time = transform_time(*y);
            actual_time.format("%H:%M").to_string()
        })
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

    // Draw rectangles and text for each band with coordinate transformation
    for band in bands {
        if let Some(&stage_idx) = stage_to_index.get(&band.stage) {
            // Position rectangles so their centers align with the integer tick marks
            let x_center = stage_idx as f32;
            let rect_width = 0.8;
            let x_start = x_center - rect_width / 2.0;
            let x_end = x_center + rect_width / 2.0;
            
            let start_time = band.start_dt.and_utc();
            let end_time = band.end_dt.and_utc();
            
            // Transform coordinates for inverted visual display
            let display_start = transform_time(start_time);
            let display_end = transform_time(end_time);
            
            // Draw the band rectangle with transformed coordinates
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x_start, display_start), (x_end, display_end)],
                BLUE.mix(0.7).filled(),
            )))?;
            
            // Calculate middle time for text using chrono's built-in methods
            let start_timestamp = start_time.timestamp();
            let end_timestamp = end_time.timestamp();
            
            // Check for valid timestamp range to prevent overflow
            if end_timestamp >= start_timestamp {
                let middle_timestamp = start_timestamp + (end_timestamp - start_timestamp) / 2;
                
                // Create middle time safely
                if let Some(middle_time) = chrono::DateTime::from_timestamp(middle_timestamp, 0) {
                    // Transform middle time for display
                    let display_middle = transform_time(middle_time);
                    
                    // Draw band name at the center of the rectangle
                    chart.draw_series(std::iter::once(Text::new(
                        band.name.clone(),
                        (x_center, display_middle),
                        ("Arial", 10).into_font().color(&WHITE),
                    )))?;
                } else {
                    // Fallback to transformed start time if middle calculation fails
                    chart.draw_series(std::iter::once(Text::new(
                        band.name.clone(),
                        (x_center, display_start),
                        ("Arial", 10).into_font().color(&WHITE),
                    )))?;
                }
            } else {
                // Fallback if timestamps are invalid
                chart.draw_series(std::iter::once(Text::new(
                    band.name.clone(),
                    (x_center, display_start),
                    ("Arial", 10).into_font().color(&WHITE),
                )))?;
            }
        }
    }

    // Ensure the chart is properly finalized
    drawing_area.present()?;
    
    Ok(())
}
