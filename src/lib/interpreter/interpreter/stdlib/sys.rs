use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::*;
use std::io;

impl Interpreter {
    pub fn breakpoint(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() == 1 {
            if let Value::String(s) = &args[0] {
                self.breakpnt(&s);
                Ok(
                    Value::Nil
                )
            } else {
                Err(
                    error!("Invalid argument, expected string, found", (&args[0].get_type()))
                )
            }
        } else if args.len() == 0 {
            self.breakpnt("UNNAMED_BREAKPOINT");
            Ok(Value::Nil)
        } else {
            Err(
                error!("Invalid number of arguments, expected 1|0, found", (args.len()))
            )
        }
    }
    fn breakpnt(&mut self, name: &str) {
        println!("\x1b[0;31m\x1b[1m== Program hit breakpoint '{}' ==\x1b[0m", name);

        println!("\nColour codes:\n- \x1b[0;31mred: immutable variables\x1b[0;32m\n- green: mutable variables\n\x1b[0m");

        println!("\nScopes: {}", self.print_scopes());
        println!("\x1b[0;31m\x1b[1m== End of scopes ==\x1b[0m");
        println!("\nPress enter key to continue ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    fn print_scopes(&mut self) -> String {
        let mut toret = String::new();
        for scope in &self.scopes {
            toret.push_str("{\n");
            for (key, (value, mutable)) in scope {
                if *mutable {
                    toret.push_str(
                        format!("\t\x1b[0;32m{} => {} \x1b[0;34m({})\x1b[0m\n", key, value, value.get_type()).as_str()
                    )
                } else {
                    toret.push_str(
                        format!("\t\x1b[0;31m{} => {} \x1b[0;34m({})\x1b[0m\n", key, value, value.get_type()).as_str()
                    )
                }
            }
            toret.push_str("},");
        }
        toret.pop();
        toret.push('\n');
        toret
    }
}