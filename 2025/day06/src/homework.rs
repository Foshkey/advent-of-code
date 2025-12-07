#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Multiply,
}

impl Default for Op {
    fn default() -> Self {
        Self::Add
    }
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("Unexpected character: {c}"),
        }
    }
}

impl Op {
    pub fn get_init(&self) -> usize {
        match self {
            Op::Add => 0,
            Op::Multiply => 1,
        }
    }

    pub fn execute(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

#[derive(Default, Debug)]
struct Problem {
    operands: Vec<usize>,
    op: Op,
}

impl Problem {
    pub fn new(op: Op) -> Self {
        Self {
            operands: vec![0],
            op,
        }
    }
}

pub struct Homework {
    problems: Vec<Problem>,
}

impl From<&str> for Homework {
    fn from(s: &str) -> Self {
        let mut problems = Vec::new();

        for line in s.lines() {
            for (i, entry) in line.split_whitespace().enumerate() {
                let problem = if i < problems.len() {
                    &mut problems[i]
                } else {
                    problems.push(Problem::default());
                    problems.last_mut().unwrap()
                };

                if let Ok(num) = entry.parse() {
                    problem.operands.push(num);
                } else if entry == "*" {
                    problem.op = Op::Multiply;
                }
            }
        }

        Self { problems }
    }
}

impl Homework {
    pub fn parse_cephalopod(input: &str) -> Self {
        // Read ops first to determine number length
        let mut lines: Vec<_> = input.lines().collect();
        let Some(op_line) = lines.pop() else {
            panic!("No lines in input");
        };

        let mut problems = init_problems(op_line);

        println!("{problems:?}");

        Self { problems }
    }

    pub fn solve(&self) -> usize {
        self.problems
            .iter()
            .map(|p| {
                p.operands
                    .iter()
                    .fold(p.op.get_init(), |acc, &x| p.op.execute(acc, x))
            })
            .sum()
    }
}

fn init_problems(op_line: &str) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut current_problem: Option<Problem> = None;

    for c in op_line.chars() {
        match c {
            ' ' => {
                if let Some(problem) = current_problem.as_mut() {
                    problem.operands.push(0);
                }
            }
            c => {
                if let Some(mut problem) = current_problem {
                    problem.operands.pop();
                    problems.push(problem)
                }

                current_problem = Some(Problem::new(c.into()));
            }
        }
    }

    if let Some(problem) = current_problem {
        problems.push(problem)
    }

    problems
}
