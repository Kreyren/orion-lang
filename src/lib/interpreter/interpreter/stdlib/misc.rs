use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;

impl Interpreter {
    pub fn assert(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }

        if let Value::Bool(b) = &args[0] {
            if *b {
                return Ok(Value::Nil);
            } else {
                panic!("Assertion failed.")
            }
        } else {
            panic!("Assertion failed.")
        }
    }

    pub fn _typeof(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }
        Ok(Value::String(args[0].get_type()))
    }

    pub fn cast(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        if let Value::String(s) = &args[0] {
            Ok(
                args[1].cast_to(s)?
            )
        } else {
            return Err(
                crate::error!("Invalid argument, expected string,  found", (args[0].get_type()))
            )
        }
    }

    pub fn import(&mut self, args: &Vec<Value>) -> crate::Result<Value> {

        use std::path::Path;

        for arg in args {
            if let Value::String(s) = arg {
                if !Path::new(&s).exists() {
                    return Err(
                        crate::error!("Cannot find file `", s, "`.")
                    )
                }

                let content = match std::fs::read_to_string(&s) {
                    Ok(c) => c,
                    Err(e) => return Err(crate::error!(e)),
                };

                let mut lexer = crate::lexer::lexer::Lexer::new(content);
                let tokens = lexer.scan_tokens();
                let mut parser = crate::parser::parser::Parser::new(tokens);
                let ast = parser.parse_tokens()?;
                self.eval_calls(&ast.children)?; // Delete the Scope.
            } else {
                return Err(
                    crate::error!("Invalid argument, expected string,  found", (arg.get_type()))
                )
            }
        }

        Ok(Value::Nil)
    }
}