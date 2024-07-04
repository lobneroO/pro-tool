// Tim Lobner

use chrono::NaiveDateTime;
use iced::Element;
use iced::widget::{container, row, checkbox};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BandMessage {
    Toggled(bool),
}

/// struct to combine a bands information
pub struct Band{
    pub name: String,
    pub start_dt: NaiveDateTime, 
    pub end_dt: NaiveDateTime, 
    pub stage: String,
    pub selected: bool,
}

impl Band{
    pub fn view(&self) -> Element<BandMessage> {
        let r = row![
            checkbox(self.name.clone(), self.selected)
                .on_toggle(BandMessage::Toggled)
        ];

        container(r).into()
    }

    pub fn update(&mut self, message: BandMessage) {
        match message {
            BandMessage::Toggled(selected) => {
                self.selected = selected;
            }
        }
    }
}
