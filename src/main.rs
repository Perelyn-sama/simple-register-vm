use std::fmt;

#[derive(Debug)]
pub enum VMError {
    InvalidRegister,
    DivisionByZero,
    InvalidProgramCounter,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(usize),
    Immediate(i64),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(usize, Operand),
    Add(usize, Operand),
    Sub(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Cmp(usize, Operand),
    Jmp(usize),
    Je(usize),
    Jne(usize),
    Jg(usize),
    Jl(usize),
    Halt,
}

pub struct RegisterVM {
    registers: [i64; 10],
    pc: usize,
    cmp_flag: i8,
    memory: Vec<Instruction>,
}

impl fmt::Debug for RegisterVM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program Counter: {}", self.pc)?;
        writeln!(f, "Comparison Flag: {}", self.cmp_flag)?;
        writeln!(f, "Registers:")?;
        for (i, reg) in self.registers.iter().enumerate() {
            writeln!(f, "R{}: {}", i, reg)?;
        }
        Ok(())
    }
}

impl RegisterVM {
    pub fn new() -> Self {
        RegisterVM {
            registers: [0; 10],
            pc: 0,
            cmp_flag: 0,
            memory: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.memory = program;
        self.pc = 0;
    }

    fn get_operand_value(&self, operand: &Operand) -> Result<i64, VMError> {
        match operand {
            Operand::Register(reg) => {
                if *reg < self.registers.len() {
                    Ok(self.registers[*reg])
                } else {
                    Err(VMError::InvalidRegister)
                }
            }
            Operand::Immediate(value) => Ok(*value),
        }
    }

    fn fetch(&mut self) -> Option<Instruction> {
        if self.pc < self.memory.len() {
            let instruction = self.memory[self.pc].clone();
            self.pc += 1;
            Some(instruction)
        } else {
            None
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<bool, VMError> {
        match instruction {
            Instruction::Mov(dest, src) => {
                if dest >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                self.registers[dest] = self.get_operand_value(&src)?;
            }

            Instruction::Add(dest, src) => {
                if dest >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                self.registers[dest] += self.get_operand_value(&src)?;
            }

            Instruction::Sub(dest, src) => {
                if dest >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                self.registers[dest] -= self.get_operand_value(&src)?;
            }

            Instruction::Mul(dest, src) => {
                if dest >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                self.registers[dest] *= self.get_operand_value(&src)?;
            }

            Instruction::Div(dest, src) => {
                if dest >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                let divisor = self.get_operand_value(&src)?;
                if divisor == 0 {
                    return Err(VMError::DivisionByZero);
                }

                self.registers[dest] /= divisor;
            }

            Instruction::Cmp(reg1, src) => {
                if reg1 >= self.registers.len() {
                    return Err(VMError::InvalidRegister);
                }
                let val1 = self.registers[reg1];
                let val2 = self.get_operand_value(&src)?;
                self.cmp_flag = (val1 > val2) as i8 - (val1 - val2) as i8;
            }

            Instruction::Jmp(addr) => {
                if addr >= self.memory.len() {
                    return Err(VMError::InvalidProgramCounter);
                }

                self.pc = addr;
            }

            Instruction::Je(addr) => {
                if self.cmp_flag == 0 {
                    if addr >= self.memory.len() {
                        return Err(VMError::InvalidProgramCounter);
                    }
                    self.pc = addr;
                }
            }

            Instruction::Jne(addr) => {
                if self.cmp_flag != 0 {
                    if addr >= self.memory.len() {
                        return Err(VMError::InvalidProgramCounter);
                    }
                    self.pc = addr;
                }
            }

            Instruction::Jg(addr) => {
                if self.cmp_flag > 0 {
                    if addr >= self.memory.len() {
                        return Err(VMError::InvalidProgramCounter);
                    }
                    self.pc = addr;
                }
            }
            Instruction::Jl(addr) => {
                if self.cmp_flag < 0 {
                    if addr >= self.memory.len() {
                        return Err(VMError::InvalidProgramCounter);
                    }
                    self.pc = addr;
                }
            }

            Instruction::Halt => return Ok(false),
        }

        Ok(true)
    }

    pub fn run(&mut self) -> Result<i64, VMError> {
        loop {
            if let Some(instruction) = self.fetch() {
                if !self.execute(instruction)? {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(self.registers[0])
    }
}

fn main() -> Result<(), VMError> {
    // Program to calculate factorial of 5
    let factorial_program = vec![
        Instruction::Mov(0, Operand::Immediate(5)), // R0 = 5 (input)
        Instruction::Mov(1, Operand::Immediate(1)), // R1 = 1 (result)
        Instruction::Cmp(0, Operand::Immediate(0)), // Compare R0 with 0
        Instruction::Je(7),                         // If R0 == 0, jump to Halt
        Instruction::Mul(1, Operand::Register(0)),  // R1 *= R0
        Instruction::Sub(0, Operand::Immediate(1)), // R0 -= 1
        Instruction::Jmp(2),                        // Jump back to comparison
        Instruction::Mov(0, Operand::Register(1)),  // Move result to R0
        Instruction::Halt,
    ];

    let mut vm = RegisterVM::new();
    vm.load_program(factorial_program);
    let result = vm.run()?;
    println!("Factorial of 5: {}", result); // should print 120

    Ok(())
}
