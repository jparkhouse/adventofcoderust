use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn char_to_priority(item_type: char) -> Result<i32, String> {
    match item_type {
        'a'..='z' => Ok(item_type as i32 - 'a' as i32 + 1),
        'A'..='Z' => Ok(item_type as i32 - 'A' as i32 + 27),
        _ => {
            // Handle other characters (not lowercase or uppercase letters) here if needed.
            // For example, you can return a default priority or an error value.
            Err("incompatible char".to_string()) // Default value for other characters
        }
    }
}

fn find_duplicate_char(vec_1: Vec<char>, vec_2: Vec<char>) -> Option<char> {
    let mut found:HashMap<char, i8> = HashMap::new();
    // hash all items from vec_1, keeping a count of how many there are just in case.
    for i in vec_1 {
        let c = found.entry(i).or_insert(0);
        *c += 1;
    }
    // check each item in vec_2 and see if there is any crossover - if there is return the char
    for i in vec_2 {
        if found.contains_key(&i) {
            return Some(i);
        }
    }
    return None;
}
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
            Some(line.trim().chars().collect())
        } else {
            // If there are no more lines in the file, return None to signal the end of iteration.
            return None
        }
    }
}

fn split_to_pockets(mut bag_contents: Vec<char>) -> Option<(Vec<char>,Vec<char>)> {
    // set up holding vecs
    let mut pocket_1: Vec<char> = Vec::new();
    let mut pocket_2: Vec<char> = Vec::new();
    // split bag_contents in half to find the contents of each pocket
    while bag_contents.len() > pocket_1.len() {
        let temp = bag_contents.pop().unwrap();
        pocket_1.push(temp);
    }
    while bag_contents.len() > 0 {
        let temp = bag_contents.pop().unwrap();
        pocket_2.push(temp);
    }
    Some((pocket_1, pocket_2))
}

fn find_badge(elf_1_items: Vec<char>, elf_2_items: Vec<char>, elf_3_items: Vec<char>) -> Result<char, String> {
    // the map of potential badges
    let mut badge_map:HashMap<char, i8> = HashMap::new();
    // the map of items already found in the bag, to skip any duplicates
    // reset for each elf
    let mut bag_map:HashMap<char, i8> = HashMap::new();
    for item in elf_1_items {
        // if the item has not already been found in the bag, add it to the map of potential badges
        // also add it to the bag map to skip duplicates
        if !bag_map.contains_key(&item) {
            bag_map.insert(item, 1);
            badge_map.insert(item, 1);
        }
    }
    // reset bag items since new bag
    let mut bag_map:HashMap<char, i8> = HashMap::new();
    for item in elf_2_items {
        // if we havent seen it in this bag before, and it was in a previous bag, then update the badge map to say we have found 2 of these
        if !bag_map.contains_key(&item) && badge_map.contains_key(&item) {
            bag_map.insert(item, 1);
            badge_map.insert(item, 2);
        }
    }
    for item in elf_3_items {
        // if it was in both previous bags, then it must be our badge
        if badge_map.contains_key(&item) {
            if badge_map.get_key_value(&item).unwrap().1 == &(2 as i8) {
                return Ok(item);
            }
        }
    }
    return Err("no item found across all three bags".to_string())
}

fn main() {
    let input: Result<GetInputSet, io::Error> = GetInputSet::new("inputs/day_3.txt");
    // for part 1
    let mut sum_1: i32 = 0;
    // for part 2
    let mut sum_2: i32 = 0;
    let mut group: Vec<Vec<char>> = Vec::new();
    for items in input.expect("Uh oh") {
        // logic for part 1
        let (pocket_1, pocket_2) = split_to_pockets(items.clone()).unwrap();
        let duplicate_item = find_duplicate_char(pocket_1, pocket_2).unwrap();
        sum_1 += char_to_priority(duplicate_item).unwrap();
        // logic for part 2
        // add elf's items to the group
        group.push(items.clone());
        // if we have collected a full group of 3 elfs
        if group.len() == 3 {
            let elf_1_items = group.pop().unwrap();
            let elf_2_items = group.pop().unwrap();
            let elf_3_items = group.pop().unwrap();
            let badge = find_badge(elf_1_items, elf_2_items, elf_3_items).unwrap();
            sum_2 += char_to_priority(badge).unwrap();
        }
    }
    println!("The answer to part 1 is {:?}", sum_1);
    println!("The answer to part 2 is {:?}", sum_2);
}