fn main() {
    let input = [ 15, 14, 12, 13 ].to_vec();
    println!("{} -> {}", to_str(&input), to_dec(&input, 16));
}

fn to_dec(input: &Vec<i32>, base: i32) -> i32 {
    let mut output: i32 = 0;
    let mut j: i32 = 0;

    for i in 0..input.len() {
        output += input[i] * num::pow(base, j as usize);
        j += 1;
    };

    return output;
}

fn to_other(input: i32, base: i32) -> Vec<i32> {
    let mut output: Vec<i32> = [].to_vec();
    let mut i = input;

    while i > 0 {
        let rest = i % base;
        i = i / 2;
        output.push(rest);
    }

    return output;
}

const CHAR_LIST: [&str; 16] = [ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F" ];
fn to_str(input: &Vec<i32>) -> String {
    let mut output = "".to_string();

    for i in (0..input.len()).rev() {
        let j = input[i] as usize;
        output += CHAR_LIST[j];
    };

    return output;
}
