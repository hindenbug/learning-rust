pub fn reply(stmt: &str) -> &str {

   if stmt.is_empty() {
        "Fine. Be that way!"
   } else if all_upppercase(stmt) {
       "Whoa, chill out!"
   } else if is_question(stmt){
       "Sure."
   } else {
       "Whatever."
   }
}

fn all_upppercase(stmt: &str) -> bool {
    stmt.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
}

fn is_question(stmt: &str) -> bool {
    stmt.ends_with("?")
}
