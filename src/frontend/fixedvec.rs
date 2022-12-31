use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct FixedVec<T, const S: usize> {
    array: [T; S],
    len: usize,
}

impl<T: Copy + Default, const S: usize> FixedVec<T, S> {
    pub fn new() -> Self {
        FixedVec {
            array: [T::default(); S],
            len: 0,
        }
    }
}
impl<T: Default, const S: usize> FixedVec<T, S> {
    pub fn push(&mut self, val: T) {
        self.array[self.len] = val;
        self.len += 1;
    }
    pub fn pop(&mut self) -> T {
        self.len -= 1;
        std::mem::take(&mut self.array[self.len])
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len != 0
    }
}
impl<T, const S: usize> Index<usize> for FixedVec<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}
impl<T, const S: usize> IndexMut<usize> for FixedVec<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}
impl<T: Default + Copy, const S: usize> Default for FixedVec<T, S> {
    fn default() -> Self {
        Self::new()
    }
}
