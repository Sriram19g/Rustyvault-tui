use crate::app::ITEM_HEIGHT;

impl super::super::App {
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.credentials.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT)
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.credentials.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn delete_entry(&mut self) {
        if let Some(index) = self.state.selected() {
            if (index < self.credentials.len()) {
                self.credentials.remove(index);

                let new_selected = if index == self.credentials.len() {
                    if self.credentials.is_empty() {
                        None
                    } else {
                        Some(index.saturating_sub(1))
                    }
                } else {
                    Some(index)
                };
                self.state.select(new_selected);
            }
        }
    }
}
