mod storage;
mod parser;
mod executor;
mod types;

fn main() {
    storage::read_int_from_file();
    storage::write_int_to_file();
}