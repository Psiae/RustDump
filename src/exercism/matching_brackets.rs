pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() { return true }
    if string.len() == 1 { return false }
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        if c == '{' || c == '[' || c == '('  { stack.push(c) ; continue }
        match c {
            '}' => if stack.pop() != Some('{') { return false },
            ']' => if stack.pop() != Some('[') { return false },
            ')' => if stack.pop() != Some('(') { return false }
            _ => {}
        }
    };

    return stack.is_empty()
}
