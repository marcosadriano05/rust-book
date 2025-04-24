mod sorting {
    fn buble_sort_rec(list: &mut Vec<i32>, len: usize) {
        if len <= 1 {
            return ();
        }

        for i in 0..len - 1 {
            let cur: i32 = list.get(i).map_or(0, |item| *item);
            let next: i32 = list.get(i + 1).map_or(0, |item| *item);
            if cur > next {
                if let Some(item) = list.get_mut(i) {
                    *item = next;
                }
                if let Some(item) = list.get_mut(i + 1) {
                    *item = cur;
                }
            }
        }

        buble_sort_rec(list, len - 1);
    }

    pub fn bubble_sort(list: &mut Vec<i32>) {
        buble_sort_rec(list, list.len());
    }
}

use std::collections::HashMap;

use sorting::bubble_sort;

fn median(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }
    if list.len() == 1 {
        return list.get(0).map(|item| *item);
    }
    let mut sorted_list = list.clone();
    bubble_sort(&mut sorted_list);
    let len = sorted_list.len();
    if len % 2 != 0 {
        if let Some(item) = sorted_list.get(len / 2) {
            return Some(*item);
        }
    }
    if let Some(item) = sorted_list.get(len / 2 - 1) {
        return Some(*item);
    }
    None
}

fn mode(list: &Vec<i32>) -> Option<i32> {
    let mut hash = HashMap::new();

    for item in list {
        let count = hash.entry(*item).or_insert(0);
        *count += 1;
    }

    let mut response: (i32, i32) = (0, 0);
    for (key, value) in hash.iter() {
        if response.1 < *value {
            response = (*key, *value);
        }
    }

    Some(response.0)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut list: Vec<i32> = vec![5, 7, 2, 4, 1, 9];
        bubble_sort(&mut list);
        assert_eq!(list, vec![1, 2, 4, 5, 7, 9]);
    }

    #[test]
    fn bubble_sort_test_2() {
        let mut list: Vec<i32> = vec![5];
        bubble_sort(&mut list);
        assert_eq!(list, vec![5]);
    }

    #[test]
    fn bubble_sort_test_3() {
        let mut list: Vec<i32> = vec![];
        bubble_sort(&mut list);
        assert_eq!(list, vec![]);
    }

    #[test]
    fn median_test() {
        let list: Vec<i32> = vec![5, 7, 2, 4, 1, 9];
        assert_eq!(median(&list), Some(4));
    }

    #[test]
    fn mode_test() {
        let list: Vec<i32> = vec![5, 1, 7, 2, 4, 1, 9];
        assert_eq!(mode(&list), Some(1));
    }
}
