use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TodoList {
    Container { items: Vec<TodoList>, text: String },
    Item { mark: bool, text: String },
}

impl TodoList {
    pub fn new(name: String) -> Self {
        Self::Container {
            items: Vec::new(),
            text: name,
        }
    }

    pub fn add_item(&mut self, item: Self) {
        match self {
            Self::Container { items, .. } => {
                items.push(item);
            }
            Self::Item { mark, text } => {
                let original_mark = *mark;
                let original_text = text.clone();

                *self = Self::new(text.clone());

                if let Self::Container { items, .. } = self {
                    items.push(Self::Item {
                        mark: original_mark,
                        text: original_text,
                    });

                    items.push(item);
                }
            }
        }
    }

    pub fn get_index(&mut self, index: usize) -> &mut TodoList {
        self._get_index(index, &mut 0).unwrap()
    }

    fn _get_index(&mut self, index: usize, i: &mut usize) -> Option<&mut TodoList> {
        if index == *i {
            return Some(self);
        }
        *i += 1;

        match self {
            Self::Container { items, text: _ } => {
                for item in items {
                    match item._get_index(index, i) {
                        Some(a) => {
                            return Some(a);
                        }
                        None => {
                            continue;
                        }
                    }
                }
                None
            }
            Self::Item { mark: _, text: _ } => None,
        }
    }

    pub fn print(&self) {
        self._print(0, &mut -1)
    }

    fn _print(&self, tab_lvl: usize, i: &mut i32) {
        let tabs = String::from_utf8(vec![b' '; tab_lvl]).unwrap();

        *i += 1;

        match self {
            Self::Container { items, text } => {
                println!("{tabs}{i}.{text}:");
                for item in items {
                    item._print(tab_lvl + 4, i);
                }
            }
            Self::Item { mark, text } => {
                println!("{tabs}{i}.{text} {}", if *mark { "X" } else { "_" });
            }
        }
    }

    pub fn mark(&mut self, new_mark: bool) {
        match self {
            Self::Item { mark, text: _ } => *mark = new_mark,
            Self::Container { items, text: _ } => {
                for item in items {
                    item.mark(new_mark);
                }
            }
        }
    }
}
