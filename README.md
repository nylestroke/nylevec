# NyleVec - Own implementation of rust vector

Example usage:

```rust
fn main() {
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
```
