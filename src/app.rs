pub enum CurrentScreen {
    Inbox,
    Compose,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Inbox,
        }
    }
}
