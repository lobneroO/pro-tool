
use iced::Element;
use iced::widget::{button, column, container, horizontal_space, row, checkbox};
use std::path::Path;

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;
use crate::band::Band;

pub fn get_band_selection_view(state: &ProToolState) -> Element<Message> {
    // for every band, add a checkbox
    let mut checkboxes: Vec<iced::widget::Checkbox<_>> = vec!();

    for band in &state.running_order {
        let checkbox = checkbox(band.name.clone(), band.selected)
            .on_toggle(Message::BandSelected);
        checkboxes.push(checkbox);
    }

    let band_grid = row![
        row(
            state.running_order
                .iter()
                .map(Band::view)
                .enumerate()
                .map(|(name, band)| {
                    band.map(move |message| Message::BandSelected(true))
                })
        )];
        // .push(checkboxes.iter());
    let back_button = button("Back")
        .on_press(Message::Back);
    let settings_button = button("Open Settings");
    let third_row = row![horizontal_space(), back_button, settings_button, horizontal_space()];
    container(column![band_grid, third_row]).padding(10).into()
}
