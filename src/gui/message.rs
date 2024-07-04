// Tim Lobner

#[derive(Debug, Clone)]
pub enum Message{
    ChooseRunningOrderInput,
    OnRunningOrderInputChanged(String),
    CreateCompleteRunningOrder,
    CreatePersonalRunningOrder,
    // for band selection subview
    // first is the index within the running order, second the selection state
    BandSelected(usize, bool),
    // generic Back. may do different things,
    // depending on which view we're at
    Back,
}
