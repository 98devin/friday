use derive_more::{From, Into};

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait Storage<Ref>
where
    Self: Sized,
{
    type Stored: ?Sized;
    type StoredRef: Borrow<Self::Stored>;
    fn get(self, r: Ref) -> Option<Self::StoredRef>;
}

pub trait StorageMut<Ref>: Storage<Ref>
where
    Self::Stored: Sized,
    Self::StoredRef: BorrowMut<Self::Stored>,
{
    fn set(self, r: Ref, t: Self::Stored) -> Self::StoredRef;
    fn get_mut(self, r: Ref) -> Option<Self::StoredRef> {
        self.get(r)
    }

    // fn get_or_set_with<F>(self, r: Ref, f: F) -> Self::StoredRef
    // where
    //     Ref: Clone,
    //     F: FnOnce() -> Self::Stored,
    // {
    //     match self.get_mut(r.clone()) {
    //         Some(stored_ref) => stored_ref,
    //         None => self.set(r, f()),
    //     }
    // }

    // fn get_or_set(self, r: Ref, t: Self::Stored) -> Self::StoredRef
    // where
    //     Ref: Clone,
    // {
    //     self.get_or_set_with(r, move || t)
    // }
}

#[derive(Debug, Clone, Copy)]
pub struct RefCounter<Ref>
where
    Ref: From<usize>,
{
    _ref: PhantomData<*const Ref>,
    count: usize,
}

impl<Ref> RefCounter<Ref>
where
    Ref: From<usize>,
{
    pub fn new() -> Self {
        RefCounter {
            count: 0,
            _ref: PhantomData,
        }
    }

    pub fn make_ref(&mut self) -> Ref {
        let new_ref = Ref::from(self.count);
        self.count += 1;
        new_ref
    }
}

pub struct RefCounterIter<Ref>
where
    Ref: From<usize>,
{
    _ref: PhantomData<*const Ref>,
    curr: usize,
    stop: usize,
}

impl<Ref> Iterator for RefCounterIter<Ref>
where
    Ref: From<usize>,
{
    type Item = Ref;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.stop {
            return None;
        }
        let next = Some(Ref::from(self.curr));
        self.curr += 1;
        next
    }
}

impl<Ref> IntoIterator for &'_ RefCounter<Ref>
where
    Ref: From<usize>,
{
    type Item = Ref;
    type IntoIter = RefCounterIter<Ref>;
    fn into_iter(self) -> Self::IntoIter {
        RefCounterIter {
            _ref: PhantomData,
            curr: 0,
            stop: self.count,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct VecRef(pub usize);

#[derive(Debug, Clone)]
pub struct VecStorage<T, Ref = VecRef>
where
    Ref: From<usize> + Into<usize>,
{
    _ref: PhantomData<*const Ref>,
    vec: Vec<Option<T>>,
}

impl<T, Ref> VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    pub fn new() -> Self {
        VecStorage {
            _ref: PhantomData,
            vec: Vec::new(),
        }
    }
}

impl<'s, T, Ref> Storage<Ref> for &'s VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Stored = T;
    type StoredRef = &'s T;
    fn get(self, r: Ref) -> Option<Self::StoredRef> {
        let ix = r.into();
        match self.vec.get(ix) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }
}

impl<'s, T, Ref> Storage<Ref> for &'s mut VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Stored = T;
    type StoredRef = &'s mut T;
    fn get(self, r: Ref) -> Option<Self::StoredRef> {
        let ix = r.into();
        match self.vec.get_mut(ix) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }
}

impl<'s, T, Ref> StorageMut<Ref> for &'s mut VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    fn set(self, r: Ref, t: Self::Stored) -> Self::StoredRef {
        let ix = r.into();
        self.vec.resize_with(ix + 1, Default::default);
        self.vec[ix] = Some(t);
        self.vec[ix].as_mut().unwrap()
    }
}

// FIXME: Implement actual iterator struct.
impl<T, Ref> IntoIterator for VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Item = (Ref, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<_> = self
            .vec
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|v| (Ref::from(i), v)))
            .collect();
        v.into_iter()
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Item = (Ref, &'s T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<_> = self
            .vec
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.as_ref().map(|v| (Ref::from(i), v)))
            .collect();
        v.into_iter()
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s mut VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Item = (Ref, &'s mut T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<_> = self
            .vec
            .iter_mut()
            .enumerate()
            .filter_map(|(i, x)| x.as_mut().map(|v| (Ref::from(i), v)))
            .collect();
        v.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct HashStorage<T, Ref = VecRef>
where
    Ref: Eq + std::hash::Hash,
{
    hash: HashMap<Ref, T>,
}

impl<T, Ref> HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            hash: HashMap::new(),
        }
    }
}

impl<'s, T, Ref> Storage<Ref> for &'s HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    type Stored = T;
    type StoredRef = &'s T;
    fn get(self, r: Ref) -> Option<Self::StoredRef> {
        self.hash.get(&r)
    }
}

impl<'s, T, Ref> Storage<Ref> for &'s mut HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    type Stored = T;
    type StoredRef = &'s mut T;
    fn get(self, r: Ref) -> Option<Self::StoredRef> {
        self.hash.get_mut(&r)
    }
}

impl<'s, T, Ref> StorageMut<Ref> for &'s mut HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    fn set(self, r: Ref, t: Self::Stored) -> Self::StoredRef {
        use std::collections::hash_map::Entry;

        match self.hash.entry(r) {
            Entry::Occupied(mut o) => {
                *o.get_mut() = t;
                o.into_mut()
            }
            Entry::Vacant(v) => v.insert(t),
        }
    }
}

impl<T, Ref> IntoIterator for HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    type Item = (Ref, T);
    type IntoIter = <HashMap<Ref, T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.hash.into_iter()
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s HashStorage<T, Ref>
where
    Ref: Clone + Eq + std::hash::Hash,
{
    type Item = (Ref, &'s T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<_> = self.hash.iter().map(|(r, t)| (r.clone(), t)).collect();
        v.into_iter()
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s mut HashStorage<T, Ref>
where
    Ref: Clone + Eq + std::hash::Hash,
{
    type Item = (Ref, &'s mut T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<_> = self.hash.iter_mut().map(|(r, t)| (r.clone(), t)).collect();
        v.into_iter()
    }
}