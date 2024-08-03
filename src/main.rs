// Tim Lobner

use iced::{Sandbox, Settings};

mod band;
mod running_order_parser;
mod gui {
    pub mod message;
    pub mod pro_tool_state;
    pub mod main_view;
    pub mod band_selection_view;
}
mod timetable {
    pub mod table_creation;
}

use gui::pro_tool_state::ProToolState;

fn main() -> iced::Result{
    ProToolState::run(Settings::default())
}
