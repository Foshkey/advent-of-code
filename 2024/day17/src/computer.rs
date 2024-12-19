#[derive(Clone, Debug)]
pub struct Computer {
    pub output: Vec<u8>,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<u8>,
    instruction_pointer: usize,
    jumped: bool,
}

impl Computer {
    pub fn find_a(&self) -> usize {
        todo!()
    }

    pub fn execute(&mut self) {
        while let Some(&opcode) = self.program.get(self.instruction_pointer) {
            let Some(&operand) = self.program.get(self.instruction_pointer + 1) else {
                break;
            };

            self.execute_instruction(opcode, operand);

            if !self.jumped {
                self.instruction_pointer += 2;
            } else {
                self.jumped = false;
            }
        }
    }

    fn execute_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => (),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.register_a /= 2usize.pow(self.combo(operand) as u32);
    }

    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as usize;
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo(operand) % 8;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
            self.jumped = true;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.register_b ^= self.register_c;
    }

    fn out(&mut self, operand: u8) {
        self.output.push((self.combo(operand) % 8) as u8);
    }

    fn bdv(&mut self, operand: u8) {
        self.register_b = self.register_a / 2usize.pow(self.combo(operand) as u32);
    }

    fn cdv(&mut self, operand: u8) {
        self.register_c = self.register_a / 2usize.pow(self.combo(operand) as u32);
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Combo operand 7 is reserved."),
            o => o as usize,
        }
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
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            program,
            instruction_pointer: 0,
            jumped: false,
        }
    }
}
