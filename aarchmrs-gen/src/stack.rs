/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use std::ops::Deref;

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
