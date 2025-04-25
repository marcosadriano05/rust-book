use std::collections::HashMap;

fn mode(list: &Vec<i32>) -> Option<i32> {
    if list.is_empty() {
        return None;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_test() {
        let list: Vec<i32> = vec![5, 1, 7, 2, 4, 1, 9];
        assert_eq!(mode(&list), Some(1));
    }
}
