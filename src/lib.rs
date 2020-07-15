///
/// This implementation of a Static Array serves the purpose for me of implementing
/// an array in rust using unsafe features so I can get a grasp of how more low level
/// memory management in Rust can be done
///
/// This is in no way is a showcase of production ready code but it's sort of a learning
/// experience writing this that hopes to be an example on how simple allocations can be done
/// in this language without using std::box
///
struct StaticArray<T> {
    ptr: *mut T,
    size: usize,
    current: usize,
}

impl<T> Drop for StaticArray<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, Layout::array::<T>(self.size).unwrap());
        }
    }
}

use std::alloc::{alloc, dealloc, Layout};
impl<T> StaticArray<T> {
    ///
    /// Returns the current length of the array
    ///
    pub fn current_len(&self) -> usize {
        self.current
    }

    ///
    /// Returns the allocated size in memory
    ///
    pub fn size(&self) -> usize {
        self.size
    }

    ///
    /// New returns a new static array of the desired size
    /// allocates the size in memory
    ///
    ///
    pub fn new(size: usize) -> StaticArray<T> {
        let mut ptr: *mut T = 0 as *mut T;
        unsafe {
            let layout = Layout::array::<T>(size).unwrap();
            ptr = alloc(layout) as *mut T;
        }
        StaticArray {
            ptr,
            size,
            current: 0,
        }
    }

    ///
    /// Pushes a new element to the static array and returns Ok if it is successful
    ///
    /// If it is not, it will return an error
    ///     ** The error can be that the array is full
    ///
    pub fn push(&mut self, element: T) -> Result<(), &str> {
        if self.current == self.size {
            return Err("full array");
        }
        unsafe {
            *(self.ptr.offset(self.current as isize)) = element;
        }
        self.current += 1;
        Ok(())
    }

    ///
    /// Gets an element on the desired index
    ///
    /// Can panic if it overflows so probably the check self.current_len() > idx is desired
    ///
    pub fn get(&self, idx: isize) -> &T {
        if idx >= self.current as isize {
            panic!("out of bounds in idx: {} at size: {}", idx, self.current);
        }
        return unsafe { &*self.ptr.offset(idx) };
    }

    ///
    /// The same as get() but with mutable reference
    ///
    pub fn get_mut(&mut self, idx: isize) -> &mut T {
        return unsafe { &mut *self.ptr.offset(idx) };
    }
}

mod test {

    ///
    ///
    /// Example of manual memory allocation in rust
    ///
    ///
    #[test]
    fn mem_alloc() {
        struct Point {
            x: u8,
            y: u8,
        }
        use super::StaticArray;
        let mut arr = StaticArray::<Point>::new(100);

        while let Ok(()) = arr.push(Point { x: 1, y: 2 }) {}

        assert_eq!(arr.get(0).x, 1);
        assert_eq!(arr.get(32).y, 2);
        assert_eq!(arr.size, 100);
        assert_eq!(arr.current, 100);
        arr.get_mut(2).y = 10;
        assert_eq!(arr.get(2).y, 10);
    }
}
