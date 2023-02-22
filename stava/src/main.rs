use std::{io, cmp, string, fs};
use std::io::{prelude::*, BufReader};

/// Kattis calls main function to run your solution.
fn main() {
    let (corrs, fixers) = input_shit();
    for i in 0..fixers.len() {
        let mut distances: Vec<u8> = Vec::with_capacity(corrs.len());
        for j in 0..corrs.len() {
            distances.push(distance(fixers[i].as_str(), corrs[j].as_str()));
        }
        let min = distances.iter().min().unwrap();
        let corr_is: Vec<bool> = distances.iter().map(|&x| &x == min).collect();
        print!("{} ({})", fixers[i], min);
        for i in 0..corr_is.len() {
            if corr_is[i] {
                print!(" {}", corrs[i]);
            }
        }
        print!("\n");
    }
}

fn distance(str1: &str, str2: &str) -> u8{
    let l1 = str1.chars().count();
    let l2 = str2.chars().count();
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; l2 +1]; l1+1];
    // println!("1: {}, l1: {}, 2: {}, l2: {}", str1, l1, str2, l2);

    if l1 == 0 {
        return l2 as u8;
    }

    if l2 == 0 {
        return l1 as u8;
    }

    for i in 1..l1 +1 {
        matrix[i][0] = i as u8;
    }

    for i in 1..l2 +1 {
        matrix[0][i] = i as u8;
    }

    // for i in 0..matrix.len() {
    //     println!("{:?}", matrix[i]);
    // }

    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    for j in 1..l2+1 {
        for i in 1..l1+1 {
            let mut cost = 1;
            if chars1[i-1] == chars2[j-1] {
                cost = 0;
            }

            matrix[i][j] = minth(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1, matrix[i- 1][j-1] + cost);
        }
    }

    // for i in 0..matrix.len() {
    //     println!("{:?}", matrix[i]);
    // }

    matrix[l1][l2]

}

fn minth(num1: u8, num2: u8, num3: u8) -> u8 {
    let res = cmp::min(num1, num2);
    
    cmp::min(res, num3)
}

fn input_shit() -> (Vec<String>, Vec<String>) {
    // get standard input stream
    let input = io::stdin();
    // let test = BufReader::new(fs::File::open("src/sample.txt").unwrap());

    let mut corr_words: Vec<String> = Vec::new();
    let mut fix_words: Vec<String> = Vec:: new();
    let mut all_corr_read: bool = false;

    // for _line in test.lines().map(|_line| _line.unwrap()) {
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        if _line == "#" {
            all_corr_read = true;
        }else if !all_corr_read {
            corr_words.push(_line);
        }else {
            fix_words.push(_line);
        }
    }

    (corr_words, fix_words)
}