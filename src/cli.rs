
use std::str::FromStr;
use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use trees::base::QueryableTree;

use std::io::{stdin, stdout, Write};


pub fn run_cli(){

    let mut first_time = true;
    loop {
        let mut selected_tree = String::new();
        while first_time{
            println!("you can select a tree to start or print 'exit' to leave");
            println!("Select a tree!.\n-AVL \n-BST \n-RBT or type 'help' to learn about the commands");
            first_time = false;
        }
        print!("input > ");
        get_user_input(&mut selected_tree);

        match selected_tree.to_lowercase().trim().split_whitespace().next().unwrap(){
            "avl" => {
                println!("\n::...AVL Tree branch...::\n");//for testing
                let mut avl = AVLTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("operation > ");
                    get_user_input(&mut operation);

                    match operation.to_lowercase().trim().split_whitespace().next().unwrap() {
                        "insert"  => {
                            let mut value = String::new();
                            print!("inserted value > ");
                            let val = get_val("inserting",&mut value);
                            avl.insert(val);
                        },
                        "delete" => {
                            let mut value = String::new();
                            print!("deleted value > ");
                            let val = get_val("deleting",&mut value);
                            avl.delete(val);  
                        },
    
                        "contain" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            println!("\nvalues found ?: {:?}",avl.contains(val));
                        },
                        "height" => println!("\nHeight of tree: {:?}",avl.height()),
                        "count" => println!("\nNumber of leaves: {:?}",avl.count_leaves()),
                        "length" => println!("\nLenght: {:?}",avl.len()),
                        "min" => println!("\nMinimum Value: {:?}",avl.min().unwrap()),
                        "max" => println!("\nMaximum Value: {:?}",avl.max().unwrap()),
                        "empty" => println!("\nIs the tree empty?: {:?}",avl.is_empty()),
                        "print" => {print!("\nYour tree: ");avl.print_inorder();},
                        "help" => list_of_operations(),
                        "exit" => return,
                        _ => println!("\nCommand not recognized. Try 'help' for valid operations"),
                    }
                    let mut stay = String::new();
                    println!("\nkeep testing AVL: (y) or exit to test other trees: (n) ?\n");
                    print!("**WARNING** chosing (n) will erase your AVL tree \n > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        eprintln!("Invalid answer, try another operation");
                    }
                }
            },
            "rbt" => {
                println!("\n::...Red-Black Tree branch...::\n");//for testing
                let mut rbt = RedBlackTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("operation > ");
                    get_user_input(&mut operation);

                    match operation.to_lowercase().trim().split_whitespace().next().unwrap() {
                        "insert"  => {
                            let mut value = String::new();
                            print!("inserted value > ");
                            let val = get_val("inserting",&mut value);
                            rbt.insert(val);
                        },
                        "delete" => {
                            let mut value = String::new();
                            print!("deleted value > ");
                            let val = get_val("deleting",&mut value);
                            rbt.delete(val);  
                        },
    
                        "contain" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            println!("\nvalues found ?: {:?}",rbt.contains(val));
                        },
                        "height" => println!("Height: {:?}",rbt.height()),
                        "count" => println!("\nNumber of leaves: {:?}",rbt.count_leaves()),
                        "length" => println!("\nLength: {:?}",rbt.len()),
                        "min" => println!("\nMinimum Value: {:?}",rbt.min().unwrap()),
                        "max" => println!("\nMaximum Value: {:?}",rbt.max().unwrap()),
                        "empty" => println!("\nIs the tree empty?: {:?}",rbt.is_empty()),
                        "print" => {print!("\nYour tree: ");rbt.print_inorder();},
                        "help" => list_of_operations(),
                        "exit" => return,
                        _ => println!("\nCommand not recognized. Try 'help' for valid operations"),
                    }
                    let mut stay = String::new();
                    println!("\nkeep testing RBT: (y) or exit to test other trees: (n) ?\n");
                    print!("**WARNING** chosing (n) will erase your RBT tree \n > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        eprintln!("Invalid answer, try another operation");
                    }
                }
            },
            "bst" => {
                println!("\n::...Binary-Search Tree branch...::\n");//for testing
                
                let mut bst = BinarySearchTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("operation > ");
                    get_user_input(&mut operation);

                    match operation.to_lowercase().trim().split_whitespace().next().unwrap() {
                        "insert"  => {
                            let mut value = String::new();
                            print!("inserted value > ");
                            let val = get_val("inserting",&mut value);
                            bst.insert(val);
                        },
                        "delete" => {
                            let mut value = String::new();
                            print!("deleted value > ");
                            let val = get_val("deleting",&mut value);
                            bst.delete(val);  
                        },
    
                        "contain" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            println!("values found ?: {:?}",bst.contains(val));
                        },
                        "height" => println!("\nHeight: {:?}",bst.height()),
                        "count" => println!("\nNumber of leaves: {:?}",bst.count_leaves()),
                        "length" => println!("\nLenght: {:?}",bst.len()),
                        "min" => println!("\nMinimum Value: {:?}",bst.min().unwrap()),
                        "max" => println!("\nMaximum Value: {:?}",bst.max().unwrap()),
                        "empty" => println!("\nIs the tree empty?: {:?}",bst.is_empty()),
                        "print" => {print!("\nYour tree: ");bst.print_inorder();},
                        "help" => list_of_operations(),
                        "exit" => return,
                        _ => println!("\nCommand not recognized. Try 'help' for valid operations"),
                    }
                    let mut stay = String::new();
                    println!("\nkeep testing BST: (y) or exit to test other trees: (n) ?\n");
                    print!("**WARNING** chosing (n) will erase your BST tree \n > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        eprintln!("Invalid answer, try another operation");
                    }
                }
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
                eprint!("{:?} is not a valid command or tree\n", selected_tree.trim().split_whitespace().next().unwrap());
            }
        }
    }
}


pub fn get_user_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read from stdin");
}
pub fn get_val(op: &str, mut value: &mut String)-> i32 {        
    get_user_input(&mut value);
    let trimmed_val = value.trim();
    match trimmed_val.parse::<i32>(){
        Ok(val) => println!("{} value: '{}' in tree",op, val),
        Err(..) => panic!("this was not an integer number"),
    };
    let val: i32 = FromStr::from_str(trimmed_val).unwrap();
    return val
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
        println!("-contain - check if the tree contains a certain value");
        println!("-print   - print tree in order\n");  
}

pub fn hello(){
    println!("----------------------------------------Trees CLI--------------------------------------------------");
    println!(":::: Please enter the name of a tree followed by the wanted action and value or 'exit' to leave :::");
    println!("---------------------------------------------------------------------------------------------------\n");
    println!("Available trees: \n---------------- \n- AVL tree (avl) \n- Red-Black Tree (rbt)\n- Binary Search Tree (bst)\n");
    println!("Availabe operations: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print\n");
    println!("How to use the CLI: ");
    println!("-------------------");
    println!("use: [tree_name] [operation] [optional value] \nExample1: avl insert 5 \nExample2: avl print ")
}
