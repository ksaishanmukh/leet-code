#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stk: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stk.push(c),
            ')' if stk.last().eq(&Some(&'(')) => {
                stk.pop();
            }
            '}' if stk.last().eq(&Some(&'{')) => {
                stk.pop();
            }
            ']' if stk.last().eq(&Some(&'[')) => {
                stk.pop();
            }
            _ => return false,
        };
    }

    stk.is_empty()
}
