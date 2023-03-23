use std::collections::HashMap;

fn main() {
    use borders::{
        styles::{ASCII, DOUBLE, THIN},
        BorderFormatter,
    };

    println!(
        "{}",
        THIN.format_iter([0, 5, 12, 3, 234, 124, 4234, 234, 234, 234].iter())
    );
    println!("{}", THIN.format_slice(&["hello", "world"]));
    println!("{}", THIN.format_slice(&["hello\nworld", "goodbye\nworld"]));

    let mut map: HashMap<_, usize, _> = HashMap::default();
    "hello world, how are you doing today?"
        .chars()
        .for_each(|c| *map.entry(c).or_default() += 1);
    println!("{}", THIN.format_hash_map(&map));

    println!("{}", DOUBLE.format_display("Hello World!"));
    println!("{}", DOUBLE.format_debug("Hello World!"));
    println!("{}", DOUBLE.format_debug(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

    let mut map = HashMap::default();
    map.insert("   ", "   ");
    let b = THIN.format_hash_map_headers(&map, "   ", "   ");
    println!("{}", b);
    let b = DOUBLE.format_display(b);
    let b = ASCII.format_display(b);
    println!("{}", b);

    let slice = [0, 1, 2, 3, 4];
    println!("{}", THIN.format_slice(&slice));

    let mut map = HashMap::default();
    map.insert("Jon", 38);
    map.insert("Jake", 25);
    map.insert("Josh", 17);
    println!("{}", THIN.format_hash_map(&map));

    println!("{}", THIN.format_display("hello"));
    println!("{}", THIN.format_debug("hello"));
}
