//pub struct Brackets(Vec<char>);

pub struct Brackets {
    input: &'static str,
}

impl From<&'static str> for Brackets {
    fn from(s: &'static str) -> Self {
        Brackets {input: s}
    }
}

impl Brackets {
    pub fn are_balanced(self) -> bool {
        let mut unbalanced: Vec<char> = Vec::new();

        for bracket in self.input.chars() {
            if !Brackets::is_bracket(bracket) { continue }

            if let Some(last_open_bracket) = unbalanced.pop() {
                let matched = match (last_open_bracket, bracket) {
                    ('(', ')') => true,
                    ('{', '}') => true,
                    ('[', ']') => true,
                    _ => false
                };

                if !matched {
                    unbalanced.push(last_open_bracket);
                    unbalanced.push(bracket);
                }
            } else {
                unbalanced.push(bracket);
            }
        }
        unbalanced.is_empty()
    }

    pub fn is_bracket(c: char) -> bool {
        c == '[' || c == ']' || c == '{' || c == '}' || c == '(' || c == ')'
    }

}
