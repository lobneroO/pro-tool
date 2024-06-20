// Tim Lobner

use iced::{Element, Sandbox};
use iced::widget::{button, column, container, horizontal_space, row, text_input};
use rfd::FileDialog;
use std::path::Path;

use crate::gui::message::Message;
use crate::running_order_parser;

// define a struct that contains _ALL_ of the program's state
#[derive(Default)]
pub struct ProToolState{
    running_order_file: String,
}


impl Sandbox for ProToolState {
    // Message is not necessarily a text,
    // it can also be a button press. 
    // anything that can change the state
    type Message = Message;

    fn new() -> Self {
        Self{
            running_order_file: "".to_string(),
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
                println!("personal Running order");
            }
        }
    }

    fn view(&self) -> Element<'_, Message> { 
        // add a text field for the input file path
        let running_order_input = text_input("", &self.running_order_file)
            .padding(10)
            .on_input(Message::OnRunningOrderInputChanged);

        // button to open a file chooser
        let running_order_file_chooser = button("...").on_press(Message::ChooseRunningOrderInput);

        // put into a row layout
        let choose_row = row![horizontal_space(), running_order_input, horizontal_space(), running_order_file_chooser, horizontal_space()];

        // create a button for a complete running order 
        // and one for a personal running order creation
        let buttons_active = Path::new(&self.running_order_file).is_file();
        let create_full_button = button("Create Complete Running Order").on_press_maybe(
            if buttons_active {
                Some(Message::CreateCompleteRunningOrder)
            } else { None });
        //button("Create Complete Running Order").on_press(Message::CreateCompleteRunningOrder);
        let create_personal_button = button("Create Personal Running Order").on_press_maybe(
            if buttons_active {
                Some(Message::CreatePersonalRunningOrder)
            } else { None });

        // create a button for settings
        let settings_button = button("Open Settings");

        let second_row = row![horizontal_space(), create_full_button, horizontal_space()];
        let third_row = row![horizontal_space(), create_personal_button, horizontal_space(), settings_button, horizontal_space()];

        container(column![choose_row, second_row, third_row]).padding(10).into()
    }
}
