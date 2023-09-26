use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub(crate) state: ListState,
    pub(crate) items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self, increment: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + increment
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, increment: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - increment
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn to_first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn to_last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}
