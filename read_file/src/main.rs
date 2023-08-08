use std::fs;
use std::env;
use std::collections::HashMap;

fn correct_usage() {
    println!("Incorrect usage of program. Please use one of the following options:");
    println!("1 - plato read <file>     --Reads entire file" );
    println!("2 - plato stats <file>    --Get's stats for a text file");
    println!("3 - plato delete <file>   --Delete's the file");
}

fn read_file(file_dir: String) {
    let text = match env::current_dir() {
        Ok(curr_dir) => fs::read_to_string(curr_dir.join(file_dir)).expect("Couldn't find file"),
        Err(_) => panic!("Couldn't read file"),
    };
    println!("{}", text);
}

fn stats_file(file_dir: String) {

    fn handle_word(word: &str, stats: &mut HashMap<String,u32>) {
        match stats.get_mut(word) {
            Some(counter) => {
                *counter += 1;
            },
            None => {let _ = stats.insert(String::from(word), 1 as u32);}
        };
    }

    let text = match env::current_dir() {
        Ok(curr_dir) => fs::read_to_string(curr_dir.join(file_dir)).expect("Couldn't find file"),
        Err(_) => panic!("Couldn't read file"),
    };

    let mut stats: HashMap<String, u32> = HashMap::new();

    let words = text.split(' ');

    for word in words {
        handle_word(word, &mut stats)
    }

    let mut sorted_keys: Vec<String> = stats.clone().into_keys().collect();

    sorted_keys.sort_by(|a, b| stats[b].cmp(&stats[a]));

    for i in sorted_keys {
        println!("{} - {}", i, stats.get(&i).expect("Couldn't read key"))
    }
}

fn delete_file(file_dir: String) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        correct_usage();
        return;
    }

    match args[1].to_ascii_lowercase().as_str() {
        "read" => read_file(args[2].clone()),
        "stats" => stats_file(args[2].clone()),
        "delete" => delete_file(args[2].clone()),
        _ => correct_usage(),
    }

    dbg!(args);
}
