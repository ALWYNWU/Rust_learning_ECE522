mod AVL_tree;
mod red_black_tree;

use red_black_tree::*;
use AVL_tree::*;

use std::io;

fn main() {
    println!("Welcome to the World of Red-Black Trees and AVL Trees!");
    let mut temp = String::new();
    let mut n: i32 = 0;
    loop {
        println!(
            "Please choose a type of tree that you want to explore: (input corresponding number)"
        );
        println!("1. Red-Black Tree");
        println!("2. AVL Tree");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line!");
        n = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0, // default value is 0
        };
        if n != 1 && n != 2 {
            println!("Invalid input!");
        } else {
            break;
        }
    }
    if n == 1 {
        // rb tree
        println!("Welcome to the World of Red-Black Trees!");
        let mut tree: RBTree<String> = RBTree::new();
        loop {
            // let mut input = String::new();
            temp.clear();
            println!("Please choose the operation you want: (input corresponding number)");
            println!("1. Insert a node to the red-black tree.");
            println!("2. Delete a node from the red-black tree.");
            println!("3. Count the number of leaves in a tree.");
            println!("4. Return the height of a tree.");
            println!("5. Print In-order traversal of the tree.");
            println!("6. Check if the tree is empty.");
            println!("7. Check if an element exists in the tree.");
            println!("8. Return the size (number of nodes) of the tree.");
            println!("9. Quit.");
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line!");
            n = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => 0, // default value is 0
            };
            println!("----------------------------------------");
            println!("");
            match n {
                1 => {
                    let mut line = String::new();
                    println!("Please input the value of the node that you want to insert:");
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Failed to read line!");
                    println!("");
                    if tree.insert(String::from(line.trim())) {
                        println!("Inserted successfully!");
                    } else {
                        println!("Inserted failed")
                    }
                }
                2 => {
                    let mut line = String::new();
                    println!("Please input the value of the node that you want to delete:");
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Failed to read line!");
                    if tree.delete(String::from(line.trim())) {
                        println!("Deleted successfully!");
                    } else {
                        println!("Failed to delete!");
                    }
                }
                3 => {
                    println!("The number of leaves is: {}", tree.get_number_of_leaves());
                }
                4 => {
                    println!("The height of the tree is: {}", tree.height());
                }
                5 => {
                    println!("In-order traversal: ");
                    tree.inorder_traversal();
                }
                6 => {
                    if tree.is_empty() {
                        println!("This tree is empty");
                    } else {
                        println!("This tree is not empty");
                    }
                }
                7 => {
                    let mut line = String::new();
                    println!("Please input the value that you want to search: ");
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Failed to read line!");
                    if tree.search_node(String::from(line.trim())).is_some() {
                        println!("This value exists in the tree.");
                    } else {
                        println!("This value does not exist in the tree.");
                    }
                }
                8 => {
                    println!("The size of the tree is: {}", tree.count());
                }
                9 => {
                    println!("See you next time!");
                    break;
                }
                _ => {
                    println!("Invalid input!");
                }
            }
            println!("");
            println!("----------------------------------------");
        }
    } else {
        // AVL tree
        println!("Welcome to the World of AVL Trees!");
        let mut set = AvlTree::new();
        loop {
            temp.clear();
            println!("Please choose the operation you want: (input corresponding number)");
            println!("1. Insert a node to the red-black tree.");
            println!("2. Delete a node from the red-black tree.");
            println!("3. Count the number of leaves in a tree.");
            println!("4. Return the height of a tree.");
            println!("5. Print In-order traversal of the tree.");
            println!("6. Check if the tree is empty.");
            println!("7. Quit.");
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line!");
            n = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => 0, // default value is 0
            };
            println!("----------------------------------------");
            println!("");
            match n {
                1 => {
                    let mut line = String::new();
                    println!("Please input the value of the node that you want to insert:");
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Failed to read line!");
                    println!("");
                    set.Insert(String::from(line.trim()));
                    println!("Inserted successfully!");
                }
                2 => {
                    let mut line = String::new();
                    println!("Please input the value of the node that you want to delete:");
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Failed to read line!");
                    println!("");
                    set.Delete(&String::from(line.trim()));
                    println!("Deleted successfully!");
                }
                3 => {
                    println!(
                        "The number of leaves is: {}",
                        set.0.as_ref().unwrap().count_leaves()
                    );
                }
                4 => {
                    println!("The height of the tree is: {}", set.tree_height());
                }
                5 => {
                    println!("In-order traversal: ");
                    set.print_in_order(0);
                }
                6 => {
                    if set.check_empty() {
                        println!("This tree is empty");
                    } else {
                        println!("This tree is not empty");
                    }
                }
                7 => {
                    println!("See you next time!");
                    break;
                }
                _ => {
                    println!("Invalid input!");
                }
            }
            println!("");
            println!("----------------------------------------");
        }
    }
}
