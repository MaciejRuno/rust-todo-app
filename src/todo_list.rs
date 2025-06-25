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
            Self::Container { items, text: _ } => {
                items.push(item);
            }
            Self::Item { mark: _, text } => {
                *self = Self::new(text.to_string());

                self.add_item(item);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_item_turns_item_into_container_and_adds_child() {
        let mut item = TodoList::Item {
            mark: false,
            text: "parent".to_string(),
        };

        item.add_item(TodoList::Item {
            mark: false,
            text: "child".to_string(),
        });

        match item {
            TodoList::Container { ref items, ref text } => {
                assert_eq!(text, "parent");
                assert_eq!(items.len(), 1);
                match items[0] {
                    TodoList::Item { ref text, mark } => {
                        assert_eq!(text, "child");
                        assert!(!mark);
                    }
                    _ => panic!("child should remain item"),
                }
            }
            _ => panic!("item should be converted to container"),
        }
    }

    #[test]
    fn get_index_returns_correct_nested_item() {
        let mut root = TodoList::new("root".into());
        root.add_item(TodoList::Item {
            mark: false,
            text: "first".into(),
        });
        root.add_item(TodoList::Item {
            mark: false,
            text: "second".into(),
        });

        if let TodoList::Container { ref mut items, .. } = root {
            items[0].add_item(TodoList::Item {
                mark: false,
                text: "nested".into(),
            });
        }

        // pre-order indices: 0 root, 1 first, 2 nested, 3 second
        let elem = root.get_index(2);
        match elem {
            TodoList::Item { text, .. } => assert_eq!(text, "nested"),
            _ => panic!("wrong element returned"),
        }
    }

    #[test]
    fn mark_recursively_updates_children() {
        let mut root = TodoList::new("root".into());
        root.add_item(TodoList::Item {
            mark: false,
            text: "first".into(),
        });

        root.add_item(TodoList::Item {
            mark: false,
            text: "second".into(),
        });

        // mark whole list as done
        root.mark(true);

        if let TodoList::Container { items, .. } = root {
            for item in items {
                match item {
                    TodoList::Item { mark, .. } => assert!(mark),
                    _ => (),
                }
            }
        } else {
            panic!("root should be a container");
        }
    }
}
