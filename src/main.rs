use args::Command;
use clap::Parser as ClapParser;
use opt::Optimizer;
use parser::Parser;

pub mod args;
pub mod backend;
pub mod inst;
pub mod opt;
pub mod parser;

fn main() {
    let command_kind = Command::parse();

    match command_kind {
        Command::BuildObj(command) => {
            let data: String = match std::fs::read_to_string(&command.input_path) {
                Ok(data) => data,
                Err(_) => {
                    println!("error: couldn't read input file");
                    return;
                }
            };

            let mut parser = Parser::new(&data);
            let tokens = parser.parse().unwrap();

            let mut optimizer = Optimizer::new();
            optimizer.write_tokens(&tokens);

            match std::fs::write(
                &command.output_path,
                backend::object::compile(&optimizer.finish()),
            ) {
                Err(_) => {
                    println!("error: couldn't write output file");
                    return;
                }
                _ => {}
            };
        }
    }
}
