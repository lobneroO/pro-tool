// Tim Lobner

use iced::Settings;

mod band;
mod running_order_parser;
mod gui {
    pub mod message;
    pub mod pro_tool_state;
    pub mod main_view;
    pub mod band_selection_view;
}
mod timetable{
    pub mod table_creation;
}

use gui::pro_tool_state::ProToolState;
use plotters::prelude::*;

fn draw_test() {
    let mut backend = SVGBackend::new("test.svg", (1024, 768));
    _ = backend.draw_rect((10, 10), (25, 50), &RED, true);
    _ = backend.draw_rect((25, 10), (40, 50), &GREEN, true);
    let text_style = &("sans-serif", 20).into_text_style(&backend.get_size()).color(&BLACK);
    _ = backend.draw_text("Fleshgod\n Apocalypse", text_style, (25, 10));
    _ = backend.present();
}

fn main() {
    draw_test();
}
// fn main() -> iced::Result{
//     iced::application("Pro Tool", ProToolState::update, ProToolState::view)
//         .run()
// }
