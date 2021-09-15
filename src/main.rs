/*  
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use std::process;

const CHAR_LIST: [&str; 16] = [ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F" ];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("\u{001b}[1mUsage:\u{001b}[0m \u{001b}[36m{} [Input number] [Input type] [Output type]\u{001b}[0m", args[0]);
        print!("Input and output type can be any of ");
        print!("\u{001b}[33m\"hex\"\u{001b}[0m, \u{001b}[33m\"dec\"\u{001b}[0m, \u{001b}[33m\"oct\"\u{001b}[0m, \u{001b}[33m\"bin\"\u{001b}[0m ");
        println!("or any number from 2 to 16.");
        println!("\u{001b}[1mExample:\u{001b}[0m \u{001b}[36m{} DCEF hex bin\u{001b}[0m", args[0]);
        process::exit(1);
    }

    if args.len() < 4 {
        println!("Expected 3 arguments; received {}.", args.len() - 1);
        process::exit(1);
    }

    if args.len() > 4 {
        println!("Expected 3 arguments; received {}. Additional arguments have been discarded.", args.len() - 1);
    }

    let in_str = &args[1].to_uppercase();
    let in_type = &args[2];
    let out_type = &args[3];
    let in_base = parse_base_input(in_type);
    let out_base = parse_base_input(out_type);

    let in_as_dec = to_dec(&str_to_vec(in_str), in_base);

    if out_base == 10 {
        println!("{} (Base {}) => {}", in_str, in_base, in_as_dec);
        process::exit(0);
    }

    let out_vec = to_other(&in_as_dec, out_base);
    let output = to_str(&out_vec);

    println!("{} (Base {}) => {} => {} (Base {})", in_str, in_base, in_as_dec, output, out_base);
}

fn parse_base_input(input: &String) -> i32 {
    match input.to_lowercase().as_str() {
        "hex" => 16,
        "dec" => 10,
        "oct" => 8,
        "bin" => 2,
        _ => {
            match input.parse::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Error: Failed to parse \"{}\".", input);
                    process::exit(1);
                }
            }
        }
    }
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

fn to_other(input: &i32, base: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut i = input.clone();

    while i > 0 {
        let rest = i % base;
        i = i / base;
        output.push(rest);
    }

    return output;
}

fn str_to_vec(input: &str) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let chars = input.split("").collect::<Vec<&str>>();
    
    for i in (0..chars.len()).rev() {
        if chars[i] != "" {
            output.push(find_arr_index(&CHAR_LIST, chars[i]));
        }
    };

    return output;
}

fn to_str(input: &Vec<i32>) -> String {
    let mut output = "".to_string();

    for i in (0..input.len()).rev() {
        let j = input[i] as usize;
        output += CHAR_LIST[j];
    };

    return output;
}

fn find_arr_index<T>(arr: &[T], item: T) -> i32 where T: std::cmp::PartialEq {
    for i in 0..arr.len() {
        if arr[i] == item {
            return i as i32;
        }
    };
    return -1;
}
