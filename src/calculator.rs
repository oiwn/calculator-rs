pub enum Events {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Number(i64),
    Eq,
    Backspace,
    Reset,
    #[allow(dead_code)]
    Idle,
}

#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    Add,
    Sub,
    Mul,
    Div,
    Number(i64),
}

pub struct Calculator {
    ops: Vec<Tokens>,
    accumulator: i64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            ops: vec![],
            accumulator: 0,
        }
    }
}

fn shunting_yard(tokens: Vec<Tokens>) -> Vec<Tokens> {
    let mut output_queue = vec![];
    let mut operator_stack = vec![];

    for token in tokens {
        match token {
            Tokens::Number(n) => output_queue.push(Tokens::Number(n)),
            Tokens::Add | Tokens::Sub => {
                while let Some(top) = operator_stack.last() {
                    if *top == Tokens::Add || *top == Tokens::Sub {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
            Tokens::Mul | Tokens::Div => {
                while let Some(top) = operator_stack.last() {
                    if *top == Tokens::Mul || *top == Tokens::Div {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}

impl Calculator {
    fn calculate(&mut self) -> i64 {
        println!("Ops: {:?}", self.ops.clone());
        println!("Algo: {:?}", shunting_yard(self.ops.clone()));
        let mut stack = vec![];

        for token in shunting_yard(self.ops.clone()) {
            match token {
                Tokens::Number(n) => stack.push(n),
                Tokens::Add => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x + y);
                }
                Tokens::Sub => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x - y);
                }
                Tokens::Mul => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x * y);
                }
                Tokens::Div => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x / y);
                }
            }
        }
        stack.pop().unwrap()
    }

    pub fn display(&self) -> String {
        self.accumulator.to_string()
    }

    pub fn dispatch(&mut self, event: Events) {
        match event {
            Events::Idle => {}
            Events::Eq => {
                // here will be complex logic
                self.ops.push(Tokens::Number(self.accumulator));
                self.accumulator = self.calculate();
                self.ops.clear();
            }
            Events::Reset => {
                self.ops.clear();
                self.accumulator = 0;
            }
            Events::Neg => {
                self.accumulator *= -1;
            }
            Events::Number(num) => {
                if self.accumulator <= 999_999_999_9 {
                    self.accumulator *= 10;
                    self.accumulator += num as i64;
                }
            }
            Events::Backspace => {
                self.accumulator = self.accumulator / 10;
            }
            op @ (Events::Add | Events::Sub | Events::Mul | Events::Div) => {
                // operation first
                let op_token: Option<Tokens> = match op {
                    Events::Add => Some(Tokens::Add),
                    Events::Sub => Some(Tokens::Sub),
                    Events::Mul => Some(Tokens::Mul),
                    Events::Div => Some(Tokens::Div),
                    _ => None,
                };

                self.ops.push(Tokens::Number(self.accumulator));
                self.ops.push(op_token.unwrap());
                self.accumulator = 0
            }
        }
    }
}
