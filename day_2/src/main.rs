use std::fs::File;
use std::io::{self, BufRead};

struct GetInputSet {
    lines_iter: io::Lines<io::BufReader<File>>,

}

impl GetInputSet {
    fn new(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        Ok(Self {
            lines_iter: reader.lines(),
        })
    }
}

impl Iterator for GetInputSet {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if there's another line in the file (Result<Option<String>>).
        if let Some(Ok(line)) = self.lines_iter.next() {
            // Process the line:
            // - Convert it into an iterator of characters.
            // - Filter out spaces.
            // - Collect the characters into a Vec<char>.
            // .filter takes out any spaces
            // .collect returns all the remaining chars in the line
            let output: Vec<char> = line.chars().filter(|&c| c != ' ').collect();

            // Return the processed line as a Vec<char>.
            Some(output)
        } else {
            // If there are no more lines in the file, return None to signal the end of iteration.
            None
        }
    }
}

fn calculate_play(input: char, choice: char) -> char {

    match (input, choice) {
        ('A','X') => return 'Z', // A for Rock, X to lose, so we play Scissors ('Z')
        ('A','Y') => return 'X', // A for Rock, Y to draw, so we also play Rock ('X')
        ('A','Z') => return 'Y', // A for Rock, Z to win, so we play Paper ('Y')
        ('B','X') => return 'X', // B for Paper, X to lose, so we play Rock ('X')
        ('B','Y') => return 'Y', // B for Paper, Y to draw, so we also play Paper ('Y')
        ('B','Z') => return 'Z', // B for Paper, Z to win, so we play Scissors ('Z')
        ('C','X') => return 'Y', // C for Scissors, X to lose, so we play Paper ('Y')
        ('C','Y') => return 'Z', // C for Scissors, Y to draw, so we also play Scissors ('Z')
        ('C','Z') => return 'X', // C for Scissors, Z to win, so we play Rock ('X')
        (_, _) => panic!("Invalid RPS decision pairing!")
    }
}

fn calculate_score(input: char, choice: char) -> i32 {

    match (input, choice) {
        ('A','X') => return 4, // 1 for rock, 3 for draw
        ('A','Y') => return 8, // 2 for paper, 6 for win
        ('A','Z') => return 3, // 3 for scissors, 0 for loss
        ('B','X') => return 1, // 1 for rock, 0 for loss
        ('B','Y') => return 5, // 2 for paper, 3 for draw
        ('B','Z') => return 9, // 3 for scissors, 6 for win
        ('C','X') => return 7, // 1 for rock, 6 for win
        ('C','Y') => return 2, // 2 for paper, 0 for loss
        ('C','Z') => return 6, // 3 for scissors, 3 for draw
        (_, _) => panic!("Invalid RPS pairing!")
    }
}

fn main() {
    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;
    let input: Result<GetInputSet, io::Error> = GetInputSet::new("inputs/day_2.txt");
    for v in input.expect("Uh oh, you didn't get what you expected") {
        if v.len() >= 2 {
            sum_1 += calculate_score(v[0], v[1]);
            sum_2 += calculate_score(v[0], calculate_play(v[0], v[1]));
        } else {
            panic!("Invalid input: Vec<char> too short!");
        }
        
    }
    println!("Result for part 1 is {:?}", sum_1);
    println!("Result for part 2 is {:?}", sum_2);
}