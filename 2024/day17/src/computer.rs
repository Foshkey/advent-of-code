use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Computer {
    pub output: Vec<u8>,
    registers: [usize; 3],
    program: Vec<u8>,
    instruction_pointer: usize,
    jumped: bool,
}

impl Computer {
    // Part 1

    pub fn execute(&mut self) {
        while let Some((opcode, operand)) = self.get_instruction() {
            self.execute_instruction(opcode, operand);

            if !self.jumped {
                self.instruction_pointer += 2;
            } else {
                self.jumped = false;
            }
        }
    }

    fn get_instruction(&self) -> Option<(u8, u8)> {
        let opcode = self.program.get(self.instruction_pointer)?;
        let operand = self.program.get(self.instruction_pointer + 1)?;
        Some((*opcode, *operand))
    }

    fn execute_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.registers[0] /= 2usize.pow(self.combo(operand) as u32),
            1 => self.registers[1] ^= operand as usize,
            2 => self.registers[1] = self.combo(operand) % 8,
            3 if self.registers[0] != 0 => {
                self.instruction_pointer = operand as usize;
                self.jumped = true;
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push((self.combo(operand) % 8) as u8),
            6 => self.registers[1] = self.registers[0] / 2usize.pow(self.combo(operand) as u32),
            7 => self.registers[2] = self.registers[0] / 2usize.pow(self.combo(operand) as u32),
            _ => (),
        }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => panic!("Combo operand 7 is reserved."),
            o => o as usize,
        }
    }

    // Part 2

    pub fn find_a(&self) -> usize {
        // This is a specific solution for part 2. Analyzing the program, it encodes
        // the last 3 binary digits of A, outputs it, and then shifts A over 3 digits.
        // The reversal of that is starting with the end of the program, see what value
        // of A matches, and then shifting over 3 binary digits (multiply by 8).

        // Set up a FIFO queue
        let mut queue = VecDeque::from([0]);
        while let Some(a) = queue.pop_front() {
            // Check 0-7 (each octal digit)
            for d in 0..0o10 {
                // Add the digit to A and try it
                let a = a + d;
                let output = self.execute_with(a);

                // If it matches, hooray we found it. Reverse priority queue means this
                // should be the lowest possible number.
                if output == self.program {
                    return a;
                }

                // Check if we at least match the end of the program.
                if matches_end(&self.program, &output) {
                    // If so, then shift A and push it into the queue
                    queue.push_back(a * 0o10);
                }
            }
        }

        0
    }

    fn execute_with(&self, a: usize) -> Vec<u8> {
        let mut computer = self.clone();
        computer.registers[0] = a;
        computer.execute();
        computer.output
    }
}

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let mut registers = [0usize; 3];
        for register in &mut registers {
            *register = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
        }

        lines.next(); // Skip the empty line
        let program = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Computer {
            output: Vec::new(),
            registers,
            program,
            instruction_pointer: 0,
            jumped: false,
        }
    }
}

fn matches_end<T: PartialEq>(main_vec: &[T], suffix: &[T]) -> bool {
    let start_index = main_vec.len() - suffix.len();
    let main_slice = &main_vec[start_index..];
    main_slice == suffix
}
