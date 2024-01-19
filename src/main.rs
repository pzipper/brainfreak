use opt::Optimizer;
use parser::Parser;

pub mod backend;
pub mod inst;
pub mod opt;
pub mod parser;

fn main() {
    let data = std::fs::read_to_string("test.bf").unwrap();

    let mut parser = Parser::new(&data);
    let tokens = parser.parse().unwrap();

    let mut optimizer = Optimizer::new();
    optimizer.write_tokens(&tokens);

    std::fs::write("test.o", backend::object::compile(&optimizer.finish())).unwrap();
}
