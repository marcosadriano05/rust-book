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
}
