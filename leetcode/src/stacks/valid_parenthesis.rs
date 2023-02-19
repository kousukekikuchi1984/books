use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue: Vec<char> = vec![];
        let map = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        for char in s.chars() {
            if char == '(' || char == '{' || char == '[' {
                queue.push(char);
            } else {
                let prev = queue.pop();
                if prev == None {
                    return false;
                } else {
                    if char != map[&prev.unwrap()] {
                        return false;
                    }
                }
            }
        }
        queue.len() == 0
    }
}
