pub fn part1(input: &str) -> usize {
    let program = parse(input);

    run(program, 12, 2)
}

pub fn part2(input: &str) -> usize {
    let program = parse(input);

    for noun in 0.. {
        for verb in 0..=noun {
            if run(program.clone(), noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn run(mut program: Vec<usize>, noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;

    let mut ic = 0;
    loop {
        let opcode = program[ic];
        let p1 = program[ic + 1];
        let p2 = program[ic + 2];
        let p3 = program[ic + 3];
        match opcode {
            1 => program[p3] = program[p1] + program[p2],
            2 => program[p3] = program[p1] * program[p2],
            99 => break,
            _ => panic!(),
        }

        ic += 4;
    }

    program[0]
}

fn parse(input: &str) -> Vec<usize> {
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

#[test]
fn test_part2() {
    assert_eq!(6533, part2(include_str!("input.txt")));
}
