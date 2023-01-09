extern crate libc;

extern "C" {
    fn print_array(array: *const i32, length: usize);
}
fn main() {
    let array = [1, 2, 3, 4, 5];
    unsafe{
    print_array(array.as_ptr(), array.len());
    }
}