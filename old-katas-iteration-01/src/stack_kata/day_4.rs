use alloc::raw_vec::RawVec;

use std::ops::{Deref, DerefMut};
use std::{slice, ptr};
use std::intrinsics::assume;

pub struct Stack<T> {
    size: usize,
    buf: RawVec<T>
}

impl <T> Deref for Stack<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.pointer(), self.size)
        }
    }
}

impl <T> DerefMut for Stack<T> {
    
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer(), self.size)
        }
    }
}

impl <T> Stack<T> {
    
    pub fn new(max_size: usize) -> Stack<T> {
        Stack { size: 0, buf: RawVec::with_capacity(max_size) }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, v: T) {
        if self.size != self.buf.cap() {
            unsafe {
                let end = self.as_mut_ptr().offset(self.size as isize);
                ptr::write(end, v);
            }
            self.size += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        }
        else {
            self.size -= 1;
            unsafe {
                Some(ptr::read(self.get_unchecked(self.size)))
            }
        }
    }

    unsafe fn pointer(&self) -> *mut T {
        let ptr = self.buf.ptr();
        assume(!ptr.is_null());
        ptr   
    }
}