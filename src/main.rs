fn shunting_yard(input: &str) -> Vec<char> {
    let mut operator_stack: Vec<char> = Vec::new();
    let mut rpn_stack: Vec<char> = Vec::new();
    for ch in input.chars() {
        match ch {
            '+' => operator_stack.push(ch),
            '1'..='9' => rpn_stack.push(ch),
            _ => panic!("unexpected character"),
        }
    }
    while let Some(ch) = operator_stack.pop() {
        rpn_stack.push(ch);
    }
    rpn_stack
}

fn main() {
    let input = "1+2";
    let output = shunting_yard(input);
    println!("input: {:?}", input);
    println!("output: {:?}", output);
}

#[cfg(test)]
mod tests {
    use crate::shunting_yard;
    #[test]
    fn one_char_case() {
        let input = "1+2";
        assert_eq!(shunting_yard(input), vec!['1', '2', '+']);
    }

    #[test]
    fn one_char_case2() {
        let input = "1+2+3";
        assert_eq!(shunting_yard(input), vec!['1', '2', '3', '+', '+']);
    }

    #[test]
    #[should_panic]
    fn unexpected_token() {
        let input = "1~2";
        shunting_yard(input);
    }
}
