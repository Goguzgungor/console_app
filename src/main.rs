pub mod store_functions;

use std::collections::HashMap;
use std::io;

use store_functions::{Store, StoreFunctions};

fn main() {
    // Implement authentication
    if !authenticate() {
        println!("Authentication failed. Exiting the program.");
        return;
    }

    let mut store = Store {
        items: HashMap::new(),
        item_list: Vec::new(),
    };

    // Initialize the store
    store.itemList();

    // Perform operations based on user input
    loop {
        println!("Please choose an operation:");
        println!("1. Add item");
        println!("2. Add item count");
        println!("3. Remove item count");
        println!("4. Remove item");
        println!("5. Get item list");
        println!("6. Exit");

        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        match operation.trim().parse::<u32>() {
            Ok(1) => {
                println!("Enter item name:");
                let mut item_name = String::new();
                io::stdin()
                    .read_line(&mut item_name)
                    .expect("Failed to read line");
                item_name = item_name.trim().to_string();

                println!("Enter item quantity:");
                let mut quantity = String::new();
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity = quantity.trim().parse::<i32>().unwrap_or(0);

                store.add_item(&item_name, quantity);
            }
            Ok(2) => {
                println!("Enter item name:");
                let mut item_name = String::new();
                io::stdin()
                    .read_line(&mut item_name)
                    .expect("Failed to read line");
                item_name = item_name.trim().to_string();

                println!("Enter quantity to add:");
                let mut quantity = String::new();
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity = quantity.trim().parse::<i32>().unwrap_or(0);

                store.add_item_count(&item_name, quantity);
            }
            Ok(3) => {
                println!("Enter item name:");
                let mut item_name = String::new();
                io::stdin()
                    .read_line(&mut item_name)
                    .expect("Failed to read line");
                item_name = item_name.trim().to_string();

                println!("Enter quantity to remove:");
                let mut quantity = String::new();
                io::stdin()
                    .read_line(&mut quantity)
                    .expect("Failed to read line");
                let quantity = quantity.trim().parse::<i32>().unwrap_or(0);

                store.remove_item_count(&item_name, quantity);
            }
            Ok(4) => {
                println!("Enter item name:");
                let mut item_name = String::new();
                io::stdin()
                    .read_line(&mut item_name)
                    .expect("Failed to read line");
                item_name = item_name.trim().to_string();

                store.remove_item(&item_name);
            }
            Ok(5) => {
                store.get_item_list();
            }
            Ok(6) => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid operation. Please try again.");
            }
        }
    }
}

// Implement authentication logic here
fn authenticate() -> bool {
    // Prompt for username
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim();

    // Prompt for password
    println!("Enter your password:");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim();

    // Check if the entered credentials match the expected values
    username == "goktug" && password == "1234"
}