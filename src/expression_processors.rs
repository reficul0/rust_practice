use std::{collections::{HashMap}};
use once_cell::sync::Lazy;

pub trait ExpressionProcessor {
    fn process(&self, expression: &String) -> String;
}

pub struct PostfixReversePolishNotation {
}

impl PostfixReversePolishNotation {
    pub fn new() -> Self {
        PostfixReversePolishNotation {}
    }

    fn append_operator_postfix(&self, op: char, postfix_expr: &mut String) {
        postfix_expr.push(' ');
        postfix_expr.push(op);
    }

    fn unwind<T>(&self, operators_stack: &mut Vec<char>, mut process_next_operator: T)
    where T: FnMut(char) -> Unwinding {
        while let Some(operator_from_stack) = operators_stack.last() {
            if let Unwinding::Stop(flags) = process_next_operator(*operator_from_stack) {
                match flags {
                    StoppingFlags::PopCurrent => { operators_stack.pop(); },
                    _ => {}
                }
                break;
            }
            operators_stack.pop();
        }
    }

    fn to_postfix_expression(&self, origin_expression: &String) -> String {
        let mut postfix_expr = String::new();
        let mut operators_stack: Vec<char> = Vec::new();

        static OPERATORS_PRIORITIES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
            HashMap::from([
                ('(', 0), // opening bracket could be in operations stack
                ('+', 1),
                ('-', 1),
                ('*', 2),
                ('/', 2),
                ('^', 3),
                //('~', 4)	//	Internal OP - unary minus
            ])
        });

        for (current_character_id, current_character) in origin_expression.chars().enumerate() {
            if current_character.is_digit(10) || current_character.is_alphabetic() {
                postfix_expr.push(current_character);
                continue;
            }

            // is it operator and not bracket
            let mut is_known_operator = false;
            match current_character {
                '(' => operators_stack.push(current_character),
                ')' => {
                    let mut is_closing_bracket_found = false;

                    self.unwind(&mut operators_stack, |operator_from_stack| {
                        return match operator_from_stack {
                            '(' => {
                                is_closing_bracket_found = true;
                                Unwinding::Stop(StoppingFlags::PopCurrent)
                            },
                            _ => {
                                self.append_operator_postfix(operator_from_stack, &mut postfix_expr);
                                Unwinding::Continue
                            }
                        }
                    });

                    if !is_closing_bracket_found {
                        panic!("Can't find open bracket '(' for closing bracket ')' at id [{current_character_id}]");
                    }
                },
                op => if let Some(priority) = OPERATORS_PRIORITIES.get(&op) {
                    is_known_operator = true;

                    self.unwind(&mut operators_stack, |operator_from_stack| {
                        let stack_op_priority = OPERATORS_PRIORITIES.get(&operator_from_stack).unwrap();
                        if stack_op_priority < priority {
                            Unwinding::Stop(StoppingFlags::None)
                        } else {
                            self.append_operator_postfix(operator_from_stack, &mut postfix_expr);
                            Unwinding::Continue
                        }
                    });
                    operators_stack.push(op);
                } else {
                    panic!("unsupported operator [{current_character}] at id [{current_character_id}]");
                }
            }

            if is_known_operator {
                postfix_expr.push(' ');
            }
        }

        for op in operators_stack.iter().rev() {
            assert!(*op != '(' && *op != ')', "Wrong braces order");
            self.append_operator_postfix(*op, &mut postfix_expr);
        }

        postfix_expr
    }
}

impl ExpressionProcessor for PostfixReversePolishNotation {
    fn process(&self, expression: &String) -> String {
        self.to_postfix_expression(expression)
    }
}

#[derive(PartialEq)]
enum StoppingFlags {
    PopCurrent,
    None
}

#[derive(PartialEq)]
enum Unwinding {
    Stop(StoppingFlags),
    Continue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;

        assert_eq!(processor.process(&String::from("1+2+3")), String::from("1 2 + 3 +"));
        assert_eq!(processor.process(&String::from("1-2-3")), String::from("1 2 - 3 -"));
        assert_eq!(processor.process(&String::from("1*2*3")), String::from("1 2 * 3 *"));
        assert_eq!(processor.process(&String::from("1/2/3")), String::from("1 2 / 3 /"));
        assert_eq!(processor.process(&String::from("(1/2/3)")), String::from("1 2 / 3 /"));

        assert_eq!(processor.process(&String::from("1+2*3")), String::from("1 2 3 * +"));
        assert_eq!(processor.process(&String::from("1*2+3")), String::from("1 2 * 3 +"));

        assert_eq!(processor.process(&String::from("1/(2-3)")), String::from("1 2 3 - /"));
        assert_eq!(processor.process(&String::from("(2-3)/1")), String::from("2 3 - 1 /"));

        assert_eq!(processor.process(&String::from("1*(2-3)-4/(5+6)")), String::from("1 2 3 - * 4 5 6 + / -"));

        assert_eq!(processor.process(&String::from("12345+6789-1")), String::from("12345 6789 + 1 -"));
    }

    #[test]
    fn test_happy_path_fictive_braces() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;

        assert_eq!(processor.process(&String::from("((1+2+3))")), String::from("1 2 + 3 +"));
        // todo: bug or feature?
        assert_eq!(processor.process(&String::from("(())((1+2+3))(())")), String::from("1 2 + 3 +"));
        assert_eq!(processor.process(&String::from("(())")), String::from(""));
        assert_eq!(processor.process(&String::from("()(())")), String::from(""));
    }

    #[test]
    fn test_happy_path_braces_with_operators_between() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;
        assert_eq!(processor.process(&String::from("((1))+((1+2+3))+((4+5))")), String::from("1 1 2 + 3 + + 4 5 + +"));
    }

    #[test]
    #[should_panic(expected = "Wrong braces order")]
    fn test_wrong_braces_with_no_operators_between() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;
        // todo: now it returns "11 2 + 3 +4 5 +", have to fix it.
        assert_eq!(processor.process(&String::from("((1))((1+2+3))((4+5))")), String::from(""));
    }

    #[test]
    #[should_panic(expected = "Wrong braces order")]
    fn test_wrong_braces_order_no_closing() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;
        let _ = processor.process(&String::from("("));
    }

    #[test]
    #[should_panic(expected = "Can't find open bracket '(' for closing bracket ')' at id [5]")]
    fn test_wrong_braces_order_no_opening() {
        let processor = Box::new(PostfixReversePolishNotation::new()) as Box<dyn ExpressionProcessor>;
        let _ = processor.process(&String::from("(1+2))"));
    }

}
