use bits_data::Show;

pub enum Event {
    ShowCreated(ShowCreated),
}

pub struct ShowCreated {
    pub show: Show,
}
