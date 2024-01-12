// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // option 1: create a copy and pass it instead
    let vec0_copy = vec0.clone();
    let vec1 = fill_vec(vec0_copy);

    // option 2: make argument mutable and clone in function
    let vec2 = fill_vec_borrow(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
    assert_eq!(vec2, vec![22, 44, 66, 110]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(110);

    vec
}
