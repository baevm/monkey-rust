mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Welcome to monkey programming language!");

    repl::start();
}
