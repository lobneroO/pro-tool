// Tim Lobner

use plotpy::*;

use chrono::NaiveDate;

use crate::band::Band;

pub fn create_running_order() {

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

    // actual plotting code
    let mut plot = Plot::new();


    // disable the plot's axes, they will be added in subplots
    plot.set_hide_axes(true);

    // let fig = plot.figure(figsize=(11.69, 8.27));
    // this is a bit hacky, but it gives out the correct time stamps for the wacken 2023 example
    // and should work as long as the y_lim is kept to 27.3 - 10.9
    let hours = vec!("10:00", "12:00", "14:00", "16:00", "18:00", "20:00", "22:00", "0:00", "2:00");

    // for readability of the resulting plot, offset the x position by 0.5
    let x_offset_axis = 0.5;
    // set axes (for bottom left first, then mirror for upper right)
    plot.set_subplot(0, 0, 0)
        .set_num_ticks_x(stages.len())
        .set_num_ticks_y(hours.len());
    // axis_bl.yaxis.grid();
    // axis_bl.set_xlim(x_offset_axis, stages.len() + x_offset_axis);
    // it will be read downwards, therefore invert the time labels on the axis
    // axis_bl.set_ylim(27.3, 10.9);
    // axis_bl.set_xticks(range(1, stages.len() + 1));
    // axis_bl.set_xticklabels(stages, rotation=30);
    // axis_bl.set_ylabel('Time');
    // axis_bl.set_yticklabels(hours);

    // let axis_ur = axis_bl.twiny().twinx();
    // axis_ur.set_xlim(axis_bl.get_xlim());
    // axis_ur.set_ylim(axis_bl.get_ylim());
    // axis_ur.set_xticks(axis_bl.get_xticks());
    // TODO: rotation doesn't work here, for whatever reason
    // axis_ur.set_xticklabels(axis_bl.get_xticklabels());
    // axis_ur.set_ylabel('Time');
    // axis_ur.set_yticklabels(axis_bl.get_yticklabels());

    println!("printing running order");
}
