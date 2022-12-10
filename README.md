# scoped_stack

scoped_stack is a Rust library that provides a stack that can be scoped. Each `ScopedStack` has a `HashMap` of values and a child `ScopedStack`. When a scope is pushed, a new `ScopedStack` is created with the uppermost `ScopedStack` as its parent. When a scope is popped, the uppermost `ScopedStack` is removed. When a value is pushed, it is added to the uppermost `ScopedStack`. When a value is popped, it is removed from the uppermost `ScopedStack` with that key. When a value is looked up, the value from the uppermost `ScopedStack` with that key is returned.

## Example

```rust
use scoped_stack::ScopedStack;

fn main() {
  let mut stack = ScopedStack::<&str, i32>::new();

  stack.insert("a", 1);
  stack.insert("b", 2);
  stack.insert("c", 3);

  assert_eq!(stack.get(&"a"), Some(&1));
  assert_eq!(stack.get(&"b"), Some(&2));
  assert_eq!(stack.get(&"c"), Some(&3));

  stack.push_scope();

  stack.insert("a", 4);
  stack.insert("b", 5);

  assert_eq!(stack.get(&"a"), Some(&4));
  assert_eq!(stack.get(&"b"), Some(&5));
  assert_eq!(stack.get(&"c"), Some(&3));

  stack.pop_scope();

  assert_eq!(stack.get(&"a"), Some(&1));
  assert_eq!(stack.get(&"b"), Some(&2));
  assert_eq!(stack.get(&"c"), Some(&3));
}
```
