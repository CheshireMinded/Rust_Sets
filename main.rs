use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum InputSource {
    Keyboard,
    File(String),
    Exit,
}

enum KeyType {
    Int,
    String,
    Char,
    Exit,
}

/*  Example: {1,2,3} and {4,5,6}
Union: 
The combination of all unique elements from both sets 
output should be: {1,2,3,4,5,6}



Intersection: 
The common elements between the two sets output should be {} 
(an empty set)


Difference (A - B): 
Elements present in the first set but NOT in the SECOND 
{1, 2, 3}

Difference (B-A):
Elements present in the second set but NOT in the FIRST
{4,5,6}

Symmetric Difference: 
Elements that are in either of the two sets but NOT in both. 
{1, 2, 3, 4, 5, 6}
*/ 



fn main() {
    let input_source = get_input_source();

    if let InputSource::Exit = input_source {
        return;
    }

    let key_type = get_key_type();

    let (set_a_input, set_b_input): (HashSet<String>, HashSet<String>) = match input_source {
        InputSource::Keyboard => read_input_from_keyboard().expect("Failed to read input from keyboard"),
        InputSource::File(path) => read_input_from_file(&path).expect("Failed to read from file"),
        _ => (HashSet::new(), HashSet::new()),
    };

    match key_type {
        KeyType::Int => {
            let int_set_a: HashSet<i32> = set_a_input.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
            let int_set_b: HashSet<i32> = set_b_input.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
            perform_int_operations(&int_set_a, &int_set_b);
        },
        KeyType::String => {
            perform_str_operations(&set_a_input, &set_b_input);
        },
        KeyType::Char => {
            //ignores more than one char
            
            let char_set_a: HashSet<char> = set_a_input.iter()
                .filter_map(|s| s.chars().next())
                .collect();
            let char_set_b: HashSet<char> = set_b_input.iter()
                .filter_map(|s| s.chars().next())
                .collect();
        
            perform_char_operations(&char_set_a, &char_set_b);
        },
        
        KeyType::Exit => return,
    }
}


/* MENU OPTION:
this prompts the user for an entry (per lab requirements) via peripheal keyboard or file upload
*/
fn get_input_source() -> InputSource {
    loop {
        println!("Select input source:");
        println!("0: Exit");
        println!("1: Keyboard");
        println!("2: File");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "0" => return InputSource::Exit,
                    "1" => return InputSource::Keyboard,
                    "2" => {
                        println!("Enter file path:");
                        let mut path = String::new();
                        if io::stdin().read_line(&mut path).is_ok() {
                            return InputSource::File(path.trim().to_string());
                        } else {
                            println!("Error reading file path, please try again.");
                        }
                    },
                    _ => println!("Invalid selection, please try again."),
                }
            },
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}



/* Ask user for the "key type" AKA what do they want to enter and then output
This will show up on menu
*/
fn get_key_type() -> KeyType {
    loop {
        println!("Select key type:");
        println!("0: Exit");
        println!("1: Integer");
        println!("2: String");
        println!("3: Char");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "0" => return KeyType::Exit,
                    "1" => return KeyType::Int,
                    "2" => return KeyType::String,
                    "3" => return KeyType::Char,
                    _ => println!("Invalid selection, please try again."),
                }
            },
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}


fn read_input_from_keyboard() -> io::Result<(HashSet<String>, HashSet<String>)> {
    let mut input = String::new();
    println!("Enter elements for the first set, separated by spaces:");
    io::stdin().read_line(&mut input)?;
    let set_a: HashSet<String> = input.trim().split_whitespace().map(String::from).collect();

    input.clear();
    println!("Enter elements for the second set, separated by spaces:");
    io::stdin().read_line(&mut input)?;
    let set_b: HashSet<String> = input.trim().split_whitespace().map(String::from).collect();

    Ok((set_a, set_b))
}

// reads input from a file that user selects
fn read_input_from_file(path: &str) -> io::Result<(HashSet<String>, HashSet<String>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let set_a: HashSet<String> = lines.get(0).unwrap_or(&String::new()).split_whitespace().map(String::from).collect();
    let set_b: HashSet<String> = lines.get(1).unwrap_or(&String::new()).split_whitespace().map(String::from).collect();

    Ok((set_a, set_b))
}




// performs the integer operations
fn perform_int_operations(set_a: &HashSet<i32>, set_b: &HashSet<i32>) {
    //Union
    let union: HashSet<_> = set_a.union(set_b).cloned().collect();
    let mut sorted_union: Vec<_> = union.into_iter().collect();
    sorted_union.sort_unstable();

    //Intersection
    let intersection: HashSet<_> = set_a.intersection(set_b).cloned().collect();
    let mut sorted_intersection: Vec<_> = intersection.into_iter().collect();
    sorted_intersection.sort_unstable();

    //Difference A-B
    let difference_ab: HashSet<_> = set_a.difference(set_b).cloned().collect();
    let mut sorted_difference_ab: Vec<_> = difference_ab.into_iter().collect();
    sorted_difference_ab.sort_unstable();

    //Difference B-A
    let difference_ba: HashSet<_> = set_b.difference(set_a).cloned().collect();
    let mut sorted_difference_ba: Vec<_> = difference_ba.into_iter().collect();
    sorted_difference_ba.sort_unstable();

    // Symm Diff
    let symmetric_difference: HashSet<_> = set_a.symmetric_difference(set_b).cloned().collect();
    let mut sorted_symmetric_difference: Vec<_> = symmetric_difference.into_iter().collect();
    sorted_symmetric_difference.sort_unstable();

    // Now, print using the sorted variables
    println!("Union (A ∪ B): {:?}", sorted_union);
    println!("Intersection (A ∩ B): {:?}", sorted_intersection);
    println!("Difference (A − B): {:?}", sorted_difference_ab);
    println!("Difference (B − A): {:?}", sorted_difference_ba);
    println!("Symmetric Difference (A Δ B): {:?}", sorted_symmetric_difference);
}




// performs the string operations
fn perform_str_operations(set_a: &HashSet<String>, set_b: &HashSet<String>) {
    // Convert all strings to lowercase for case-insensitive comparison and remove trailing commas
    let normalize_set = |set: &HashSet<String>| -> HashSet<String> {
        set.iter()
            .map(|s| s.trim_end_matches(',').to_lowercase())
            .collect()
    };

    let normalized_set_a = normalize_set(set_a);
    let normalized_set_b = normalize_set(set_b);

    // Perform operations on normalized sets
    let union: HashSet<_> = normalized_set_a.union(&normalized_set_b).cloned().collect();
    let intersection: HashSet<_> = normalized_set_a.intersection(&normalized_set_b).cloned().collect();
    let difference_ab: HashSet<_> = normalized_set_a.difference(&normalized_set_b).cloned().collect();
    let difference_ba: HashSet<_> = normalized_set_b.difference(&normalized_set_a).cloned().collect();
    let symmetric_difference: HashSet<_> = normalized_set_a.symmetric_difference(&normalized_set_b).cloned().collect();

    // Sort results for display
    let sorted_union: Vec<_> = union.into_iter().collect();
    let sorted_intersection: Vec<_> = intersection.into_iter().collect();
    let sorted_difference_ab: Vec<_> = difference_ab.into_iter().collect();
    let sorted_difference_ba: Vec<_> = difference_ba.into_iter().collect();
    let sorted_symmetric_difference: Vec<_> = symmetric_difference.into_iter().collect();

    // Print sorted results
    println!("Union (A ∪ B): {:?}", sorted_union);
    println!("Intersection (A ∩ B): {:?}", sorted_intersection);
    println!("Difference (A − B): {:?}", sorted_difference_ab);
    println!("Difference (B − A): {:?}", sorted_difference_ba);
    println!("Symmetric Difference (A Δ B): {:?}", sorted_symmetric_difference);
}



fn perform_char_operations(char_set_a: &HashSet<char>, char_set_b: &HashSet<char>) {
    // Union: All unique elements from both sets
    let mut union: Vec<_> = char_set_a.union(char_set_b).cloned().collect();
    union.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    
    // Intersection: Elements common to both sets
    let mut intersection: Vec<_> = char_set_a.intersection(char_set_b).cloned().collect();
    intersection.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    
    // Difference (A - B): Elements in A but not in B
    let mut difference_ab: Vec<_> = char_set_a.difference(char_set_b).cloned().collect();
    difference_ab.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    
    // Difference (B - A): Elements in B but not in A
    let mut difference_ba: Vec<_> = char_set_b.difference(char_set_a).cloned().collect();
    difference_ba.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    
    // Symmetric Difference: Elements in either A or B but not in both
    let mut symmetric_difference: Vec<_> = char_set_a.symmetric_difference(char_set_b).cloned().collect();
    symmetric_difference.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    
    // Use the sorted and directly defined variables for printing
    println!("Union (A ∪ B): {:?}", union);
    println!("Intersection (A ∩ B): {:?}", intersection);
    println!("Difference (A − B): {:?}", difference_ab);
    println!("Difference (B − A): {:?}", difference_ba);
    println!("Symmetric Difference (A Δ B): {:?}", symmetric_difference);
}




