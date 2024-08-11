// Tim Lobner

use charming::{
    component::{Legend,
        Title, Axis},
    element::{ItemStyle,
        AxisType},
    // series::{Pie, PieRoseType},
    Chart, ImageRenderer
};
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

    // prepare the y axis. we don't have access to any sort of time(?), but
    // we can use strings...

    let chart = Chart::new()
        .title(Title::new().text("Sales Report"))
        .x_axis(Axis::new().data(stages))
        .y_axis(
            Axis::new()
                .type_(AxisType::Time)
                // .axis_label()
        );
        // .legend(Legend::new().data(vec!["coffee", "juice", "milk"]));
    // let chart = Chart::new()
    //     // .legend(Legend::new().top("bottom"))
    //     .series(
    //         Pie::new()
    //             .name("Nightingale Chart")
    //             .rose_type(PieRoseType::Radius)
    //             .radius(vec!["50", "250"])
    //             .center(vec!["50%", "50%"])
    //             .item_style(ItemStyle::new().border_radius(8))
    //             .data(vec![
    //                 (40.0, "rose 1"),
    //                 (38.0, "rose 2"),
    //                 (32.0, "rose 3"),
    //                 (30.0, "rose 4"),
    //                 (28.0, "rose 5"),
    //                 (26.0, "rose 6"),
    //                 (22.0, "rose 7"),
    //                 (18.0, "rose 8"),
    //             ]),
    //     );

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "test.svg").unwrap();
}
