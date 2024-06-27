mod lexer;
mod repl;
mod token;

fn main() {
    println!("Welcome to monkey programming language!");

    repl::start();
}
