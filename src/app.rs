use ratatui::widgets::ListState;

pub enum Screen {
    Main,
}

pub struct List<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

pub struct App<T> {
    pub title: String,
    pub screen: Screen,
    pub list: List<T>,
}

impl<T> App<T>
where
    T: std::clone::Clone,
{
    pub fn new(title: &str, items: &[T]) -> Self {
        App {
            title: title.to_string(),
            screen: Screen::Main,
            list: List {
                items: items.to_vec(),
                state: ListState::default(),
            },
        }
    }
}
