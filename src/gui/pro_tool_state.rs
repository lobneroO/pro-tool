// Tim Lobner

use chrono::NaiveDate;
use iced::Element;
use rfd::FileDialog;
use std::path::Path;

use crate::gui::message::Message;
use crate::running_order_parser;
use crate::band::Band;

use super::main_view::get_main_view;
use super::band_selection_view::get_band_selection_view;
use super::super::timetable::table_creation;

#[derive(Default, PartialEq)]
enum View{
    #[default] Main,
    BandSelection,
}

// define a struct that contains _ALL_ of the program's state
#[derive(Default)]
pub struct ProToolState{
    view: View,
    pub running_order_file: String,
    pub running_order: Vec<Band>,
}

impl ProToolState {
    // Message is not necessarily a text,
    // it can also be a button press. 
    // anything that can change the state
    // type Message = Message;

    fn new() -> Self {
        Self{
            running_order_file: "".to_string(),
            view: View::Main,
            running_order: vec!(),
        }
    }

    pub fn update(& mut self, message: Message) {
        match message {
            Message::ChooseRunningOrderInput=> {
                // open a file chooser
                let file = FileDialog::new()
                    .add_filter("CSV", &["csv"])
                    .set_directory("~")
                    .pick_file();

                self.running_order_file = file.unwrap().into_os_string().to_str().unwrap().to_string();
            },
            Message::OnRunningOrderInputChanged(text) => {
                // update the text input
                self.running_order_file = text; 
            },
            Message::CreateCompleteRunningOrder => {
                self.running_order = running_order_parser::parse_running_order(Path::new(&self.running_order_file));
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
                ];
                table_creation::create_table(out_path, &bands, "Thursday");
            },
            Message::CreatePersonalRunningOrder => {
                self.running_order = running_order_parser::parse_running_order(Path::new(&self.running_order_file));
                self.view = View::BandSelection;
                println!("personal Running order");
            },
            Message::BandSelected(index, selected) => {
                println!("test {}: {}", self.running_order[index].name, selected);
                self.running_order[index].selected = selected;
            }
            Message::Back => {
                if self.view == View::BandSelection {
                   self.view = View::Main;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> { 
        match self.view {
            View::Main => {
                get_main_view(self)
            },
            View::BandSelection => {
                get_band_selection_view(self)
            },
        }
    }
}
