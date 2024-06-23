// Tim Lobner

#[derive(Debug, Clone)]
pub enum Message{
    ChooseRunningOrderInput,
    OnRunningOrderInputChanged(String),
    CreateCompleteRunningOrder,
    CreatePersonalRunningOrder,
}
