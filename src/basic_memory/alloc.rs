//allocate memory into the module's linear memory
//and return the offset to the start of the block
#[wasm_bindgen]
pub fn alloc(len: usize) -> *mut u8 {
    //create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    //take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();

    //take ownership of the memory block
    //ensure its destructor is not called
    //when the obj goes out of scope
    //at the end of the function
    std::mem::forget(buf);
    //return the pointer so the runtime
    //can write data at this offset
    return ptr;
}

