use trees::bstree::BinarySearchTree;
use trees::rbtree::RedBlackTree;
use trees::avltree::AVLTree;
use crate::base::{QueryableTreeNode, QueryableTree};

use structopt::StructOpt;

##[derive(StructOpt)]
struct Cli{
    ///Authors
    #[structopt(default_value = "Created by: Amina & Hao Li & Ke Fang")]
    ///Need Help ?
    tree: String,
    command: String,
    val: i32
    choice: String
}
impl Cli{

    fn tree(tree_name:String){

        if tree_name.to_lowercase().contains("avl"){
            tree_name = AVLTree
        }
        else if tree_name.to_lowercase().contains("rbt"){
            tree_name = RedBlackTree
        }
        let mut t = tree_name::new();
        match Cli.command.to_lowercase(){
            "insert" => t.insert(args.val),

        }


    }
    fn rbt(){

    } 
    fn avl(){

        let args = Cli::from_args();
        let mut avl = AVLTree::new();

        match args.command.to_lowercase(){
            "insert" => avl.insert(args.val),
            "delete" => avl.delete(args.val),
            "height" => avl.height(),
            "count" => avl.count_leaves(),
            "length" => avl.len(),
            "contains" => avl.contains(args.val),
            "min" => avl.min(),
            "max" => avl.max(),
            "empty" => avl.is_empty(),
            "print" => avl.print_inorder(),
            "help" => println!("Availabe commands: - insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print"),
            "exit" => return,

            _ => println!("Command {:?} not recognized. Type help for available commands.", args.command),
        }
    } 
}

fn main(){

    println!("----------------------CLI-------------------------");
    println!("Please enter the name of a tree followed by the wanted action and value or 'exit' to leave:");
    println!("Available trees: \n- AVL tree (avl) \n- Red-Black Tree (rbt)");
    println!("Availabe commands: \n- insert \n- delete \n- height \n- count \n- length \n- min \n- max \n- empty \n- contains \n- print");

    let args = Cli::from_args();

    let t = args.tree;
    let ch = args.choice;

    println!("Chosen Tree: {}",t);

    loop{
        println!("Do you want to enter another command?");

        if t.to_lowercase().contains("rbt"){
            Cli::rbt();
        }
        else if t.to_lowercase().contains("avl"){
            Cli::avl();
        }
        else if t.to_lowercase().contains("exit"){
            exit(1);
        }
        else{
            eprint!("There is no tree with that name");
        }

    }

    




    
    // println!("----------------------CLI-------------------------");
    // println!("Hello, Please enter a number of your chosen tree: ");
    // println!("1- Binary Search Tree\n2- AVL tree\n3- Red-Black Tree");

    // let mut input_text = String::new();
    // io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    // let trimmed_input = input_text.trim();
    // match trimmed_input.parse::<i8>() {
    //     Ok(t) => println!("Your Chosen Tree: {}", t),
    //     Err(..) => println!("Invalid !, please enter an integer: {}", trimmed_input),
    // };
    // let choice = i8 = FromStr::from_str(trimmed_input).unwrap();

    // if choice == 1 {

    // }
    // else if choice == 2 {

    // }
    // else if choice == 3 {

    // }
    // else{
    //     panic!("Invalid Input");
    // }

}
// fn get_command(cmd:i32){

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("failed to read from stdin");

// }