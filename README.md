# Brainfreak
A simple Brainf-ck compiler written in Rust with Cranelift.

## Use Brainfreak

### Prerequisites
- The Cargo package manager
- A linker of your choice

```
cargo install brainfreak
brainfreak build-obj brainfreak-program.bf -o program.o
gcc program.o -o program
./program
```