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
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = Vec::new();

        while let Some(Ok(line)) = self.lines_iter.next() {
            if line.trim().is_empty() {
                // blank line encountered, return the output
                if !output.is_empty() {
                    return Some(output);
                }
            } else {
                // the next line is not empty, therefore it will have an integer to grab
                if let Ok(number) = line.trim().parse() {
                    output.push(number);
                } else {
                    eprintln!("Error parsing integer");
                }
            }
        }

        if !output.is_empty() {
            Some(output)
        } else {
            None
        }
    }
}

fn manage_top_three(set: &mut Vec<i32>, entry: i32) -> &mut Vec<i32> {
    let mut memory_a: i32 = 0;
    let mut replaced: bool = false;

    for i in 0..3 {
        if replaced {
            if memory_a > set[i] {
                let memory_b = set[i];
                set[i] = memory_a;
                memory_a = memory_b;
            }
        } else {
            if entry > set[i] {
                memory_a = set[i];
                set[i] = entry;
                replaced = true;
            }
        }
    }

    return set;
}

fn main() -> io::Result<()> {
    let file_path = "inputs/day_1.txt";
    let int_sets_iterator = GetInputSet::new(file_path)?;

    let mut part_1_output: i32 = 0;
    let mut part_2_output: Vec<i32> = vec![0, 0, 0];

    for int_set in int_sets_iterator {
        println!("Set: {:?}", int_set);
        let vec_sum: i32 = int_set.iter().sum();
        if vec_sum > part_1_output {
            part_1_output = vec_sum;
        }
        let _part_2_output = manage_top_three(&mut part_2_output, vec_sum);
    }

    println!("Part 1 output: {:?}", part_1_output);
    let sum_part_2: i32 = part_2_output.iter().sum();
    println!("Part 2 output: {:?}", sum_part_2);
    Ok(())
}