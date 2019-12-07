extern crate aoc2019;

#[cfg(test)]
mod tests {
    use aoc2019::day_five::IntcodeComputer;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_intcode_computer_new() {
        let intcode = String::from("1,0,0,0,99"); 
        let intcode_computer = IntcodeComputer::new(intcode);
        assert_eq!(intcode_computer.memory.len(), 5);
        assert_eq!(intcode_computer.output.len(), 0);
        assert_eq!(intcode_computer.stack_ptr, 0);
    }

    #[test]
    fn test_intcode_input_and_output() {
        let intcode = String::from("3,0,4,0,99");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(10);
        assert_eq!(intcode_computer.output.len(), 1);
        assert_eq!(intcode_computer.output[0], 10);
    }

    #[test]
    fn day_five_part_one() {
        let mut input = String::new();
        let mut file = File::open("./input/input5.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents into string."); 

        let mut intcode_computer = IntcodeComputer::new(input);
        intcode_computer.run(1);

        for i in intcode_computer.output {
            println!("{}", i);
        }
    }

    #[test]
    fn test_input_equal_to_eight_positional() {
        let intcode = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(8);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn test_input_equal_to_eight_immediate() {
        let intcode = String::from("3,3,1108,-1,8,3,4,3,99");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(8);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn test_input_less_than_eight_positional() {
        let intcode = String::from("3,9,7,9,10,9,4,9,99,-1,8");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(6);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn test_input_less_than_eight_immediate() {
        let intcode = String::from("3,3,1107,-1,8,3,4,3,99");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(6);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn test_input_jump_if_nonzero_positional() {
        let intcode = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(6);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn test_input_jump_if_nonzero_immediate() {
        let intcode = String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let mut intcode_computer = IntcodeComputer::new(intcode);
        intcode_computer.run(6);

        assert_eq!(intcode_computer.output[0], 1)
    }

    #[test]
    fn day_five_part_two() {
        let mut input = String::new();
        let mut file = File::open("./input/input5.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents into string."); 

        let mut intcode_computer = IntcodeComputer::new(input);
        intcode_computer.run(5);

        for i in intcode_computer.output {
            println!("{}", i);
        }
    }
}