
use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use trees::base::QueryableTree;

use std::io::{stdin, stdout, Write};
use std::process::exit;
//use std::env;

extern crate structopt;
use structopt::StructOpt;

// cargo run -- --help 

#[derive(Debug, StructOpt)]
/// About : CLI for Trees
/// ```
/// Authors: Amina A. Elathir, Hao Li , Ke Fang
pub struct Cli{
    /// Tree name ex: AVL, RBT
    tree: String,
    /// Operation on tree ex: insert, delete, contains
    op: Option<String>,
    /// value to insert/ delete/search  
    val: Option<i32>,
}

pub fn run_cli(){
    loop{
        let args = Cli::from_args();
        let t = args.tree;
        let operation = args.op;
        let value = args.val.unwrap();
        // let args: Vec<String> = env::args().collect();
        // let t = &args[1];
        // let operation = &args[2];
        // let value = &args[3];

        if t.to_lowercase().contains("avl"){

            println!("testing if....you're in avl branch;");

            let mut avl = AVLTree::new();

            match operation.unwrap().to_lowercase().as_str() {
                "insert"  => avl.insert(value),
                "delete" => avl.delete(value),
                "height" => println!("{:?}",avl.height()),
                "count" => println!("{:?}",avl.count_leaves()),
                "length" => println!("{:?}",avl.len()),
                "contains" => println!("{:?}",avl.contains(value)),
                "min" => println!("{:?}",avl.min()),
                "max" => println!("{:?}",avl.max()),
                "empty" => println!("{:?}",avl.is_empty()),
                "print" => avl.print_inorder(),

                "help" => println!("Availabe commands: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                
                "exit" => return,

                _ => println!("Command not recognized. Type help for available commands."),
            }
            let mut choice = String::new();
            print!("Do you want to enter another command? (y/n) >");
            get_user_input(&mut choice);

            if choice.to_lowercase().contains("y") {
                continue; //fix looop back
            }
            else if choice.to_lowercase().contains("n") {
                break;
            }
        }
        else if t.to_lowercase().contains("rbt"){

            println!("testing else if: you're in rbt branch;");
            
            let mut rbt = RedBlackTree::new(0); //change to empty tree
            match operation.unwrap().to_lowercase().as_str() {
                "insert"  => rbt.insert(value),
                "delete" => rbt.delete(value),
                "height" => println!("{:?}",rbt.height()),
                "count" => println!("{:?}",rbt.count_leaves()),
                "length" => println!("{:?}",rbt.len()),
                "contains" => println!("{:?}",rbt.contains(value)),
                "min" => println!("{:?}",rbt.min()),
                "max" => println!("{:?}",rbt.max()),
                "empty" => println!("{:?}",rbt.is_empty()),
                "print" => rbt.print_inorder(),

                "help" => println!("Availabe commands: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                
                "exit" => return,

                _ => println!("Command not recognized. Type help for available commands."),
            }
        }
        else if t.to_lowercase().contains("bst"){

            println!("testing bst branch");

            let mut bst = BinarySearchTree::new();

            match operation.unwrap().to_lowercase().as_str() {
                "insert"  => bst.insert(value),
                "delete" => bst.delete(value),
                "height" => println!("{:?}",bst.height()),
                "count" => println!("{:?}",bst.count_leaves()),
                "length" => println!("{:?}",bst.len()),
                "contains" => println!("{:?}",bst.contains(value)),
                "min" => println!("{:?}",bst.min()),
                "max" => println!("{:?}",bst.max()),
                "empty" => println!("{:?}",bst.is_empty()),
                "print" => bst.print_inorder(),

                "help" => println!("Availabe commands: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
                
                "exit" => return,

                _ => println!("Command not recognized. Type help for available commands."),
            }


        }
        else if t.to_lowercase().contains("exit"){
            exit(1);
        }
        else if t.to_lowercase().contains("help"){
            println!("use: cargo run [tree_name] [operation] [optional value] \nExample1: cargo run avl insert 5 \nExample2: cargo run avl print ")
        }
        else{
            eprint!("There is no tree with that name");
        }    
    }  
}    

pub fn get_user_input(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read from stdin");
}

// pub fn chosen_tree<T>(operation: Option<String>, tree:T, value:T)
//     where
//     T: QueryableTree<T: Ord + Copy + fmt::Debug>
// {
//     match operation.unwrap().to_lowercase().as_str() {
//         "insert"  => tree.insert(value),
//         "delete" => tree.delete(value),
//         "height" => println!("{:?}",tree.height()),
//         "count" => println!("{:?}",tree.count_leaves()),
//         "length" => println!("{:?}",tree.len()),
//         "contains" => println!("{:?}",tree.contains(value)),
//         "min" => println!("{:?}",tree.min()),
//         "max" => println!("{:?}",tree.max()),
//         "empty" => println!("{:?}",tree.is_empty()),
//         "print" => tree.print_inorder(),
//         "help" => println!("Availabe commands: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
//         "exit" => return,
//         _ => println!("Command not recognized. Type help for available commands."),
//     }
// }
pub fn hello(){
    println!("----------------------------------------Trees CLI--------------------------------------------------");
    println!(":::: Please enter the name of a tree followed by the wanted action and value or 'exit' to leave :::");
    println!("---------------------------------------------------------------------------------------------------\n");
    println!("Available trees: \n---------------- \n- AVL tree (avl) \n- Red-Black Tree (rbt)\n");
    println!("Availabe commands: \n------------------ \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print\n");
    println!("How to use the CLI: ");
    println!("-------------------");
    println!("use: cargo run [tree_name] [operation] [optional value] \nExample1: cargo run avl insert 5 \nExample2: cargo run avl print ")
}
