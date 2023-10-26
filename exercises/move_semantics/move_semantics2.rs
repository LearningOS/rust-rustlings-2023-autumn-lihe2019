// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


pub fn main() {
    let mut vec0 = Vec::new(); // As per hint #3

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
