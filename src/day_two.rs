pub fn process_opcodes(opcode_input: &mut Vec<u32>) {
    let mut ptr_index = 0;
    while opcode_input[ptr_index] != 99 {
        let operation = opcode_input[ptr_index];
        let loc_one = opcode_input[(ptr_index + 1)] as usize;
        let loc_two = opcode_input[(ptr_index + 2)] as usize;
        let result_location = opcode_input[(ptr_index + 3)] as usize;

        let operation_result = match operation {
            1 => opcode_input[loc_one] + opcode_input[loc_two],
            2 => opcode_input[loc_one] * opcode_input[loc_two],
            _ => panic!("Cannot parse operation")
        };

        opcode_input[result_location] = operation_result;
        ptr_index += 4;
    }
}