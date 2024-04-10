#![allow(unused)]

use std::collections::HashMap;

/// 线性数据结构：数组、链表、栈、队列等

/// 栈
#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

/// 栈的使用案例1：括号匹配
pub fn bracket_match(s: &str) -> bool {
    let mut stack = Stack::new();

    let mut match_ruls = HashMap::new();
    match_ruls.insert(')', '(');
    match_ruls.insert(']', '[');
    match_ruls.insert('}', '{');
    match_ruls.insert('>', '<');

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if stack.is_empty() {
                    return false;
                } else {
                    let last_c = match stack.peek() {
                        Some(v) => v,
                        None => return false,
                    };

                    if let Some(v) = match_ruls.get(&c) {
                        if v == last_c {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

/// 栈的使用案例2：中缀转后缀表达式
pub fn infix_to_postfix(s: &str) -> Option<String> {
    // 使用 Stack 保存操作符
    let mut op_stack: Stack<&str> = Stack::new();
    //  后缀字符串
    let mut postfix: Vec<&str> = Vec::new();

    // 设置操作符优先级
    let mut op_priority: HashMap<&str, i32> = HashMap::new();
    op_priority.insert("+", 1);
    op_priority.insert("-", 1);
    op_priority.insert("*", 2);
    op_priority.insert("/", 2);
    op_priority.insert("(", 0);
    op_priority.insert(")", 0);

    // 遍历中缀字符串
    for token in s.split("") {
        if (token >= "0" && token <= "9") || (token >= "A" && token <= "Z") || token == "." {
            postfix.push(token);
        } else if token == "+" || token == "-" || token == "*" || token == "/" {
            if (!op_stack.is_empty() && op_priority[token] <= op_priority[op_stack.peek().unwrap()])
            {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            while let Some(c) = op_stack.pop() {
                if c == "(" {
                    break;
                } else {
                    postfix.push(c);
                }
            }
        }
        println!("{:?}", postfix);
        println!("{:?}", op_stack);
    }

    //  剩余操作符入栈
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    Some(postfix.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_match_test() {
        let str1 = "()(()())";
        let err_str1 = "()()(";
        let str2 = "(()()[]){<>[]}";
        let err_str2 = "()[]{<}>[]}";
        assert_eq!(bracket_match(str1), true);
        assert_eq!(bracket_match(err_str1), false);
        assert_eq!(bracket_match(str2), true);
        assert_eq!(bracket_match(err_str2), false);
    }

    #[test]
    fn infix_to_postfix_test() {
        let infix = "(A+B)*(C+D)";

        match infix_to_postfix(infix) {
            Some(val) => {
              assert_eq!(val, String::from("A B + C D + *"));
              println!("Postfix result: {}", val)
            },
            None => println!("None"),
        }
    }
}
