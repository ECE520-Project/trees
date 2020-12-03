
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
                println!("avl branch");//for testing
                let mut avl = AVLTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("op > ");
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
    
                        "contains" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            avl.contains(val);
                        },
                        "height" => println!("{:?}",avl.height()),
                        "count" => println!("{:?}",avl.count_leaves()),
                        "length" => println!("{:?}",avl.len()),
                        "min" => println!("{:?}",avl.min()),
                        "max" => println!("{:?}",avl.max()),
                        "empty" => println!("{:?}",avl.is_empty()),
                        "print" => {print!("your tree: ");avl.print_inorder();},
                        "help" => println!("Availabe operations: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                        "exit" => return,
                        _ => println!("Command not recognized."),
                    }
                    let mut stay = String::new();
                    print!("keep testing AVL: (y) or exit to test other trees: (n) ? > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        println!("Invalid answer, exiting AVL tree");
                        break;
                    }
                }
            },
            "rbt" => {
                println!("rbt branch");//for testing
                let mut rbt = RedBlackTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("op > ");
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
    
                        "contains" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            rbt.contains(val);
                        },
                        "height" => println!("{:?}",rbt.height()),
                        "count" => println!("{:?}",rbt.count_leaves()),
                        "length" => println!("{:?}",rbt.len()),
                        "min" => println!("{:?}",rbt.min()),
                        "max" => println!("{:?}",rbt.max()),
                        "empty" => println!("{:?}",rbt.is_empty()),
                        "print" => rbt.print_inorder(),
                        "help" => println!("Availabe operations: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                        "exit" => return,
                        _ => println!("Command not recognized."),
                    }
                    let mut stay = String::new();
                    print!("keep testing RBT: (y) or exit to test other trees: (n) ? > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        println!("Invalid answer, exiting RBT tree");
                        break;
                    }
                }
            },
            "bst" => {
                println!("bst branch");//for testing
                
                let mut bst = BinarySearchTree::<i32>::new();
                let mut stay_in_tree = true;

                while stay_in_tree{
                    let mut operation = String::new();
                    print!("op > ");
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
    
                        "contains" => {
                            let mut value = String::new();
                            print!("value to search > ");
                            let val = get_val("searching for",&mut value);
                            bst.contains(val);
                        },
                        "height" => println!("{:?}",bst.height()),
                        "count" => println!("{:?}",bst.count_leaves()),
                        "length" => println!("{:?}",bst.len()),
                        "min" => println!("{:?}",bst.min()),
                        "max" => println!("{:?}",bst.max()),
                        "empty" => println!("{:?}",bst.is_empty()),
                        "print" => bst.print_inorder(),
                        "help" => println!("Availabe operations: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                        "exit" => return,
                        _ => println!("Command not recognized."),
                    }
                    let mut stay = String::new();
                    print!("keep testing BST: (y) or exit to test other trees: (n) ? > ");
                    get_user_input(&mut stay);

                    if stay.to_lowercase().contains("n"){
                        stay_in_tree = false;
                    }
                    else if stay.to_lowercase().contains("y"){
                        continue;
                    }
                    else{
                        println!("Invalid answer, exiting BST tree");
                        break;
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
        Err(..) => println!("this was not an integer number"),
    };
    let val: i32 = FromStr::from_str(trimmed_val).unwrap();
    return val
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
