use std::collections::HashMap;


pub trait StoreFunctions {
    fn add_item(&mut self, item: &str, quantity: i32);
    fn remove_item(&mut self, item: &str);
    fn get_item(&mut self, item: &str) -> String;
    fn add_item_count(&mut self, item: &str, quantity: i32);
    fn remove_item_count(&mut self, item: &str, quantity: i32);
    fn itemList(&mut self);
    fn get_item_list(&mut self);
}

pub struct Store {
    pub(crate) items: HashMap<String, i32>,
    pub(crate) item_list: Vec<String>,
}

impl StoreFunctions for Store {
    fn itemList(&mut self) {
        self.item_list = vec![
            "Apple".to_string(),
            "Peach".to_string(),
            "Banana".to_string(),
            "Orange".to_string(),
            "Grapes".to_string(),
            "Watermelon".to_string(),
            "Mango".to_string(),
            "Pineapple".to_string(),
            "Strawberry".to_string(),
            "Cherry".to_string(),
        ];
        for (index, value) in self.item_list.iter().enumerate() {
            self.items.insert(value.to_string(), 0);
        }
    }
    fn add_item(&mut self, item: &str, quantity: i32) {
        self.item_list.push(item.to_string());
        self.items.insert(item.to_string(), quantity);
    }

    fn add_item_count(&mut self, item: &str, quantity: i32) {
        let mut index = 0;
        let current_count: i32 = *self.items.get(item).unwrap();
        let new_count = current_count + quantity;
        while index < self.item_list.len() {
            if self.item_list[index] == item {
                self.items.insert(item.to_string(), new_count);
                break;
            } else {
                index += 1;
            }
        }
    }

    fn remove_item(&mut self, item: &str) {
        let mut index = 0;
        let is_item_available = self.items.contains_key(item);

        if { is_item_available==false} {
            println!("Item not found");
        } else {
            while index < self.item_list.len() {
                if self.item_list[index] == item {
                    self.item_list.remove(index);
                    break;
                } else {
                    index += 1;
                }
            }
        }
    }

    fn get_item(&mut self, item: &str) -> String {
        let mut index = 0;
        let text = String::from("Item not found");
        while index < self.item_list.len() {
            if self.item_list[index] == item {
                let quantity = self.items.get(item);
                println!("{}: {:?}", item, quantity);
            } else {
                index += 1;
            }
        }

        return text;
    }

    fn get_item_list(&mut self) {
        for (index, value) in self.item_list.iter().enumerate() {
            let count = self.items.get(value);

            println!("{}: {:?}, count: {}", index + 1, value, count.unwrap());
        }
    }

    fn remove_item_count(&mut self, item: &str, quantity: i32) {
        let mut index = 0;
        let current_count: i32 = *self.items.get(item).unwrap();
        let new_count = current_count - quantity;
        if new_count < 0 {
            println!("Quantity cannot be less than 0");
            return;
        }
        else {
            while index < self.item_list.len() {
                if self.item_list[index] == item {
                    self.items.insert(item.to_string(), new_count);
                    break;
                } else {
                    index += 1;
                }
            }
        }
        
    }
}
