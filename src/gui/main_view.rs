
use iced::Element;
use iced::widget::{button, column, container, horizontal_space, row, text_input};
use std::path::Path;

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;

pub fn get_main_view(state: &ProToolState) -> Element<Message> { 
    // add a text field for the input file path
    let running_order_input = text_input("", &state.running_order_file)
        .padding(10)
        .on_input(Message::OnRunningOrderInputChanged);

    // button to open a file chooser
    let running_order_file_chooser = button("...").on_press(Message::ChooseRunningOrderInput);

    // put into a row layout
    let choose_row = row![horizontal_space(), horizontal_space(), 
        running_order_input, horizontal_space(), running_order_file_chooser, 
        horizontal_space(), horizontal_space()];

    // create a button for a complete running order 
    // and one for a personal running order creation
    let buttons_active = Path::new(&state.running_order_file).is_file();
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

    let second_row = row![horizontal_space(), horizontal_space(), 
        create_full_button, horizontal_space(), create_personal_button,
        horizontal_space(), horizontal_space()];
    let third_row = row![horizontal_space(), settings_button, horizontal_space()];

    container(column![choose_row, second_row, third_row]).padding(10).into()
}

