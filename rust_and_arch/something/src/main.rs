// Let's build a stack machine! A very simple one

use core::f64;

/// This enum represents an instruction that our virtual stack machine can compute
/// Another line
/// ```rust
/// println!("Hello world");
/// ```
#[derive(Clone, Copy)]
pub enum Instr {
    /// The add instruction pops 2 values of the stack and pushes the sum onto the stack
    ADD,
    SUB,
    MUL,
    DIV,
    /// Pushes a number onto the stack
    PUSH(f64),
    PRINT
}

struct Machine {
    data_stack: Vec<f64>,
    instr_stack: Vec<Instr>,
}

trait StackMachine {
    fn add(&mut self);
    fn sub(&mut self);
    fn div(&mut self);
    fn mul(&mut self);
    fn push(&mut self, val: f64);
    fn print(&mut self);
}

impl StackMachine for Machine {
    
}

impl Machine {
    fn execute(&mut self) {
        for instr in self.instr_stack.clone().iter() {
            match instr {
                Instr::ADD => self.add(),
                Instr::SUB => self.sub(),
                Instr::MUL => self.mul(),
                Instr::DIV => self.div(),
                Instr::PUSH(val) => self.push(*val),
                Instr::PRINT => self.print(),
            }
        }
    }
}

fn main() {
}

#[cfg(test)]
pub mod test {
    use crate::Machine;

    #[test]
    fn test_program() {
        use crate::Instr::*;
        let instr_vec = vec![PUSH(13.0), PUSH(12.0), SUB, PRINT];
        let mut machine = Machine {
            data_stack: vec![],
            instr_stack: instr_vec,
        };

        machine.execute();
    }

    #[test]
    #[should_panic]
    fn test_bad_program() {
        use crate::Instr::*;
        let instr_vec = vec![PUSH(12.0), SUB, PRINT];
        let mut machine = Machine {
            data_stack: vec![],
            instr_stack: instr_vec,
        };

        machine.execute();
    }
}
