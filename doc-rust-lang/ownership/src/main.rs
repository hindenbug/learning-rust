// Ownership Rules:
// Rust ensures that there is exactly one binding to any given resource
// when ownership is transferred to another binding, you cannot use the original binding
// * At any given point for a non primitive (non-copyable) value, there is only one owner that can change it (example 2)

fn main() {

    //example 1

    // It does not follow ownership rules for primitive data types,
    // as primitive data types have a Copy trait or move semantics
    let a = true;
    // ownership transfer
    let _y = copy_semantics(a);

    // This works and returns 'true'
    println!("{}", a);


    // example 2

    // non - copyable types would follow ownership rules or have move semantics
    let v = vec![1, 2, 3, 4];
    // ownership transfer
    move_semantics(v);
    // Won't compile as it throws error "value moved here"
    // println!("{}", v[0]);

}

fn copy_semantics(x: bool) -> bool {
    !x
}

fn move_semantics(v: Vec<i32>) -> Vec<i32>{
    v
}

// If every function hands back ownership which can be tideous, this where borrowing comes in
