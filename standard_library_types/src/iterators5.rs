// iterators5.rs

use std::collections::HashMap;

pub fn run_iterators5() {
    let map = get_map();
    let items = get_vec_map();

    let for_count = count_for(&map, Progress::Complete);
    let count = count_iterator(&map, Progress::Complete);

    println!("(for) count: {}", for_count);
    println!("(iterator) count: {}", count);

    let for_count_items = count_collection_for(&items, Progress::Complete);
    let count_items = count_collection_iterator(&items, Progress::Complete);

    println!("(for) count items: {}", for_count_items);
    println!("(iterator) count items: {}", count_items);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.iter().filter(|&(_, val)| *val == value).count()
}

fn count_collection_for(items: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for item in items {
        for val in item.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(items: &[HashMap<String, Progress>], value: Progress) -> usize {
    items
        .iter()
        .flat_map(|item| item.values())
        .fold(0, |acc, val| if val == &value { acc + 1 } else { acc })
}

fn get_map() -> HashMap<String, Progress> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("avocado"), Complete);
    map.insert(String::from("cherry"), Complete);
    map.insert(String::from("banana"), Complete);
    map.insert(String::from("peach"), Some);
    map.insert(String::from("grape"), None);
    map.insert(String::from("pear"), None);

    map
}

fn get_vec_map() -> Vec<HashMap<String, Progress>> {
    use Progress::*;

    let map_one = get_map();

    let mut map_two = HashMap::new();
    map_two.insert(String::from("avocado"), Complete);
    map_two.insert(String::from("pear"), Complete);
    map_two.insert(String::from("banana"), Complete);
    map_two.insert(String::from("cherry"), None);
    map_two.insert(String::from("grape"), None);

    vec![map_one, map_two]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count_iterator(&map, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_complete() {
        let items = get_vec_map();
        assert_eq!(6, count_collection_iterator(&items, Progress::Complete));
    }

    #[test]
    fn count_collection_equals_for() {
        let items = get_vec_map();
        assert_eq!(
            count_collection_for(&items, Progress::Complete),
            count_collection_iterator(&items, Progress::Complete)
        );
    }
}
