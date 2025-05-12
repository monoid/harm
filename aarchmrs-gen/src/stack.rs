use std::ops::Deref;

#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
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
    data: &'a mut Vec<T>,
}

impl<'a, T> StackGuard<'a, T> {
    pub fn push<'b>(&'b mut self, val: T) -> StackGuard<'b, T>
    where
        'a: 'b,
    {
        self.data.push(val);
        StackGuard { data: self.data }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::<i32>::new();
        {
            let mut g1 = stack.push(42);
            let g2 = g1.push(43);
            assert_eq!(*g2, [42, 43]);
        }
        assert!(stack.data.is_empty());
    }
}
