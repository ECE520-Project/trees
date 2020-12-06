#[path = "../examples/avl_tree.rs"]
mod avl_tree;
#[path = "../examples/binary_search_tree.rs"]
mod binary_search_tree;
#[path = "../examples/red_black_tree.rs"]
mod red_black_tree;
mod cli;
use std::env;


pub fn main(){

    let args: Vec<String> = env::args().collect();

    match args.len(){
        1 => {
            //no arguments passed: cargo run
            println!("Welcome!");
            println!("View examples of the program or use the CLI ?");
            println!("Enter 'Yes' to view examples or 'No' to use the CLI");

            print!("Your choice > ");
            let answer = cli::get_user_input();
        
            if answer.to_lowercase().contains("n") {
                cli::hello();
                cli::run_cli();
            }
            else if answer.as_str().to_lowercase().contains("y") {
                avl_tree::main();
                red_black_tree::main();
                binary_search_tree::main();
            }
            else{eprint!("Invalid choice , restart");}

        },    
        _ => eprint!("Invalid input , restart"),
    } 
}
