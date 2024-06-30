// Tim Lobner

#[derive(Debug, Clone)]
pub enum Message{
    ChooseRunningOrderInput,
    OnRunningOrderInputChanged(String),
    CreateCompleteRunningOrder,
    CreatePersonalRunningOrder,
    // for band selection subview
    BandSelected(bool),
    // generic Back. may do different things,
    // depending on which view we're at
    Back,
}
