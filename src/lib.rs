use std::collections::HashMap;

/// A scoped stack is a stack of hashmaps that allows you to push and pop scopes.
/// When you push a scope, a new hashmap is created and pushed onto the stack.
/// When you pop a scope, the top hashmap is popped off the stack.
/// When you insert a value, it is inserted into the top hashmap.
/// When you get a value, it is searched for in the top hashmap, and if it is not found, it is searched for in the next hashmap down the stack.
/// When you remove a value, it is removed from the top hashmap, and if it is not found, it is removed from the next hashmap down the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct ScopedStack<K, V> where K: std::cmp::Eq + std::hash::Hash {
  values: HashMap<K, V>,
  child: Option<Box<ScopedStack<K, V>>>,
}

impl<K, V> ScopedStack<K, V> where K: std::cmp::Eq + std::hash::Hash {
  /// Creates a new scoped stack.
  pub fn new() -> Self {
    ScopedStack {
      values: HashMap::new(),
      child: None,
    }
  }

  /// Pushes a new scope onto the stack.
  pub fn push_scope(&mut self) {
    if let Some(child) = self.child.as_mut() {
      child.push_scope();
    } else {
      self.child = Some(Box::new(ScopedStack::new()));
    }
  }

  /// Pops the top scope off the stack.
  pub fn pop_scope(&mut self) {
    if let Some(child) = self.child.as_mut() {
      if child.child.is_some() {
        child.pop_scope();
      } else {
        self.child = None;
      }
    } else {
      self.child = None;
    }
  }

  /// Inserts a value into the top scope.
  pub fn insert(&mut self, key: K, value: V) {
    if let Some(child) = self.child.as_mut() {
      child.insert(key, value);
    } else {
      self.values.insert(key, value);
    }
  }

  /// Inserts a value at the top-most scope where the key already exists (or the bottom scope if it does not exist)
  pub fn insert_existing(&mut self, key: K, value: V) {
    if let Some(child) = self.child.as_mut() {
      if child.has(&key) {
        child.insert_existing(key, value);
      } else {
        self.values.insert(key, value);
      }
    } else {
      self.values.insert(key, value);
    }
  }

  /// Gets a value from the top scope, or any scope below it if it is not found in the top scope.
  pub fn get(&self, key: &K) -> Option<&V> {
    let mut value = self.values.get(key);
    let mut child = self.child.as_ref();

    loop {
      match child {
        Some(c) => {
          if let Some(v) = c.values.get(key) {
            value = Some(v);
          }
          child = c.child.as_ref();
        },
        None => break,
      }
    }

    value
  }

  /// Checks if a value exists in the top scope, or any scope below it.
  pub fn has(&self, key: &K) -> bool {
    if let Some(child) = self.child.as_ref() {
      child.has(key)
    } else {
      self.values.contains_key(key)
    }
  }

  /// Removes a value from the top scope, or any scope below it if it is not found in the top scope.
  pub fn remove(&mut self, key: &K) -> Option<V> {
    if let Some(child) = self.child.as_mut() {
      child.remove(key)
    } else {
      self.values.remove(key)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let stack = ScopedStack::<String, String>::new();
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child, None);
  }

  #[test]
  fn test_push() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.push_scope();
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child.is_some(), true);
  }

  #[test]
  fn test_push_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.push_scope();
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child.is_some(), true);
  }

  #[test]
  fn test_pop() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.push_scope();
    stack.pop_scope();
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child, None);
  }

  #[test]
  fn test_pop_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.push_scope();
    stack.pop_scope();
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child, None);
  }

  #[test]
  fn test_insert() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    assert_eq!(stack.values.len(), 1);
    assert_eq!(stack.values.get("foo"), Some(&"bar".to_string()));
    assert_eq!(stack.child, None);
  }

  #[test]
  fn test_insert_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.push_scope();
    stack.insert("foo".to_string(), "bar".to_string());
    assert_eq!(stack.values.len(), 0);
    assert_eq!(stack.child.is_some(), true);
    assert_eq!(stack.child.as_ref().unwrap().values.len(), 1);
    assert_eq!(stack.get(&"foo".to_string()), Some(&"bar".to_string()));
  }

  #[test]
  fn test_insert_existing_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    stack.push_scope();
    stack.insert_existing("foo".to_string(), "baz".to_string());
    stack.pop_scope();
    assert_eq!(stack.values.len(), 1);
    assert_eq!(stack.child.is_some(), false);
    assert_eq!(stack.get(&"foo".to_string()), Some(&"baz".to_string()));
  }

  #[test]
  fn test_get() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    assert_eq!(stack.get(&"foo".to_string()), Some(&"bar".to_string()));
  }

  #[test]
  fn test_get_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    stack.push_scope();
    stack.insert("foo".to_string(), "baz".to_string());
    assert_eq!(stack.get(&"foo".to_string()), Some(&"baz".to_string()));
  }

  #[test]
  fn test_has() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    assert_eq!(stack.has(&"foo".to_string()), true);
  }

  #[test]
  fn test_has_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    stack.push_scope();
    stack.insert("foo".to_string(), "baz".to_string());
    assert_eq!(stack.has(&"foo".to_string()), true);
  }

  #[test]
  fn test_remove() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    assert_eq!(stack.remove(&"foo".to_string()), Some("bar".to_string()));
    assert_eq!(stack.values.len(), 0);
  }

  #[test]
  fn test_remove_scope() {
    let mut stack = ScopedStack::<String, String>::new();
    stack.insert("foo".to_string(), "bar".to_string());
    stack.push_scope();
    stack.insert("foo".to_string(), "baz".to_string());
    assert_eq!(stack.remove(&"foo".to_string()), Some("baz".to_string()));
    assert_eq!(stack.values.len(), 1);
    assert_eq!(stack.get(&"foo".to_string()), Some(&"bar".to_string()));
  }
}
