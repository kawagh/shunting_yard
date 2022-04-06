use std::collections::HashMap;
fn shunting_yard(input: &str) -> Vec<char> {
    let operators = vec!['+', '-', '*', '/', '(', ')'];
    let precedences = vec![0, 0, 1, 1, -1, -1];
    let precedences_map: HashMap<_, _> = operators.iter().zip(precedences.iter()).collect();
    let mut operator_stack: Vec<char> = Vec::new();
    let mut rpn_tokens: Vec<char> = Vec::new();
    for ch in input.chars() {
        match ch {
            '+' | '-' | '*' | '/' => {
                while let Some(stack_top) = operator_stack.last() {
                    if precedences_map[stack_top] >= precedences_map[&ch] {
                        let stack_top = operator_stack.pop().unwrap();
                        rpn_tokens.push(stack_top);
                    } else {
                        break;
                    }
                }
                operator_stack.push(ch);
            }
            '(' => operator_stack.push(ch),
            ')' => {
                while let Some(stack_top) = operator_stack.last() {
                    if stack_top == &'(' {
                        operator_stack.pop();
                        break;
                    }
                    rpn_tokens.push(operator_stack.pop().unwrap());
                }
            }
            '1'..='9' => rpn_tokens.push(ch),
            _ => panic!("unexpected character"),
        }
    }
    while let Some(ch) = operator_stack.pop() {
        rpn_tokens.push(ch);
    }
    rpn_tokens
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
        let expected = vec!['1', '2', '+'];
        assert_eq!(expected, shunting_yard(input));
    }

    #[test]
    fn one_char_case2() {
        let input = "1+2+3";
        let expected = vec!['1', '2', '+', '3', '+'];
        assert_eq!(expected, shunting_yard(input));
    }

    #[test]
    #[should_panic]
    fn unexpected_token() {
        let input = "1~2";
        shunting_yard(input);
    }

    #[test]
    fn accept_operators() {
        let input = "1*2";
        let expected = vec!['1', '2', '*'];
        assert_eq!(expected, shunting_yard(input));
    }

    #[test]
    fn one_char_with_precedence() {
        let input = "1*2+3";
        let expected = vec!['1', '2', '*', '3', '+'];
        assert_eq!(expected, shunting_yard(input));
    }

    #[test]
    fn one_char_with_precedence2() {
        let input = "1+2*3";
        let expected = vec!['1', '2', '3', '*', '+'];
        assert_eq!(expected, shunting_yard(input));
    }

    #[test]
    fn with_parentheses() {
        let input = "(1+2)*3";
        let expected = vec!['1', '2', '+', '3', '*'];
        assert_eq!(expected, shunting_yard(input));
    }
}
