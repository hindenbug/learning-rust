pub fn hello(name: Option<&str>) -> String {
    if let Some(name) = name {
        format!("Hello, {}!", name)
    } else {
        format!("Hello, World!")
    }
}
