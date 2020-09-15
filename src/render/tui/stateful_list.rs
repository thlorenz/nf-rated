use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn advance_by(&mut self, delta: i32) {
        let nitems = self.items.len() as i32;
        let i = match self.state.selected() {
            Some(i) => {
                if (i as i32) >= nitems - delta {
                    0
                } else {
                    i as i32 + delta
                }
            }
            None => 0,
        };
        let clamped = if i < 0 {
            (nitems + i).max(0)
        } else {
            if i > nitems {
                nitems - i
            } else {
                i
            }
        };
        self.state.select(Some(clamped as usize));
    }

    pub fn next(&mut self) {
        self.advance_by(1)
    }

    pub fn next_page(&mut self, height: i32) {
        self.advance_by(height)
    }

    pub fn previous(&mut self) {
        self.advance_by(-1)
    }

    pub fn previous_page(&mut self, height: i32) {
        self.advance_by(-height)
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
