// Tim Lobner

use iced::{Element, Sandbox};
use rfd::FileDialog;
use std::path::Path;

use crate::gui::message::Message;
use crate::running_order_parser;

use super::main_view::get_main_view;
use super::band_selection_view::get_band_selection_view;

#[derive(Default)]
enum View{
    #[default] Main,
    BandSelection,
}

// define a struct that contains _ALL_ of the program's state
#[derive(Default)]
pub struct ProToolState{
    view: View,
    pub running_order_file: String,
}

impl Sandbox for ProToolState {
    // Message is not necessarily a text,
    // it can also be a button press. 
    // anything that can change the state
    type Message = Message;

    fn new() -> Self {
        Self{
            running_order_file: "".to_string(),
            view: View::Main,
        }
    }

    fn title(&self) -> String {
        String::from("pro tool")
    }

    fn update(& mut self, message: Message) {
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
                let _ = running_order_parser::parse_running_order(Path::new(&self.running_order_file));
            },
            Message::CreatePersonalRunningOrder => {
                let _ = running_order_parser::parse_running_order(Path::new(&self.running_order_file));
                self.view = View::BandSelection;
                println!("personal Running order");
            }
        }
    }

    fn view(&self) -> Element<'_, Message> { 
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
