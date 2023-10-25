// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// learn: 传递的时候必须声明是不是引用类型
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // learn: 引用被克隆的时候不再是引用类型
    let mut vec = vec.clone();

    vec.push(88);

    vec
}
