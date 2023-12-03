use std::io;

fn main() {
    part_2();
}


fn part_1() {
    let mut buf = String::new();
    let mut sum = 0;

    loop {
        let result = io::stdin().read_line(&mut buf);
        let size = result.expect("failed to read from the file");
        if size == 0 {
            break
        }
        
        let first_i = buf.find(|c: char| c.is_digit(10)).expect("failed to find first digit]");
        let first_digit = buf.chars().nth(first_i).unwrap().to_digit(10).unwrap();
        let last_i = buf.rfind(|c: char| c.is_digit(10)).expect("failed to find the last digit]");
        let last_digit = buf.chars().nth(last_i).unwrap().to_digit(10).unwrap();
        buf.clear();

        println!("first digit: {}; last digit: {}", first_digit, last_digit);
        sum += first_digit * 10 + last_digit; 
    }

    println!("sum is {}", sum);
}


fn part_2() {
    let mut sum = 0;

    let texts = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    loop {
        let mut buf = String::new();
        let result = io::stdin().read_line(&mut buf);
        let size = result.expect("failed to read from the file");
        if size == 0 {
            break
        }

        let mut to_parse: &str = buf.as_str();
        let mut parsed = vec![];
        'outer: while to_parse.len() > 0 {
            let c = to_parse.chars().nth(0).unwrap();
            if let Some(num) = c.to_digit(10) {
                parsed.push(num);
                to_parse = &to_parse[1..];
                continue 'outer
            } 
            // the first char is not a digit
            for (index, text) in texts.iter().enumerate() {
                let num = (index + 1) as u32;
                let size = text.len();
                if to_parse.starts_with(text) {
                    parsed.push(num);
                    to_parse = &to_parse[1..];
                    continue 'outer
                }
            }
            // Neither, moves forward
            to_parse = &to_parse[1..];
        }

        
        let first_digit = parsed[0];
        let last_digit = parsed[parsed.len()-1];

        println!("input: {}", buf.trim());
        println!("after parsing: {:?}", parsed);
        println!("first digit: {}; last digit: {}", first_digit, last_digit);
        println!("");
        sum += first_digit * 10 + last_digit; 
    }

    println!("sum is {}", sum);
}
