use std::io;

fn main() {
    let mut products: Vec<(u32, String, u32)> = Vec::new();

    loop {
        println!("Warehouse Inventory Management:");
        println!("1. Add New Product");
        println!("2. Update Stock Quantity");
        println!("3. Remove Product");
        println!("4. List All Products");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to Read");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------Invalid Input. Please Enter a number------------");
                return();
            }
        };

        if choice == 1 {
            // Add new product
            let mut input = String::new();

            // ID input
            println!("Enter Product ID");
            io::stdin().read_line(&mut input).expect("Failed to Read");
            let id: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("------------Invalid ID------------");
                    continue;
                }
            };
            input.clear();

            // Name input
            println!("Enter Product Name");
            io::stdin().read_line(&mut input).expect("Failed to Read");
            let name:String = input.trim().to_string();
            input.clear();

            // Quantity input
            println!("Enter Product quantity");
            io::stdin().read_line(&mut input).expect("Failed to Read");
            let quantity: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("------------Invalid quantity------------");
                    continue;
                }
            };
            input.clear();

            // Check if product already exists
            let mut id_exists = false;
            for (existing_id, _, _) in &products {
                if *existing_id == id {
                    id_exists = true;
                    break;
                }
            }

            if id_exists {
                println!("------------Product ID already exists.------------");
                continue;
            } else {
                // Push product into products
                products.push((id, name, quantity));
                println!("------------Product added successfully.------------");
            }

            // Continue showing the menu
            continue;
        } else if choice == 2 {
            // Update Stock Quantity
            let mut input = String::new();
            println!("Product ID to update stock quantity");
            io::stdin().read_line(&mut input).expect("Failed to Read");

            let id_change: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };

            let mut id_exists = false;

            for (existing_id, _, existing_quantity) in &mut products {
                if *existing_id == id_change {
                    id_exists = true;
                }

                if id_exists {
                    let mut input = String::new();
                    println!("Enter the updated quantity: ");
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    let quantity: u32 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid quantity.");
                            continue;
                        }
                    };
                        *existing_quantity = quantity;
                }
            }
            if !(id_exists) {
                println!("Product not Found!");
                continue;
            }
 
        } else if choice == 3 {
            // Remove Product
            let mut input = String::new();
            println!("Product ID to update stock quantity");
            io::stdin().read_line(&mut input).expect("Failed to Read");

            let id: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };

            let mut id_exists = false;

            for (existing_id, _, _) in &products {
                if *existing_id == id {
                    id_exists = true;
                }
            }

            if id_exists {
                let mut i = 0; 
                while i < products.len() {
                    if products[i].0 == id {
                        products.remove(i);
                        println!("Product removed successfully.");
                        break;
                    } else {
                        i += 1;
                    }
                }

            } else {
                println!("Product not Found!");
                continue;
            }
            
        } else if choice == 4 {
            // List all Products
            println!("--------Product list--------");

            for k in &products {
                println!("{:?}", k);
            }

        } else if choice == 5 {
            // Exit
            break;
        } else {
            println!("------------Please Enter a valid number------------");
            continue;
        }
    }
}
