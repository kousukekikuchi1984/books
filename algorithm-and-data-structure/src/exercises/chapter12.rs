pub fn insert_sort(mut v: Vec<u32>) -> Vec<u32> {
    println!("{:?}", v);
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
    v
}

fn merge_sort(arr: Vec<u32>) -> Vec<u32> {
    fn _merge(mut arr: Vec<u32>, left: usize, mid: usize, right: usize) -> Vec<u32> {
        let n1 = mid - left;
        let n2 = right - mid;
        let l1 = arr.clone();
        let r1 = arr.clone();
        let l = &l1[left..mid];
        let r = &r1[mid..right];

        let mut i = 0;
        let mut j = 0;
        let mut k = left;
        while i < n1 && j < n2 {
            if l[i] < r[j] {
                arr[k] = l[i];
                i += 1;
            } else {
                arr[k] = r[j];
                j += 1;
            }
            k += 1;
        }
        while i < n1 {
            arr[k] = l[i];
            i += 1;
            k += 1;
        }
        while j < n2 {
            arr[k] = r[j];
            j += 1;
            k += 1;
        }
        arr
    }
    fn _merge_sort(mut arr: Vec<u32>, left: usize, right: usize) -> Vec<u32> {
        if right - 1 > left {
            let mid = left + (right - left) / 2;
            arr = _merge_sort(arr, left, mid);
            arr = _merge_sort(arr, mid, right);
            arr = _merge(arr, left, mid, right);
        }
        arr
    }

    let size = arr.len();
    _merge_sort(arr, 0, size)
}

pub fn quick_sort(arr: Vec<u32>) -> Vec<u32> {
    fn _partition(arr: &mut Vec<u32>, left: isize, right: isize) -> isize {
        let pivot = right as usize;
        let mut store_index = left - 1;
        let mut last_index = right;

        loop {
            store_index += 1;
            while arr[store_index as usize] < arr[pivot] {
                store_index += 1;
            }
            last_index -= 1;
            while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
                last_index -= 1;
            }
            if store_index >= last_index {
                break;
            } else {
                arr.swap(store_index as usize, last_index as usize);
            }
        }
        arr.swap(store_index as usize, pivot as usize);
        store_index
    }
    fn _quick_sort(arr: &mut Vec<u32>, left: isize, right: isize) {
        if left < right {
            let p = _partition(arr, left, right);
            _quick_sort(arr, left, p - 1);
            _quick_sort(arr, p + 1, right);
        }
    }

    let size = arr.len() - 1;
    let mut a = arr.to_vec();
    _quick_sort(&mut a, 0, size as isize);
    a
}

pub fn heap_sort(a: Vec<u32>) -> Vec<u32> {
    let mut arr = a.to_vec();
    if a.len() <= 1 {
        return arr;
    }

    heapify(&mut arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0)
    }
    return arr;
}

fn heapify(arr: &mut Vec<u32>) {
    let last_parent = (arr.len() - 2) / 1;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

fn move_down(arr: &mut [u32], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };
        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

pub fn bucket_sort(a: Vec<u32>) -> Vec<u32> {
    let arr = a.to_vec();

    if arr.is_empty() {
        return vec![];
    }

    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];

    for x in &arr {
        let idx = len * *x as usize / max as usize;
        buckets[idx].push(*x as usize);
    }

    for bucket in buckets.iter_mut() {
        // this should be insertion sort
        bucket.sort();
    }

    let mut result = vec![];

    for bucket in buckets {
        for x in bucket {
            result.push(x as u32);
        }
    }

    result
}

#[cfg(test)]
mod test {

    use super::{bucket_sort, heap_sort, insert_sort, merge_sort, quick_sort};

    #[test]
    fn test_insert_sort() {
        assert_eq!(insert_sort(vec![1, 5, 2, 4, 3]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(
            merge_sort(vec![64, 34, 25, 8, 22, 11, 9]),
            vec![8, 9, 11, 22, 25, 34, 64]
        );
    }

    #[test]
    fn test_quick_sort() {
        assert_eq!(
            quick_sort(vec![64, 34, 25, 8, 22, 11, 9]),
            vec![8, 9, 11, 22, 25, 34, 64]
        );
    }

    #[test]
    fn test_heap_sort() {
        assert_eq!(
            heap_sort(vec![64, 34, 25, 8, 22, 11, 9]),
            vec![8, 9, 11, 22, 25, 34, 64]
        );
    }

    #[test]
    fn test_bucket_sort() {
        assert_eq!(
            bucket_sort(vec![64, 34, 25, 8, 22, 11, 9]),
            vec![8, 9, 11, 22, 25, 34, 64]
        );
    }
}
