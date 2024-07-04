
use iced::Element;
use iced::widget::{button, column, container, horizontal_space, row, checkbox};

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;
use crate::band::{Band, BandMessage};

pub fn get_band_selection_view(state: &ProToolState) -> Element<Message> {
    // for every band, add a checkbox
    // let mut checkboxes: Vec<iced::widget::Checkbox<_>> = vec!();

    // for band in &state.running_order {
    //     let checkbox = checkbox(band.name.clone(), band.selected)
    //         .on_toggle(Message::BandSelected);
    //     checkboxes.push(checkbox);
    // }

    let band_grid = row![
        row(
            state.running_order
                // take all bands in the running order
                // (NOTE: if a band plays multiple times, it will be listed multiple times!)
                .iter()
                // call view on every iterator item. since this is implemented
                // for the Band struct, it will return the checkbox item in a row (TODO: checkbox
                // only suffices!)
                .map(Band::view)
                // get a tuple of the index of the iterator
                // and the element of the iterator
                .enumerate()
                // the checkbox will produce a band message,
                // which we have to consume here. this means:
                // turn the BandMessage into a (global) Message
                .map(|(index, band)| {
                    band.map(move |message| 
                        Message::BandSelected(index, message == BandMessage::Selected))
                })
        )];
        // .push(checkboxes.iter());
    let back_button = button("Back")
        .on_press(Message::Back);
    let settings_button = button("Open Settings");
    let third_row = row![horizontal_space(), back_button, settings_button, horizontal_space()];
    container(column![band_grid, third_row]).padding(10).into()
}
