
use iced::Element;
use iced::widget::{button, column, container, horizontal_space, row, text_input};
use std::path::Path;

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;

pub fn get_band_selection_view(state: &ProToolState) -> Element<Message> {

    let settings_button = button("Open Settings");
    let third_row = row![horizontal_space(), settings_button, horizontal_space()];
    container(column![third_row]).padding(10).into()
}
