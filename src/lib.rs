use std::ptr::NonNull;
use std::alloc;
use std::mem;

pub struct NyleVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> NyleVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(), 
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        // Check if type no zero sized
        assert_ne!(mem::size_of::<T>(), 0, "Zero sized types are not allowed");

        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4)
                .expect("Could not allocate memory");
            
            // The layout is hardcoded to be 4 * size_of<T> and size_of<T> > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr)
                .expect("Could not allocate memory");

            // Pointer is non-null and we have just allocated enough space for this item (and 3 more).
            // The memeory previously at pointer is not read
            unsafe { ptr.as_ptr().write(item); };
            
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self.len.checked_mul(mem::size_of::<T>())
                .expect("Cannot reach memory location");

            assert!(offset < isize::MAX as usize, "Wrapped usize");

            // Offset cannot wrap around  and pointer is pointing to valid memory
            // And writing to an offet at self.len is valid
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        } else {
            // Check if len no more than capacity
            debug_assert!(self.len == self.capacity);

            let size = mem::size_of::<T>() * self.capacity;
            let align = mem::align_of::<T>();

            size.checked_add(size % align).expect("Cannot allocate");

            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");            

            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(
                    size,
                    align,
                );

                let new_size = mem::size_of::<T>() * new_capacity;

                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");

                ptr.as_ptr().add(self.len).write(item);
                
                self.ptr = ptr;
                self.capacity = new_capacity;
                self.len += 1;
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe {
            &*self.ptr.as_ptr().add(index)
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}


impl<T> Drop for NyleVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(
                std::slice::from_raw_parts_mut(
                    self.ptr.as_ptr(), 
                    self.len
                )
            );

            let layout = alloc::Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>()
            );

            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let mut vec = NyleVec::<usize>::new();

       vec.push(1usize);
       vec.push(2);
       vec.push(3);
       vec.push(4);
       vec.push(5);

       for n in 0..vec.len() {
        assert_eq!(vec.get(n), Some(&(n + 1)))
       }

       assert_eq!(vec.get(3), Some(&4));
       assert_eq!(vec.capacity(), 8);
       assert_eq!(vec.len(), 5);
    }
}