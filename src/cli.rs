use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use trees::base::QueryableTree;

use std::io::{stdin, stdout, Write};


fn avl_cli() {
    println!("\n::...AVL Tree branch...::\n");
    let mut tree = AVLTree::<i32>::new();
    list_of_operations();

    loop {
        print!("operation > ");
        let operation = get_user_input();

        match operation.to_lowercase().trim() {
            "insert"  => {
                let val = get_val("insert");
                tree.insert(val);
            },
            "delete" => {
                let val = get_val("delete");
                tree.delete(val);
            },

            "contain" | "search" => {
                let val = get_val("search");
                println!("values found? {:?}", tree.contains(val));
            },
            "height" => println!("Height of tree: {:?}", tree.height()),
            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "length" => println!("Length: {:?}", tree.len()),
            "min" => {
                let min_val = tree.min();
                match min_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Minimum Value: {:?}", v),
                }
            },
            "max" => {
                let max_val = tree.max();
                match max_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Maximum Value: {:?}", v),
                }
            },
            "empty" => println!("Is the tree empty?: {:?}", tree.is_empty()),
            "print" => {print!("Your tree: ");
                tree.print_inorder();},
            "help" => list_of_operations(),
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}


fn rbt_cli() {
    println!("\n::...Red-Black Tree branch...::\n");
    let mut tree = RedBlackTree::<i32>::new();
    list_of_operations();

    loop {
        print!("operation > ");
        let operation = get_user_input();

        match operation.to_lowercase().trim() {
            "insert"  => {
                let val = get_val("insert");
                tree.insert(val);
            },
            "delete" => {
                let val = get_val("delete");
                tree.delete(val);
            },

            "contain" | "search" => {
                let val = get_val("search");
                println!("values found? {:?}", tree.contains(val));
            },
            "height" => println!("Height of tree: {:?}", tree.height()),
            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "length" => println!("Length: {:?}", tree.len()),
            "min" => {
                let min_val = tree.min();
                match min_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Minimum Value: {:?}", v),
                }
            },
            "max" => {
                let max_val = tree.max();
                match max_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Maximum Value: {:?}", v),
                }
            },
            "empty" => println!("Is the tree empty?: {:?}", tree.is_empty()),
            "print" => {print!("Your tree: ");
                tree.print_inorder();},
            "help" => list_of_operations(),
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}


fn bst_cli() {
    println!("\n::...Binary-Search Tree branch...::\n");
    let mut tree = BinarySearchTree::<i32>::new();
    list_of_operations();

    loop {
        print!("operation > ");
        let operation = get_user_input();

        match operation.to_lowercase().trim() {
            "insert"  => {
                let val = get_val("insert");
                tree.insert(val);
            },
            "delete" => {
                let val = get_val("delete");
                tree.delete(val);
            },

            "contain" | "search" => {
                let val = get_val("search");
                println!("values found? {:?}", tree.contains(val));
            },
            "height" => println!("Height of tree: {:?}", tree.height()),
            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "length" => println!("Length: {:?}", tree.len()),
            "min" => {
                let min_val = tree.min();
                match min_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Minimum Value: {:?}", v),
                }
            },
            "max" => {
                let max_val = tree.max();
                match max_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Maximum Value: {:?}", v),
                }
            },
            "empty" => println!("Is the tree empty?: {:?}", tree.is_empty()),
            "print" => {print!("Your tree: ");
                tree.print_inorder();},
            "help" => list_of_operations(),
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}


pub fn run_cli(){
    loop {
        println!("you can select a tree to start or print 'exit' to leave");
        println!("Select a tree!\n-AVL \n-BST \n-RBT or type 'help' to learn about the commands");
        print!("input > ");
        let selected_tree = get_user_input();

        match selected_tree.to_lowercase().trim() {
            "avl" => {
                avl_cli();
            },
            "rbt" => {
                rbt_cli();
            },
            "bst" => {
                bst_cli();
            },
            "help" => {
                println!("Available commands:\n------------------ \n");
                println!("-exit - Exit the program.");
                println!("-rbt  - Tests RBT.");
                println!("-avl  - Tests AVL.");
                println!("-bst  - Tests BST.");
                println!("-help - Show available commands.\n");
            },
            "exit" => break,
            _ => {
                eprint!("Command not recognized. \n");
            }
        }
    }
}


pub fn get_user_input() -> String {
    let mut line = String::new();
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut line).expect("failed to read from stdin");
    line.to_string()
}
pub fn get_val(op: &str)-> i32 {
    loop {
        print!("{} value > ", op);
        let value = get_user_input();
        let trimmed_val = value.trim();
        match trimmed_val.parse::<i32>(){
            Ok(val) => {
                println!("{} value '{}' in tree ... done!", op, val);
                return val;
            },
            Err(..) => {
                println!("this was not an integer number");
            },
        };
    }
}

pub fn list_of_operations(){
    println!("\nAvailabe operations: \n------------------ \n");
    println!("-insert  - insert node into the tree.");
    println!("-delete  - delete node from the tree.");
    println!("-height  - find the height of the tree");
    println!("-count   - count the leaves of the tree.");
    println!("-length  - find the length of the tree");
    println!("-max     - find the maximum value in the tree");
    println!("-min     - find the minimum value in the tree");
    println!("-empty   - check if the tree is empty");
    println!("-contain");
    println!("    /");
    println!(" search  - check if the tree contains a certain value");
    println!("-print   - print tree in order\n");
    println!("-exit    - exit and erase current tree \n");
}

pub fn hello(){
    println!("----------------------------------------Trees CLI--------------------------------------------------");
    println!(":::: Please enter the name of a tree followed by the wanted action and value or 'exit' to leave :::");
    println!("---------------------------------------------------------------------------------------------------\n");
    println!("Available trees: \n---------------- \n- AVL tree (avl) \n- Red-Black Tree (rbt)\n- Binary Search Tree (bst)\n");
    println!("Availabe operations: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains/search \n- print\n");
    println!("How to use the CLI: ");
    println!("-------------------");
}
