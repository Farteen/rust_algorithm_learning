#[test]
fn test_access_modify_global_variable() {
    println!("{}", HELLO_WORLD);
}

static HELLO_WORLD: &str = "Hello world";

#[derive(Debug,Default)]
struct Circle<T> {
    data: T
}