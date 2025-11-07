/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::io;
use std::io::Write;

fn do_sort<T>(array: &mut[T], start: usize, end: usize) where T:PartialOrd + Clone {
    if start >= end {
        return;
    }
    let mut pivot = array[(start + end - 1) / 2].clone();
    let mut i = start;
    let mut j = end - 1;

    while i < j {
        while array[i] < pivot {
            i += 1;
        }
        while pivot < array[j] {
            j -= 1;
        }
        array.swap(i, j);
    }


    do_sort(array, start, i);
    do_sort(array, i + 1, end);
}

fn main() {
    let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
    sort(&mut vec);
    assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
}

fn sort<T>(array: &mut [T]) where T: PartialOrd + Clone {
    do_sort(array, 0, array.len());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
