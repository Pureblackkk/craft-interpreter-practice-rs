use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::environment::Environment;
use crate::grammer::statement::Stmt;
use crate::runner::error::RunTimeError;
use crate::scanner::token::Token;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,   
    pub locals: HashMap<Token, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        // TODO：add native function clock
        let globals = Rc::new(RefCell::new(Environment::new()));

        Interpreter{
            globals: globals.clone(),
            environment: globals.clone(),
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RunTimeError> {
        for statement in statements {
            let res = self.exectue(&statement)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, token: &Token, depth: usize) {
        self.locals.insert(token.clone(), depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::Resolver;
    use crate::scanner::Scanner;
    use crate::scanner::token::Token;
    use crate::parser::Parser;

    #[test]
    fn simple_statement() {
        let source_expected: Vec<String> = vec![
            // (String::from("
            //     print \"one\";
            //     print true;
            //     print 2 + 1;
            // ")),
            // (String::from("
            //     var a = 1;
            //     var b = 2;
            //     print a + b;
            // ")),
            // (String::from("
            //     var a = 1;
            //     var c = 2;
            //     a = 2;
            //     a = c = 3;
            // ")),
            // (String::from("
            //     var a = \"global a\";
            //     {
            //         var a = \"inner a\";
            //         print a;
            //     }
            //     print a;
            // ")),
            // (String::from("
            //     var a = \"global a\";
            //     var b = \"global b\";
            //     {
            //         var a = \"outer a\";
            //         var b = \"outer b\";
            //         {
            //             var a = \"inner a\";
            //             print b;                        
            //         }
            //         print a;
            //         print b;
            //     }
            //     print a;
            //     print b;
            // ")),
            // (String::from("
            //     var a = \"a\";
            //     var b = \"b\";
            //     if (1)
            //         if (a = b)
            //             print b;
            //         else 
            //             print a;
            //     else
            //         print 2 + 1;
            // ")),
            // (String::from("
            //     var a = 1;
            //     var b = 1;
            //     print (a <= b);
            // ")),
            // (String::from("
            //     var a = 1;
            //     while (a < 5) {
            //         print a;
            //         a = a + 1;
            //     }
            // ")),
            // (String::from("
            //     for (var i = 1; i < 5; i = i + 1) {
            //         print \" while \";
            //         print i;
            //     }
            // ")),
            // (String::from("
            //     fun add(a, b) {
            //         print a + b;
            //     }

            //     add(2.1, 3.0);
            // ")),
            // (String::from("
            //     fun add(a, b) {
            //         print a + b;
            //     }

            //     var a = add(2.1, 3.0);
            //     print a;
            // ")),
            // (String::from("
            //     fun fib(n) {
            //         if (n <= 1) return n;
            //         return fib(n - 2) + fib(n - 1);
            //     }

            //     print fib(3);
            // ")),
            // (String::from("
            //     fun makeCounter() {
            //         var i = 0;
            //         fun count() {
            //             i = i + 1;
            //             print i;
            //         }

            //         return count;
            //     }

            //     var counter = makeCounter();
            //     counter();
            //     counter();
            //     counter();
            //     counter();
            // ")),
            // (String::from("
            //   class DevonshireCream {
            //     serveOn() {
            //         return \" Scones\";
            //     }
            //   }

            //   print DevonshireCream;
            // ")),
            // (String::from("
            //     class Counter {
            //         init() {
            //             this.count = 0;
            //             return;
            //         }

            //         addCount() {
            //             this.count = this.count + 1;
            //         }

            //         console() {
            //             print this.count;
            //         }
            //     }

            //     var counter = new Counter();
            //     counter.addCount();
            //     counter.addCount();
            //     counter.addCount();

            //     counter.console();
            // ")),
            // (String::from("
            //     class Father {
            //         init() {
            //             this.name = \"Father\";
            //             this.money = 100;
            //         }

            //         console1() {
            //             print \"Father\";
            //         }
            //     }

            //     class Child extend Father {
            //         init() {
            //             this.name = \"Child\";
            //         }

            //         console() {
            //             super.console1();
            //         }
            //     }

            //     var child = new Child();
            //     child.console();
            // ")),
            (String::from("
                fun fib(n) {
                    if (n < 2) return n;
                    return fib(n - 1) + fib(n - 2);
                }

                print fib(36);
            ")),
        ];

        for source in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let statements = Parser::new(tokens).parser().unwrap();
            
            // Resolve
            let mut interpreter = Interpreter::new();
            let mut resolver = Resolver::new(&mut interpreter);
            resolver.resolve(&statements).unwrap();

            // Interpret
            interpreter.interpret(statements).unwrap();     
        }
    } 
}