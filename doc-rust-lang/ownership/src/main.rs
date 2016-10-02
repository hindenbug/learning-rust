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
    // println!("{}", a);


    // example 2

    // non - copyable types would follow ownership rules or have move semantics
    let v = vec![1, 2, 3, 4];
    // ownership transfer
    move_semantics(v);
    // Won't compile as it throws error "value moved here"
    // println!("{}", v[0]);

    // IntroRustExample
    // ownership transfered to adjective and name
    let(adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);

}

fn copy_semantics(x: bool) -> bool {
    !x
}

fn move_semantics(v: Vec<i32>) -> Vec<i32>{
    v
}

// If every function hands back ownership which can be tideous, this where borrowing comes in


// Excellent Example on Ownership from: http://intorust.com/tutorial/ownership/

fn two_words() -> (String, String) {
    let string1 = format!("Fellow");
    let string2 = format!("Rustaceans");

    // transfer ownership
    (string1, string2)
}

fn remove_vowels(name: String) -> (String, String) {
    // Goal #1: What is needed here to make this compile?
    // Sol : make the var mutable
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }

    // transfer ownership
    (output, name)
}

fn print_out(name: String) {
    let (devowelized_name, name1) =  remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2
    println!("Removing vowels from {:?} yields {:?}", name1, devowelized_name);
}
