use super::comparator::*;
use std::cmp::Ordering;
use std::fmt;
pub struct Heap<T: PartialOrd + Copy, C: Comparable<T>> {
    array: Vec<T>,
    len: usize,
    comparator: C,
}

impl<T: PartialOrd + Copy + fmt::Display, C: Comparable<T>> fmt::Display for Heap<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.array;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl<T: PartialOrd + Copy + fmt::Display, C: Comparable<T>> Heap<T, C> {
    pub fn new(comparator: C) -> Heap<T, C> {
        Heap {
            array: Vec::new(),
            len: 0,
            comparator,
        }
    }
    pub fn add(&mut self, element: T) {
        self.array.push(element);
        self.len += 1;
        self.sift_upp(self.array.len() - 1);
    }
    pub fn len(self) -> usize {
        self.len
    }
    pub fn peak(self) -> Option<T> {
        if self.len > 1 {
            Some(self.array[0])
        } else {
            None
        }
    }
    pub fn remove_root(&mut self) -> Option<T> {
        if self.len > 0 {
            let temp = *(&self.array[0]);

            self.array.swap(0, self.len - 1);
            self.array.pop();
            self.len -= 1;
            self.sift_down(0);
            Some(temp)
        } else {
            None
        }
    }
    fn sift_upp(&mut self, i: usize) {
        let mut parent_index = get_parent(i);
        let mut i = i;
        loop {
            if parent_index.is_some() {
                let pi = parent_index.unwrap() as usize;
                if self
                    .comparator
                    .compare(self.array[pi], self.array[i])
                    .is_greater()
                {
                    self.array.swap(pi, i);
                    i = pi;
                    parent_index = get_parent(i);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, i: usize) {
        let mut index = i;
        let value = self.array[index];
        while get_left_child(index) < self.array.len() {
            let left_index = get_left_child(index);
            let right_index = get_right_child(index);
            let mut child_index = left_index;
            let mut child_value = self.array[left_index];

            if right_index < self.len {
                let right_child_value = self.array[right_index];
                if self
                    .comparator
                    .compare(child_value, right_child_value)
                    .is_greater()
                {
                    child_index = right_index;
                    child_value = right_child_value;
                }
            }

            if self.comparator.compare(value, child_value).is_greater() {
                self.array[index] = child_value;
                index = child_index;
            } else {
                break;
            }
        }
    }
}
fn get_parent(i: usize) -> Option<usize> {
    if (i as f64 - 1.0 / 2.0).floor() != -1.0 {
        Some((i as f64 - 1.0 / 2.0).floor() as usize)
    } else {
        None
    }
}
fn get_right_child(i: usize) -> usize {
    2 * i + 2
}
fn get_left_child(i: usize) -> usize {
    2 * i + 1
}
