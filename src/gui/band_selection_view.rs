
use iced::Element;
use iced::widget::{button, column, container, grid, space, row, scrollable};
use iced::Fill;

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;
use crate::band::{Band, BandMessage};

pub fn get_band_selection_view(state: &ProToolState) -> Element<Message> {
    // calculate the maximum width of a band selection (i.e. the box + the longest name)
    let full_slice = &state.running_order.as_slice();
    
    let band_auto_grid = grid(
        full_slice
            .iter()
            .map(Band::view)
            .enumerate()
            .map(|(index, band)| {
                #[allow(unused_variables)]
                band.map(move |message: BandMessage|
                    Message::BandSelected(index, !state.running_order[index].selected))
            })
            // fluid adjusts the amount of columns. max 123 is not a great value,
            // but in effect it says the user can pull the width of the window as far as they want,
            // without running into a limit
    )
        .fluid(123)
        .height(grid::Sizing::EvenlyDistribute(iced::Length::Shrink));

        // TODO: the following does all bands. I'll leave it here as a reminder of how it works
        // column(
        //     state.running_order
        //         // take all bands in the running order
        //         // (NOTE: if a band plays multiple times, it will be listed multiple times!)
        //         .iter()
        //         // call view on every iterator item. since this is implemented
        //         // for the Band struct, it will return the checkbox item in a row (TODO: checkbox
        //         // only suffices!)
        //         .map(Band::view)
        //         // get a tuple of the index of the iterator
        //         // and the element of the iterator
        //         .enumerate()
        //         // the checkbox will produce a band message,
        //         // which we have to consume here. this means:
        //         // turn the BandMessage into a (global) Message
        //         .map(|(index, band)| {
        //             // the message variable is unused, but the closure expects it, 
        //             // so we cannot remove it. instead, hide the warning.
        //             #[allow(unused_variables)]
        //             band.map(move |message: BandMessage|
        //                 // we cannot access "band" directly here, but we have the running order and
        //                 // index, so we stell get to it. set the selected member to its inverted
        //                 // state to make the checkbox a toggle
        //                 Message::BandSelected(index, !state.running_order[index].selected))
        //         })
        // )];
        // .push(checkboxes.iter());
    let back_button = button("Back")
        .on_press(Message::Back);
    let settings_button = button("Open Settings");
    let button_row = row![space::horizontal(), back_button, settings_button, space::horizontal()];
    container(scrollable(column![band_auto_grid, button_row])).padding(10).into()
}
