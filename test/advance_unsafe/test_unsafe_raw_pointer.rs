#[test]
fn test_unsafe_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

#[test]
fn test_arbitary_memory_address() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

#[test]
fn test_access_raw_pointer_in_unsafe() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}