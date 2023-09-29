use std::ops::{Index, RangeFrom};
use crate::e2019::vec2::Vec2;

/// vec3
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vec3<T>(Vec<Vec2<T>>);

impl<T> From<Vec<Vec2<T>>> for Vec3<T> {
    fn from(v: Vec<Vec2<T>>) -> Self {
        Vec3(v)
    }
}

impl <T> Vec3<T> {
    pub fn iter(&self) -> impl Iterator<Item=&Vec2<T>> {
        self.0.iter()
    }
}

// indexing
impl <T> Index<usize> for Vec3<T> {
    type Output = Vec2<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <T> Index<RangeFrom<usize>> for Vec3<T> {
    type Output = [Vec2<T>];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index.start..]
    }
}

impl<T> From<Vec<Vec<Vec<T>>>> for Vec3<T> {
    fn from(v: Vec<Vec<Vec<T>>>) -> Self {
        Vec3(v.into_iter().map(Vec2::from).collect())
    }
}

impl <I> FromIterator<Vec2<I>> for Vec3<I> {
    fn from_iter<T: IntoIterator<Item=Vec2<I>>>(iter: T) -> Self {
        Vec3(iter.into_iter().collect())
    }
}
