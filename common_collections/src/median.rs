use super::sorting::bubble_sort;

pub fn median(list: &Vec<i32>) -> Option<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_test() {
        let list: Vec<i32> = vec![5, 7, 2, 4, 1, 9];
        assert_eq!(median(&list), Some(4));
    }
}
