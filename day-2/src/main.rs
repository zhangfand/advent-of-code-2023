use std::collections::hash_map::HashMap;
use std::io;
use log::debug;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: usize,
    runs: Vec<Run>,
}

#[derive(Debug)]
struct Run(HashMap<Color, usize>);

impl Run {
    fn new() -> Self {
        let mut counter = HashMap::new();

        counter.insert(Color::Red, 0);
        counter.insert(Color::Green, 0);
        counter.insert(Color::Blue, 0);

        Run(counter)
    }

    fn from_vec(draws: &[Draw]) -> Self {
        let mut run = Run::new();
        for draw in draws {
            run.0.insert(draw.color, draw.count);
        }

        run
    }
}

struct Draw {
    color: Color,
    count: usize,
}

fn main() {
    env_logger::init();

    part2();
}

fn part1() {
    let given = Run::from_vec(&vec![
        Draw {
            color: Color::Red,
            count: 12,
        },
        Draw {
            color: Color::Green,
            count: 13,
        },
        Draw {
            color: Color::Blue,
            count: 14,
        },
    ]);

    debug!("given: {given:?}");

    let mut sum = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        debug!("line: {line}");
        let game = parse_game(&mut line.as_str()).unwrap();

        let possible = {
            let mut possible = true;

            'outer: for run in game.runs.iter() {
                for (color, count) in run.0.iter() {
                    if count > &given.0[color] {
                        possible = false;
                        break 'outer;
                    }
                }
            }

            possible
        };

        if possible {
            sum += game.id;
        }

        debug!("game: {game:?}; possible: {possible}");
    }

    println!("sum: {:?}", sum);
}

fn part2() {
    let mut sum = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        debug!("line: {line}");

        let game = parse_game(&mut line.as_str()).unwrap();
        let power = get_power_of_minimum_set(&game);

        debug!("game: {game:?}; power: {power}");

        sum += power;
    }

    println!("sum: {:?}", sum);

}

fn get_power_of_minimum_set(game: &Game) -> usize {
    let red = game.runs.iter().map(|r| r.0.get(&Color::Red).unwrap()).max().unwrap();
    let blue = game.runs.iter().map(|r| r.0.get(&Color::Blue).unwrap()).max().unwrap();
    let green = game.runs.iter().map(|r| r.0.get(&Color::Green).unwrap()).max().unwrap();

    return red * blue * green;
}

fn parse_game(input: &mut &str) -> Option<Game> {
    parse_constant(input, "Game ")?;
    let id = parse_number(input)?;
    parse_constant(input, ": ");

    let mut game = Game { runs: vec![], id };
    loop {
        let run = parse_run(input)?;
        game.runs.push(run);

        if parse_constant(input, "; ").is_none() {
            break;
        }
    }

    Some(game)
}

fn parse_run(input: &mut &str) -> Option<Run> {
    let mut run = Run::new();
    loop {
        let draw = parse_draw(input)?;
        let count = run.0.get_mut(&draw.color).unwrap();
        *count += draw.count;

        if parse_constant(input, ", ").is_none() {
            break;
        }
    }

    Some(run)
}

fn parse_draw(input: &mut &str) -> Option<Draw> {
    let count = parse_number(input)?;
    parse_constant(input, " ")?;
    let color = parse_color(input)?;

    Some(Draw { count, color })
}

fn parse_color(input: &mut &str) -> Option<Color> {
    if parse_constant(input, "red").is_some() {
        return Some(Color::Red);
    }
    if parse_constant(input, "green").is_some() {
        return Some(Color::Green);
    }
    if parse_constant(input, "blue").is_some() {
        return Some(Color::Blue);
    }
    None
}

fn parse_constant(input: &mut &str, constant: &str) -> Option<()> {
    if let Some(trimmed) = input.strip_prefix(constant) {
        *input = trimmed;
        debug!("parsed: `{constant}`, rest: `{input}`");
        Some(())
    } else {
        None
    }
}

fn parse_number(input: &mut &str) -> Option<usize> {
    let mut split_index = input.len();

    for (i, c) in input.char_indices() {
        if !c.is_digit(10) {
            split_index = i;
            break;
        }
    }

    let rest = &input[split_index..];
    let digits = &input[..split_index];
    let num = usize::from_str_radix(digits, 10).ok()?;

    debug!("parsed: `{num}`, rest: `{rest}`");
    *input = &input[split_index..];
    Some(num)
}

