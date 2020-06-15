use std::cmp::Ordering;

pub struct MaxComparator();
pub struct MinComparator();
pub struct OrderingWrapper(Ordering);
pub trait Comparable<T: PartialOrd> {
    fn compare(&self, a: T, b: T) -> OrderingWrapper;
}

impl MinComparator {
    pub fn new() -> MinComparator {
        MinComparator()
    }
}
impl MaxComparator {
    pub fn new() -> MaxComparator {
        MaxComparator()
    }
}

impl OrderingWrapper {
    pub fn is_equal(self) -> bool {
        match self {
            OrderingWrapper(Ordering::Less) => true,
            _ => false,
        }
    }
    pub fn is_greater(self) -> bool {
        match self {
            OrderingWrapper(Ordering::Greater) => true,
            _ => false,
        }
    }
    pub fn is_less(self) -> bool {
        match self {
            OrderingWrapper(Ordering::Less) => true,
            _ => false,
        }
    }
    pub fn unwrap(self) -> Ordering {
        match self {
            OrderingWrapper(a) => a,
        }
    }
}

impl<T: PartialOrd> Comparable<T> for MinComparator {
    fn compare(&self, a: T, b: T) -> OrderingWrapper {
        if a > b {
            return OrderingWrapper(Ordering::Greater);
        } else if a < b {
            return OrderingWrapper(Ordering::Less);
        }
        OrderingWrapper(Ordering::Equal)
    }
}

impl<T: PartialOrd> Comparable<T> for MaxComparator {
    fn compare(&self, a: T, b: T) -> OrderingWrapper {
        if a < b {
            return OrderingWrapper(Ordering::Greater);
        } else if a > b {
            return OrderingWrapper(Ordering::Less);
        }
        OrderingWrapper(Ordering::Equal)
    }
}
