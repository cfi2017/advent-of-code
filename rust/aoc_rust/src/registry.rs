use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;

/// Scope is an enum that provides a scoped value or an unscoped value.
/// Both the scope and the value must implement Hash, PartialEq and Eq.
#[derive(Hash, Eq, PartialEq)]
pub enum Scope<S, N>
    where S: Hash + PartialEq + Eq,
          N: Hash + PartialEq + Eq,
{
    Scoped(S, N),
    Unscoped(N),
}

impl<S, N> Scope<S, N>
    where S: Hash + PartialEq + Eq,
          N: Hash + PartialEq + Eq,
{
    /// unscope returns the unscoped version of a given value. If called on an already unscoped
    /// value, it returns itself.
    fn unscope(self) -> Scope<S, N> {
        match self {
            Scope::Scoped(_, n) => Scope::Unscoped(n),
            _ => self
        }
    }
}

/// ScopedRegistry wraps a HashMap and allows to bind an entry to a scope. Entries can be either
/// scoped or unscoped.
pub struct ScopedRegistry<S, N, T>
    where S: Hash + PartialEq + Eq,
          N: Hash + PartialEq + Eq,
{
    registry: HashMap<Scope<S, N>, T>,
}

impl<S, N, T> ScopedRegistry<S, N, T>
    where S: Hash + PartialEq + Eq,
          N: Hash + PartialEq + Eq,
{
    pub fn new() -> ScopedRegistry<S, N, T> {
        ScopedRegistry {
            registry: HashMap::new()
        }
    }

    // Utility Functions operating on the underlying map

    /// insert inserts a value for the given scope
    pub fn insert(&mut self, scope: Scope<S, N>, value: T) {
        self.registry.insert(scope, value);
    }

    /// get returns the value for a given scoped key, or the unscoped equivalent if available.
    pub fn get(&self, scope: Scope<S, N>) -> Option<&T> {
        self.registry.get(&scope).or(self.registry.get(&scope.unscope()))
    }

    /// get_mut functions like get, but returning a mutable value instead
    pub fn get_mut(&mut self, scope: Scope<S, N>) -> Option<&mut T> {
        return if self.registry.contains_key(&scope) {
            self.registry.get_mut(&scope)
        } else {
            self.registry.get_mut(&scope.unscope())
        };
    }

    pub fn get_exact(&self, scope: Scope<S, N>) -> Option<&T> {
        self.registry.get(&scope)
    }

    pub fn get_exact_mut(&mut self, scope: Scope<S, N>) -> Option<&mut T> {
        self.registry.get_mut(&scope)
    }

    pub fn clear(&mut self) {
        self.registry.clear()
    }

    pub fn iter(&self) -> Iter<'_, Scope<S, N>, T> {
        self.registry.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Scope<S, N>, T> {
        self.registry.iter_mut()
    }

    pub fn remove(&mut self, scope: Scope<S, N>) -> Option<T> {
        self.registry.remove(&scope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populate_registry() -> ScopedRegistry<i32, i32, i32> {
        let mut registry = ScopedRegistry::new();
        registry.insert(Scope::Scoped(1, 2), 3);
        registry.insert(Scope::Unscoped(2), 4);
        registry.insert(Scope::Unscoped(5), 6);

        registry.insert(Scope::Scoped(1, 9), 3);
        registry.insert(Scope::Scoped(3, 9), 8);

        return registry;
    }

    #[test]
    fn can_insert_scoped() {
        let mut map = ScopedRegistry::new();
        map.insert(Scope::Scoped(1234, 123), 999);
    }

    #[test]
    fn can_insert_unscoped() {
        let mut map: ScopedRegistry<i32, i32, i32> = ScopedRegistry::new();
        map.insert(Scope::Unscoped(123), 321);
    }

    #[test]
    fn can_get_scoped() {
        let map = populate_registry();
        assert_eq!(*map.get(Scope::Scoped(1, 2)).unwrap(), 3);
    }

    #[test]
    fn can_get_unscoped() {
        let map = populate_registry();
        assert_eq!(*map.get(Scope::Unscoped(5)).unwrap(), 6);
    }

    #[test]
    fn can_get_unscoped_with_scoped_query() {
        let map = populate_registry();
        assert_eq!(*map.get(Scope::Scoped(1, 5)).unwrap(), 6);
    }

    #[test]
    fn scoped_result_overrides_unscoped() {
        let map = populate_registry();
        assert_eq!(*map.get(Scope::Scoped(1, 2)).unwrap(), 3);
    }

    #[test]
    fn unset_value_returns_none() {
        let map = populate_registry();
        assert_eq!(map.get(Scope::Unscoped(8)), None);
    }

    #[test]
    fn cannot_access_other_scope() {
        let map = populate_registry();
        assert_eq!(map.get(Scope::Scoped(2, 9)), None);
        assert_eq!(*map.get(Scope::Scoped(3, 9)).unwrap(), 8);
    }

}
