// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 不能将字符串的一部分所有权转移出去
    let nice_slice = &a[1..4];
    // if not  borrowing here 
    // ^^^^^^^^^^ doesn't have a size known at compile-time


    assert_eq!([2, 3, 4], nice_slice)
}
