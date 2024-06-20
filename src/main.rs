// Tim Lobner

// #[path = "ui/main_window.rs"]mod main_window;
use iced::{Element, Settings, Sandbox};
use iced::widget::{button, column, container, horizontal_space, row, text_input};
use rfd::FileDialog;
use std::path::Path;

mod band;
mod running_order_parser;

// define a struct that contains _ALL_ of the program's state
#[derive(Default)]
struct ProToolState{
    running_order_file: String,
    create_full_button_state: bool,
}

#[derive(Debug, Clone)]
enum Message{
    ChooseRunningOrderInput,
    OnRunningOrderInputChanged(String),
    CreateCompleteRunningOrder,
    CreatePersonalRunningOrder,
}

impl Sandbox for ProToolState {
    // Message is not necessarily a text,
    // it can also be a button press. 
    // anything that can change the state
    type Message = Message;

    fn new() -> Self {
        Self{
            running_order_file: "".to_string(),
            create_full_button_state: false,
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
                self.create_full_button_state = Path::new(&self.running_order_file).is_file();
            },
            Message::OnRunningOrderInputChanged(text) => {
                // update the text input
                self.running_order_file = text; 
                self.create_full_button_state = Path::new(&self.running_order_file).is_file();
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

        // create a button for a complete running order and one for a personal running order
        // creation
        let create_full_button = button("Create Complete Running Order").on_press(Message::CreateCompleteRunningOrder);
        //button("Create Complete Running Order").on_press(Message::CreateCompleteRunningOrder);
        let create_personal_button = button("Create Personal Running Order").on_press_maybe(
            if self.create_full_button_state { Some(Message::CreatePersonalRunningOrder) } else { None });


        // create a button for settings
        let settings_button = button("Open Settings");

        let second_row = row![horizontal_space(), create_full_button, horizontal_space()];
        let third_row = row![horizontal_space(), create_personal_button, horizontal_space(), settings_button, horizontal_space()];

        container(column![choose_row, second_row, third_row]).padding(10).into()
    }
}

fn main() -> iced::Result{
    ProToolState::run(Settings::default())
}
