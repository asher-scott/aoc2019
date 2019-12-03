extern crate aoc2019;

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_opcode() {
        use aoc2019::day_two::process_opcodes;

        let mut test_case_one = vec![1,0,0,0,99];
        let mut test_case_two = vec![2,3,0,3,99];
        let mut test_case_three = vec![2,4,4,5,99,0];
        let mut test_case_four = vec![1,1,1,4,99,5,6,0,99];

        process_opcodes(&mut test_case_one);
        process_opcodes(&mut test_case_two);
        process_opcodes(&mut test_case_three);
        process_opcodes(&mut test_case_four);

        let output_one = test_case_one.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        let output_two = test_case_two.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        let output_three = test_case_three.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        let output_four = test_case_four.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");

        assert_eq!(output_one, "2,0,0,0,99");
        assert_eq!(output_two, "2,3,0,6,99");
        assert_eq!(output_three, "2,4,4,5,99,9801");
        assert_eq!(output_four, "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn day_two_part_one() {
        use aoc2019::day_two::process_opcodes;
        use std::fs::File;
        use std::io::Read;

        let mut input = String::new();
        let mut file = File::open("./input/input2.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents into string.");
        let mut opcode_input = input.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        process_opcodes(&mut opcode_input);
        println!("Value at position 0 - {}", opcode_input[0]);
    }

    #[test]
    fn day_two_part_two() {
        use aoc2019::day_two::process_opcodes;
        use std::fs::File;
        use std::io::Read;

        let desired_output = 19690720;

        let mut input = String::new();
        let mut file = File::open("./input/input2.txt").expect("Unable to find input file.");
        file.read_to_string(&mut input).expect("Unable to read file contents into string.");

        for noun in 0..99 {
            for verb in 0..99 {
                let mut opcode_input = new_opcode_input(&mut input);
                // set noun and verb
                opcode_input[1] = noun;
                opcode_input[2] = verb;

                process_opcodes(&mut opcode_input);

                if opcode_input[0] == desired_output {
                    println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                    break;
                }
            }
        }
    }

    fn new_opcode_input(input: &mut String) -> Vec<u32> {
        input.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect()
    }
}