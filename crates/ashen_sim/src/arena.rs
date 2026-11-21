//! Generationelle Arena: stabile, wiederverwendbare IDs ohne hängende Referenzen.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Typisierte ID mit Generationszähler.
#[derive(Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Id<T> {
    pub index: u32,
    pub generation: u32,
    #[serde(skip)]
    _m: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn new(index: u32, generation: u32) -> Self {
        Id { index, generation, _m: PhantomData }
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, o: &Self) -> bool {
        self.index == o.index && self.generation == o.generation
    }
}
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.index.hash(h);
        self.generation.hash(h);
    }
}
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}v{}", self.index, self.generation)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// Arena mit Freiliste. Iteration erfolgt in Indexreihenfolge (deterministisch).
#[derive(Clone, Serialize, Deserialize)]
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    len: usize,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Arena { slots: Vec::new(), free: Vec::new(), len: 0 }
    }
}

impl<T> Arena<T> {
    pub fn insert(&mut self, v: T) -> Id<T> {
        self.len += 1;
        if let Some(i) = self.free.pop() {
            let s = &mut self.slots[i as usize];
            s.generation = s.generation.wrapping_add(1);
            s.value = Some(v);
            Id::new(i, s.generation)
        } else {
            self.slots.push(Slot { generation: 0, value: Some(v) });
            Id::new(self.slots.len() as u32 - 1, 0)
        }
    }

    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        let s = self.slots.get_mut(id.index as usize)?;
        if s.generation != id.generation {
            return None;
        }
        let v = s.value.take();
        if v.is_some() {
            self.free.push(id.index);
            self.len -= 1;
        }
        v
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        let s = self.slots.get(id.index as usize)?;
        if s.generation == id.generation { s.value.as_ref() } else { None }
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        let s = self.slots.get_mut(id.index as usize)?;
        if s.generation == id.generation { s.value.as_mut() } else { None }
    }

    pub fn contains(&self, id: Id<T>) -> bool {
        self.get(id).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.value.as_ref().map(|v| (Id::new(i as u32, s.generation), v)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id<T>, &mut T)> {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(i, s)| {
                let g = s.generation;
                s.value.as_mut().map(|v| (Id::new(i as u32, g), v))
            })
    }

    /// Alle aktuell gültigen IDs (Schnappschuss, erlaubt Mutation während der Iteration).
    pub fn ids(&self) -> Vec<Id<T>> {
        self.iter().map(|(id, _)| id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generations_prevent_stale_access() {
        let mut a: Arena<i32> = Arena::default();
        let x = a.insert(1);
        assert_eq!(a.remove(x), Some(1));
        let y = a.insert(2);
        assert_eq!(x.index, y.index);
        assert!(a.get(x).is_none());
        assert_eq!(a.get(y), Some(&2));
        assert_eq!(a.len(), 1);
    }
}
