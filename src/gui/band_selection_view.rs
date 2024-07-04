
use iced::Element;
use iced::widget::{button, column, container, horizontal_space, row, scrollable};

use crate::gui::message::Message;
use crate::gui::pro_tool_state::ProToolState;
use crate::band::{Band, BandMessage};

pub fn get_band_selection_view(state: &ProToolState) -> Element<Message> {
    // for every band, add a checkbox
    // we want to have multiple columns. assume 3 (TODO: make variable, based on window width!)
    // the sorting is: first column starting with lowest name (e.g. aborted, amon amarth, ...)
    // and so forth until the column is done and the next one starts. then go down again with the 
    // current name and onwards (e.g. fleshgod apocalypse, gojira, hammerfall)
    //
    // we can savely assume that state.running_order is sorted by bandname (TOOD: what about
    // multiple occurences of one band on a festival?).
    // therefore, make three slices, one for each column, that are roughly equal in size.
    let num_slices = 3;
    let num_entries = state.running_order.len();
    let slice_size = num_entries / num_slices;
    // if the number of bands is not divisible by three, then the slices will need to add one band
    // to their list.
    // len() % 3 = 0 -> divisible by three, all equal
    // len() % 3 = 1 -> first slice +1, all other equal
    // len() % 3 = 2 -> first and second slice + 1, last as is

    // hardcoded slices won't work if num_slices changes...
    assert!(num_slices == 3);
    let first_slice_size = if num_entries % num_slices > 0 { slice_size + 1 } else { slice_size };
    let second_slice_size = if num_entries % num_slices > 1 { slice_size + 1 } else { slice_size };

    let first_slice = &state.running_order[0..first_slice_size];
    // note that the ranges are end-exclusive and slices 0 based.
    // therefore second slice goes from first_slice_size to third_slice_start
    let third_slice_start = first_slice_size + second_slice_size;
    let second_slice = &state.running_order[first_slice_size..third_slice_start];
    let third_slice = &state.running_order[third_slice_start..];

    let band_grid = row![
        column(
            first_slice
                .iter()
                .map(Band::view)
                .enumerate()
                .map(|(index, band)| {
                    #[allow(unused_variables)]
                    band.map(move |message: BandMessage|
                        Message::BandSelected(index, !state.running_order[index].selected))
                })
        ),
        column(
            second_slice
                .iter()
                .map(Band::view)
                .enumerate()
                .map(|(index, band)| {
                    #[allow(unused_variables)]
                    band.map(move |message: BandMessage|
                        Message::BandSelected(index + first_slice_size, !state.running_order[index + first_slice_size].selected))
                })
        ),
        column(
            third_slice 
                .iter()
                .map(Band::view)
                .enumerate()
                .map(|(index, band)| {
                    #[allow(unused_variables)]
                    band.map(move |message: BandMessage|
                        Message::BandSelected(index + third_slice_start, !state.running_order[index + third_slice_start].selected))
                })
        )
    ];
    let scroll_band_grid = scrollable(band_grid);

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
    let third_row = row![horizontal_space(), back_button, settings_button, horizontal_space()];
    container(column![scroll_band_grid, third_row]).padding(10).into()
}
