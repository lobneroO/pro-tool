// Tim Lobner

use std::path::Path;
use std::collections::HashMap;
use plotters::prelude::*;
use chrono::Timelike;
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

    // Round to sensible boundaries for display
    // Start: 30 minutes before first start time, rounded down to nearest 30 minutes
    let display_start_minutes = (first_utc.minute() / 30) * 30;
    let display_start = first_utc
        .with_minute(display_start_minutes).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap() - chrono::Duration::minutes(30);
    
    // End: 30 minutes after last end time, rounded up to nearest 30 minutes  
    let display_end_minutes = ((last_utc.minute() + 29) / 30) * 30;
    let display_end = if display_end_minutes >= 60 {
        last_utc
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap() + chrono::Duration::hours(1) + chrono::Duration::minutes(30)
    } else {
        last_utc
            .with_minute(display_end_minutes).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap() + chrono::Duration::minutes(30)
    };

    // Convert to numeric coordinates to avoid DateTime overflow issues
    let start_timestamp = display_start.timestamp() as f64;
    let end_timestamp = display_end.timestamp() as f64;
    
    // Helper functions to convert between timestamps and datetime
    let timestamp_to_datetime = |ts: f64| -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or(display_start)
    };
    
    let datetime_to_timestamp = |dt: chrono::DateTime<chrono::Utc>| -> f64 {
        dt.timestamp() as f64
    };

    // SVG backend for selectable text
    let drawing_area = SVGBackend::new(out_path, (1024, 768))
        .into_drawing_area();

    drawing_area.fill(&WHITE)?;
    
    // Rectangle width and chart layout - add margin to accommodate rectangle width
    let chart_width = stage_names.len() as f32;
    let rect_width = 0.8;
    let margin = rect_width / 2.0; // Half rectangle width for proper spacing
    
    // Function to transform time coordinates for visual inversion (using numeric coordinates)
    let transform_time = |timestamp: f64| -> f64 {
        let total_duration = end_timestamp - start_timestamp;
        let time_offset = timestamp - start_timestamp;
        let inverted_offset = total_duration - time_offset;
        start_timestamp + inverted_offset
    };
    
    // Use NUMERIC coordinate system with proper margins to contain rectangles
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(day_label, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .build_cartesian_2d(-margin..(chart_width - 1.0 + margin), start_timestamp..end_timestamp)?;

    // Configure mesh with numeric coordinates (no DateTime calculations)
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Time")
        .x_desc("Stages")
        .y_labels(0)  // We'll draw custom labels
        .x_labels(stage_names.len())
        .x_label_formatter(&|x| {
            let idx = (*x + margin).round() as usize; // Adjust for margin offset
            if idx < stage_names.len() {
                stage_names[idx].clone()
            } else {
                String::new()
            }
        })
        .draw()?;

    // Generate custom time labels at 30-minute intervals using numeric coordinates
    let total_duration_minutes = (end_timestamp - start_timestamp) / 60.0;
    let label_interval_minutes = 30.0;
    let num_labels = (total_duration_minutes / label_interval_minutes) as i32 + 1;
    
    // Draw time labels directly on the drawing area (outside chart coordinate system)
    for i in 0..num_labels {
        let label_timestamp = start_timestamp + (i as f64 * label_interval_minutes * 60.0);
        if label_timestamp <= end_timestamp {
            // Convert back to datetime for display WITHOUT transformation
            // We want to show actual times, not inverted times
            let actual_datetime = timestamp_to_datetime(label_timestamp);
            
            // Calculate pixel position on the drawing area
            // Chart area starts at (80, 30) due to label area sizes
            let chart_top = 30; // Caption area
            let chart_height = 768 - 30 - 100; // Total height - caption - bottom label area
            let relative_position = (label_timestamp - start_timestamp) / (end_timestamp - start_timestamp);
            let pixel_y = chart_top + (relative_position * chart_height as f64) as i32;
            
            // Draw label to the left of the chart area
            drawing_area.draw(&Text::new(
                actual_datetime.format("%H:%M").to_string(),
                (10, pixel_y), // 10 pixels from left edge
                ("Arial", 12).into_font().color(&BLACK),
            ))?;
        }
    }

    // Draw horizontal grid lines at time intervals using chart coordinates
    for i in 0..num_labels {
        let label_timestamp = start_timestamp + (i as f64 * label_interval_minutes * 60.0);
        if label_timestamp <= end_timestamp {
            let actual_datetime = timestamp_to_datetime(label_timestamp);
            
            // Determine if this is a full hour (darker gray) or half hour (light gray)
            let is_full_hour = actual_datetime.minute() == 0;
            let line_color = if is_full_hour {
                RGBColor(128, 128, 128) // Darker gray for full hours
            } else {
                RGBColor(200, 200, 200) // Light gray for half hours
            };
            
            // Transform the timestamp to match the visual inversion used by rectangles
            let transformed_timestamp = transform_time(label_timestamp);
            
            // Draw horizontal line across the entire chart width using chart coordinates
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (-margin, transformed_timestamp),
                    (chart_width - 1.0 + margin, transformed_timestamp)
                ],
                line_color.stroke_width(1),
            )))?;
        }
    }

    // Draw rectangles and text for each band using numeric coordinates
    for band in bands {
        if let Some(&stage_idx) = stage_to_index.get(&band.stage) {
            // Position rectangles so their centers align with the integer tick marks
            let x_center = stage_idx as f32;
            let x_start = x_center - rect_width / 2.0;
            let x_end = x_center + rect_width / 2.0;
            
            let start_time = band.start_dt.and_utc();
            let end_time = band.end_dt.and_utc();
            
            // Convert to numeric coordinates
            let start_timestamp = datetime_to_timestamp(start_time);
            let end_timestamp = datetime_to_timestamp(end_time);
            
            // Transform coordinates for inverted visual display
            let display_start_pos = transform_time(start_timestamp);
            let display_end_pos = transform_time(end_timestamp);
            
            // Draw the band rectangle with transformed coordinates
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x_start, display_start_pos), (x_end, display_end_pos)],
                BLUE.mix(0.7).filled(),
            )))?;
            
            // Calculate middle time for text
            let middle_timestamp = start_timestamp + (end_timestamp - start_timestamp) / 2.0;
            let display_middle = transform_time(middle_timestamp);
            
            // Draw band name at the center of the rectangle
            chart.draw_series(std::iter::once(Text::new(
                band.name.clone(),
                (x_center, display_middle),
                ("Arial", 10).into_font().color(&WHITE),
            )))?;
        }
    }

    // Ensure the chart is properly finalized
    drawing_area.present()?;
    
    Ok(())
}
