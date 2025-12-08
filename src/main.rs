// Tim Lobner

use chrono::NaiveDate;
// use iced::Settings;

use std::path::Path;

mod band;
mod running_order_parser;
mod gui {
    pub mod message;
    pub mod pro_tool_state;
    pub mod main_view;
    pub mod band_selection_view;
}
mod timetable{
    pub mod table_creation;
}

use crate::band::Band;
use crate::timetable::table_creation;
use gui::pro_tool_state::ProToolState;
// use plotters::prelude::*;

#[allow(dead_code)]
fn draw_test() {
    // let mut backend = SVGBackend::new("test.svg", (1024, 768));
    // _ = backend.draw_rect((10, 10), (25, 50), &RED, true);
    // _ = backend.draw_rect((25, 10), (40, 50), &GREEN, true);
    // let text_style = &("sans-serif", 20).into_text_style(&backend.get_size()).color(&BLACK);
    // _ = backend.draw_text("Fleshgod\n Apocalypse", text_style, (25, 10));
    // _ = backend.present();
    let out_path = Path::new("test.svg");
    // table_creation::create_table(out_path);
    let bands = vec![
        Band {
            name: String::from("Fleshgod Apocalypse"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(12, 0, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(13, 0, 0).unwrap(),
            stage: String::from("T-Stage"),
            selected: true,
        },
        Band {
            name: String::from("Anaal Nathrakh"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(13, 15, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(14, 0, 0).unwrap(),
            stage: String::from("T-Stage"),
            selected: true,
        },
        Band {
            name: String::from("Meshuggah"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(14, 30, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(15, 30, 0).unwrap(),
            stage: String::from("Main Stage"),
            selected: true,
        },
        Band {
            name: String::from("Heretics"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(14, 30, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(15, 30, 0).unwrap(),
            stage: String::from("Wera Tool Rebel Stage"),
            selected: true,
        },
        Band {
            name: String::from("In Sanity"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(11, 30, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 15).unwrap().and_hms_opt(12, 30, 0).unwrap(),
            stage: String::from("Wera Tool Rebel Stage"),
            selected: false,
        },

        Band {
            name: String::from("Kataklysm"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 16).unwrap().and_hms_opt(14, 30, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 16).unwrap().and_hms_opt(15, 30, 0).unwrap(),
            stage: String::from("T-Stage"),
            selected: true,
        },
        Band {
            name: String::from("Beyond Creation"),
            start_dt: NaiveDate::from_ymd_opt(2024, 8, 16).unwrap().and_hms_opt(11, 30, 0).unwrap(),
            end_dt: NaiveDate::from_ymd_opt(2024, 8, 16).unwrap().and_hms_opt(12, 30, 0).unwrap(),
            stage: String::from("Wera Tool Rebel Stage"),
            selected: true,
        }
    ];
    table_creation::create_table(out_path, &bands);
}

// fn main() {
//     draw_test();
// }
fn main() -> iced::Result{
    // iced::application("Pro Tool", ProToolState::update, ProToolState::view)
    iced::application(ProToolState::default, ProToolState::update, ProToolState::view)
        .title("Pro Tool")
        .run()
}
