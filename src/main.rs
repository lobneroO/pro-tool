// Tim Lobner

// #[path = "ui/main_window.rs"]mod main_window;
use iced::{Element, Settings, Sandbox};
use iced::widget::{button, text, column, row, container};
use rfd::FileDialog;

// define a struct that contains _ALL_ of the program's state
#[derive(Default)]
struct ProToolState{
    running_order_file: String,
}

#[derive(Debug, Clone, Copy)]
enum Message{
    ChooseRunningOrderInput,
}

impl Sandbox for ProToolState {
    // Message is not necessarily a text,
    // it can also be a button press. 
    // anything that can change the state
    type Message = Message;

    fn new() -> Self {
        Self{
            running_order_file: "/path/to/file".to_string(),
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
            }
        }
    }


    fn view(&self) -> Element<'_, Message> { 
        // add a text field for the input file path
        let running_order_input = text(self.running_order_file.to_string());

        // button to open a file chooser
        let running_order_file_chooser = button("...").on_press(Message::ChooseRunningOrderInput);

        // put into a row layout
        let choose_row = row![running_order_input, running_order_file_chooser];

        container(column![choose_row]).padding(10).into()
    }
}

fn main() -> iced::Result{
    ProToolState::run(Settings::default())
}
