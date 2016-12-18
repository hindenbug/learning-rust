pub fn raindrops(n: i32) -> String {
    let mut sound = String::new();

    // must be a better way to do this

    if is_factor(n, 3) {
        sound.push_str("Pling")
    }

    if is_factor(n, 5){
        sound.push_str("Plang")
    }

    if is_factor(n, 7){
        sound.push_str("Plong")
    }

    if sound.is_empty(){
        let s = format!("{}", n);
        sound.push_str(&s)
    }

    sound
}

fn is_factor(n: i32, f: i32) -> bool {
    n % f == 0
}
