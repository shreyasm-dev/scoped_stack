use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ScopedStack<K, V> where K: std::cmp::Eq + std::hash::Hash {
  values: HashMap<K, V>,
  child: Option<Box<ScopedStack<K, V>>>,
}

impl<K, V> ScopedStack<K, V> where K: std::cmp::Eq + std::hash::Hash {
  pub fn new() -> Self {
    ScopedStack {
      values: HashMap::new(),
      child: None,
    }
  }

  pub fn push_scope(&mut self) {
    let child = Box::new(ScopedStack::new());
    self.child = Some(child);
  }

  pub fn pop_scope(&mut self) {
    if let Some(child) = self.child.take() {
      *self = *child;
    }
  }

  pub fn insert(&mut self, key: K, value: V) {
    if let Some(child) = self.child.as_mut() {
      child.insert(key, value);
    } else {
      self.values.insert(key, value);
    }
  }

  pub fn get(&self, key: &K) -> Option<&V> {
    if let Some(child) = self.child.as_ref() {
      if child.has(key) {
        child.get(key)
      } else {
        self.values.get(key)
      }
    } else {
      self.values.get(key)
    }
  }

  pub fn has(&self, key: &K) -> bool {
    if let Some(child) = self.child.as_ref() {
      child.has(key)
    } else {
      self.values.contains_key(key)
    }
  }

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
