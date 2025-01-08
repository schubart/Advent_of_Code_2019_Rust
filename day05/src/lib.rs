pub fn part1(input: &str) -> isize {
    let program = parse(input);

    run(program).last().copied().unwrap()
}

fn run(mut program: Vec<isize>) -> Vec<isize> {
    let mut output = Vec::new();

    let mut ic = 0;
    loop {
        let instruction = program[inc(&mut ic)];
        let opcode = instruction % 100;
        let mut mode = instruction / 100;
        eprintln!("{ic} {instruction} {opcode}");

        let mut get_parameter = || {
            let result = program[inc(&mut ic)];

            let result = match mode % 10 {
                0 => result,
                1 => program[result as usize],
                _ => panic!(),
            };

            mode /= 10;

            result
        };

        match opcode {
            1 => program[inc(&mut ic)] = get_parameter() + get_parameter(),
            2 => program[inc(&mut ic)] = get_parameter() * get_parameter(),
            3 => program[inc(&mut ic)] = 1, // Hardcoded input for this day.
            4 => output.push(get_parameter()),
            99 => break,
            _ => panic!(),
        }
    }

    output
}

fn inc(value: &mut usize) -> usize {
    *value += 1;
    *value - 1
}

fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(3790689, part1(include_str!("input.txt")));
}
