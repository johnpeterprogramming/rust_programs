use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};

use std::collections::HashSet;

const SIZE:usize = 4;


fn main () {
    let boggle : [[char; SIZE]; SIZE ] =
    [['m', 'l', 'i', 'a'],
    ['n', 'u', 'i', 't'],
    ['l', 'e', 'n', 'p'],
    ['u', 's', 'e', 'e']];
    
    find_words(boggle);
}

fn recurse_board (boggle:[[char; SIZE]; SIZE ], visited : &mut [[bool; SIZE];SIZE], i:usize, j:usize, word:&mut String, hashset: &HashSet<String>) {
    visited[i][j] = true;
    word.push(boggle[i][j]);
    // println!("{}", word);

    if hashset.contains(word) {
        println!("{}", word);
    }

    let mut row = (i as i32) - 1;
    let larger_row = (i as i32) + 1;
    while row <= larger_row && row < SIZE as i32 {
        let mut col = (j as i32) - 1;
        let larger_col = (j as i32) + 1;
        while col <= larger_col && col < SIZE as i32 {
            if row >= 0 && col >= 0 {
                // println!("{}", row);
                // println!("{}", col);
                let row = row as usize;
                let col = col as usize;
                if !visited[row][col] {
                    recurse_board(boggle, visited, row, col, word, hashset);
                }
            }
        col += 1;
        }
    row += 1;
    }
    word.pop();

    visited[i][j] = false;
        
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn find_words (boggle:[[char; SIZE]; SIZE ]) {
    let mut visited: [[bool; SIZE];SIZE] = [[false;SIZE]; SIZE];
    let mut word = String::new();
    
    println!("Starting to initialize!");
    let mut hashset = HashSet::new();
    for line in lines_from_file("words.txt").expect("Couln't load text file into memory") {
        hashset.insert(line);
    }
    println!("Done initializing!");

    for i in 0..SIZE {
        for j in 0..SIZE {
            recurse_board(boggle, &mut visited, i, j, &mut word, &hashset);
        }
    }
    
}
