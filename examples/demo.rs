fn main() {
    let message = Box::new("if you can read this, the custom global_allocator was not used");
    println!("{}", message);
}
