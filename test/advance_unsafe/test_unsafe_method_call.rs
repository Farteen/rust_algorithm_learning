#[test]
fn test_unsafe_method_call() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}