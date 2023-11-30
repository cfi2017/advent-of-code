use std::ops::{Index, IndexMut};

impl<T> From<Vec<Vec<T>>> for Vec2<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        Vec2(v)
    }
}

/// vec2
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vec2<T>(Vec<Vec<T>>);

impl <I> FromIterator<Vec<I>> for Vec2<I> {
    fn from_iter<T: IntoIterator<Item=Vec<I>>>(iter: T) -> Self {
        Vec2(iter.into_iter().collect())
    }
}

impl <T> Index<usize> for Vec2<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <T> Vec2<T> {

    pub fn push(&mut self, i: Vec<T>) {
        self.0.push(i);
    }
    pub fn iter(&self) -> impl Iterator<Item=&Vec<T>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Vec<T>> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
