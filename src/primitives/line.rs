/// A line between two points of type `T`
pub struct Line<T> {
    p1: T,
    p2: T,
}

impl<T> Line<T> {
    pub fn new(p1: T, p2: T) -> Self {
        Self { p1, p2 }
    }
}

impl<T> PartialEq for Line<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2
    }
}
