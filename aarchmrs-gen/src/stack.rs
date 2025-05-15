use std::ops::Deref;

#[derive(Debug)]
pub struct Stack<T> {
    pub data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            data: <_>::default(),
        }
    }

    pub fn push(&mut self, val: T) -> StackGuard<'_, T> {
        self.data.push(val);
        StackGuard {
            data: &mut self.data,
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct StackGuard<'a, T> {
    pub data: &'a mut Vec<T>,
}

impl<'a, T> StackGuard<'a, T> {
    pub fn push(data: &'a mut Vec<T>, val: T) -> Self {
        data.push(val);
        StackGuard { data }
    }
}

impl<T> Deref for StackGuard<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> Drop for StackGuard<'_, T> {
    fn drop(&mut self) {
        self.data.pop();
    }
}
