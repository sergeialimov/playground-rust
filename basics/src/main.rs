mod control_flow;
mod data_structures;
mod strings;
mod borrow_checker;
mod dangle;
mod slice;
mod iterators;

fn main() {
    println!("Hey, rust!");
    // control_flow::loops::another_loop::nested_loop();
    borrow_checker::reverse::main();
}
