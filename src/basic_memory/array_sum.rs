//given a pointer to the start of a byte array and
//its length, return the sum of its elements.
#[wasm_bindgen]
pub unsafe fn array_sum(ptr: *mut u8, len: usize) -> u8 {
    //create a Vec<u8> from the pointer to the
    //linear memory and the length
    let data = Vec::from_raw_parts(ptr, len, len);
    //compute the sum
    data.iter().sum();
}