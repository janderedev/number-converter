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

use std::{process};
use clap::{Arg, App};

const CHAR_LIST: [&str; 36] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y", "Z"
];

const GRAY: &str = "\u{001b}[30;1m";
const RESET: &str = "\u{001b}[0m";

fn main() {
    let matches = App::new("Base converter thingy")
        .version("1.2")
        .arg(Arg::with_name("INNUM")
             .help("The number that you want to convert to a different number system")
             .required(true)
             .value_name("Input number")
             .index(1))
        .arg(Arg::with_name("INSYS")
             .help("The number system that the input uses")
             .required(true)
             .value_name("Input system")
             .index(2))
        .arg(Arg::with_name("OUTSYS")
             .help("The number system that you want to convert the input to")
             .required(true)
             .value_name("Output system")
             .index(3))
        .arg(Arg::with_name("VERBOSE")
             .short("v")
             .long("verbose")
             .value_name("BOOLEAN")
             .help("Enables verbose output which you can copypaste into a document")
             .takes_value(false))
        .get_matches();

    let in_str = &matches.value_of("INNUM").unwrap().to_string();
    let in_type = &matches.value_of("INSYS").unwrap().to_string();
    let out_type = &matches.value_of("OUTSYS").unwrap().to_string();
    let verbose = matches.is_present("VERBOSE");
    let in_base = parse_base_input(in_type);
    let out_base = parse_base_input(out_type);

    let in_as_dec = to_dec(&str_to_vec(in_str), in_base, verbose);

    if out_base == 10 {
        println!("{} (Base {}) => {}", in_str, in_base, in_as_dec);
        process::exit(0);
    }

    let out_vec = to_other(&in_as_dec, out_base, verbose);
    let output = to_str(&out_vec);

    println!("{} (Base {}) => {} => {} (Base {})", in_str, in_base, in_as_dec, output, out_base);
}

fn parse_base_input(input: &String) -> i32 {
    let result = match input.to_lowercase().as_str() {
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
    };

    if result < 2 || result > 36 {
        println!("Error: Cannot convert from or to a number system with a base of {}.", input);
        process::exit(1);
    }

    return result;
}

fn to_dec(input: &Vec<i32>, base: i32, verbose: bool) -> i32 {
    let mut output: i32 = 0;
    let mut j: i32 = 0;

    if verbose {
        println!("Converting {} (Base {}) to decimal", to_str(input), base);
    }

    for i in 0..input.len() {
        if verbose {
            print!("{}", GRAY);
            if i == 0 {
                print!("{} = ", to_str(input));
            } else {
                for _ in 0..to_str(input).len() as i32 + 1 { print!(" "); }
                print!("+ ");
            }

            print!("{}*{}^{}", input[i], base, j);
        };

        let add_to_output = input[i] * num::pow(base, j as usize);
        j += 1;

        if verbose {
            println!(" = {}", add_to_output);
            print!("{}", RESET);
        }

        output += add_to_output;
    };

    if verbose {
        println!("= {} (Base 10)\n", output);
    }

    return output;
}

fn to_other(input: &i32, base: i32, verbose: bool) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut i = input.clone();

    if verbose {
        println!("Converting {} to Base {}", input, base);
    }

    while i > 0 {
        if verbose {
            print!("{}", GRAY);
            print!("{}", i);
            for _ in 0..input.to_string().len() as i32 - i.to_string().len() as i32 { print!(" "); }
            print!(" / {} = {}", base, i / base);
            for _ in 0..(input / base).to_string().len() as i32 - (i / base).to_string().len() as i32 + 1 { print!(" "); }
            println!("Rest {}",  i % base);
            print!("{}", RESET);
        };

        let rest = i % base;
        i = i / base;
        output.push(rest);
    }

    if verbose {
        println!("= {} (Base {})\n", to_str(&output), base);
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
