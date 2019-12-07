pub struct IntcodeComputer {
    pub memory: Vec<i32>,
    pub output: Vec<i32>,
    pub stack_ptr: usize
}

impl IntcodeComputer {
    pub fn new(intcode: String) -> IntcodeComputer {
        let memory = intcode.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        IntcodeComputer { memory: memory, output: Vec::new(), stack_ptr: 0 }
    }

    pub fn run(&mut self, input: i32) {
        while self.memory[self.stack_ptr] != 99 {
            let opcode = self.memory[self.stack_ptr];
            let (operation, param_one_mode, param_two_mode) = self.parse_opcode(opcode); 
            match operation {
                1 => self.add_operation(param_one_mode, param_two_mode),
                2 => self.multiply_operation(param_one_mode, param_two_mode),
                3 => self.input_operation(input),
                4 => self.output_operation(param_one_mode),
                5 => self.jump_if_true_operation(param_one_mode, param_two_mode),
                6 => self.jump_if_false_operation(param_one_mode, param_two_mode),
                7 => self.less_than_operation(param_one_mode, param_two_mode),
                8 => self.equal_to_operation(param_one_mode, param_two_mode),
                _ => panic!("Unrecognized operation")
            };
        }
    }

    fn parse_opcode(&mut self, opcode: i32) -> (u8, u8, u8) {
        let mut operation_values = vec![0,0,0,0,0];
        let opcode_string = opcode.to_string();

         for (i, c) in opcode_string.chars().rev().enumerate() {
             operation_values[i] = c.to_string().parse::<u8>().unwrap();
         }

        (operation_values[0], operation_values[2], operation_values[3])
    }

    fn add_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];
        let result_location = self.memory[self.stack_ptr + 3] as usize;

        let left_operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unknown parameter mode for param_one")
        };

        let right_operand = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unknown parameter mode for param_two")
        };
    
        self.memory[result_location] = left_operand + right_operand;
        self.stack_ptr += 4;
    }

    fn multiply_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];
        let result_location = self.memory[self.stack_ptr + 3] as usize;

        let left_operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unknown parameter mode for param_one")
        };

        let right_operand = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unknown parameter mode for param_two")
        };
    
        self.memory[result_location] = left_operand * right_operand;
        self.stack_ptr += 4;
    }

    fn input_operation(&mut self, input: i32) {
        let location = self.memory[self.stack_ptr + 1] as usize;
        self.memory[location] = input;
        self.stack_ptr += 2;
    }

    fn output_operation(&mut self, param_one_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];

        match param_one_mode {
            0 => self.output.push(self.memory[param_one as usize]),
            1 => self.output.push(param_one),
            _ => panic!("Unrecognized parameter mode")
        };

        self.stack_ptr += 2;
    }

    fn jump_if_true_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];

        let operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unrecognized parameter mode")
        };

        let new_location = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unrecognized parameter mode")
        };

        if operand != 0 {
            self.stack_ptr = new_location as usize;
        } else {
            self.stack_ptr += 3
        }

    }

    fn jump_if_false_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];

        let operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unrecognized parameter mode")
        };

        let new_location = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unrecognized parameter mode")
        };

        if operand == 0 {
            self.stack_ptr = new_location as usize;
        } else {
            self.stack_ptr += 3
        }

    }

    fn less_than_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];
        let location = self.memory[self.stack_ptr + 3];

        let left_operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unrecognized parameter mode")
        };

        let right_operand = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unrecognized parameter mode")
        };

        if left_operand < right_operand {
            self.memory[location as usize] = 1;
        } else {
            self.memory[location as usize] = 0;
        }

        self.stack_ptr += 4;
    }

    fn equal_to_operation(&mut self, param_one_mode: u8, param_two_mode: u8) {
        let param_one = self.memory[self.stack_ptr + 1];
        let param_two = self.memory[self.stack_ptr + 2];
        let location = self.memory[self.stack_ptr + 3];

        let left_operand = match param_one_mode {
            0 => self.memory[param_one as usize],
            1 => param_one,
            _ => panic!("Unrecognized parameter mode")
        };

        let right_operand = match param_two_mode {
            0 => self.memory[param_two as usize],
            1 => param_two,
            _ => panic!("Unrecognized parameter mode")
        };

        if left_operand == right_operand {
            self.memory[location as usize] = 1;
        } else {
            self.memory[location as usize] = 0;
        }

        self.stack_ptr += 4;
    }
} 