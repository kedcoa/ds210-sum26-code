use malloc::MALLOC;

#[test]
fn test_malloc() {
    let ptr = unsafe { MALLOC.malloc(4) };
    assert!(!ptr.is_null());

    let actual_ptr = ptr as *mut u32;
    unsafe {*actual_ptr = u32::MAX - 10;};
    assert_eq!(unsafe { *actual_ptr }, u32::MAX - 10);

    unsafe { MALLOC.free(ptr); }
}

#[test]
fn test_malloc2() {
    let ptr = unsafe { MALLOC.malloc(0) };
    assert!(!ptr.is_null());
    unsafe { MALLOC.free(ptr); }
}