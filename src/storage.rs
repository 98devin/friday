use derive_more::{From, Into};

use std::borrow::{Borrow, BorrowMut};
use std::collections::{hash_map, HashMap};
use std::marker::PhantomData;

pub trait Storage<'r, Ref> {
    type Stored: ?Sized + 'r;
    type StoredRef: Borrow<Self::Stored> + 'r;
    fn get(&'r self, r: Ref) -> Option<Self::StoredRef>;
}

pub trait StorageMut<'r, Ref>: Storage<'r, Ref> {
    type StoredRefMut: BorrowMut<Self::Stored> + 'r;
    fn get_mut(&'r mut self, r: Ref) -> Option<Self::StoredRefMut>;
    fn set(&'r mut self, r: Ref, t: Self::Stored) -> Self::StoredRefMut
    where
        Self::Stored: Sized;
}

#[derive(Debug, Clone, Copy)]
pub struct RefCounter<Ref> {
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

#[derive(Debug, Clone)]
pub struct RefCounterIter<Ref> {
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
            vec: Vec::with_capacity(16),
        }
    }

    fn convert_pair((i, x): (usize, Option<T>)) -> Option<(Ref, T)> {
        x.map(|v| (Ref::from(i), v))
    }

    fn convert_pair_ref((i, x): (usize, &Option<T>)) -> Option<(Ref, &T)> {
        x.as_ref().map(|v| (Ref::from(i), v))
    }

    fn convert_pair_ref_mut((i, x): (usize, &mut Option<T>)) -> Option<(Ref, &mut T)> {
        x.as_mut().map(|v| (Ref::from(i), v))
    }
}

impl<'r, T: 'r, Ref> Storage<'r, Ref> for VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type Stored = T;
    type StoredRef = &'r T;
    fn get(&'r self, r: Ref) -> Option<Self::StoredRef> {
        let ix = r.into();
        match self.vec.get(ix) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }
}

impl<'r, T: 'r, Ref> StorageMut<'r, Ref> for VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize>,
{
    type StoredRefMut = &'r mut T;

    fn get_mut(&'r mut self, r: Ref) -> Option<Self::StoredRefMut> {
        let ix = r.into();
        match self.vec.get_mut(ix) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }

    fn set(&'r mut self, r: Ref, t: Self::Stored) -> Self::StoredRefMut {
        let ix = r.into();
        if ix >= self.vec.len() {
            self.vec.resize_with(ix + 1, Default::default);
        }
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
    type IntoIter = std::iter::FilterMap<
        std::iter::Enumerate<std::vec::IntoIter<Option<T>>>,
        fn((usize, Option<T>)) -> Option<(Ref, T)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.vec
            .into_iter()
            .enumerate()
            .filter_map(VecStorage::convert_pair)
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize> + Clone,
{
    type Item = (Ref, &'s T);

    type IntoIter = std::iter::FilterMap<
        std::iter::Enumerate<std::slice::Iter<'s, Option<T>>>,
        fn((usize, &'s Option<T>)) -> Option<(Ref, &'s T)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.vec
            .iter()
            .enumerate()
            .filter_map(VecStorage::convert_pair_ref)
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s mut VecStorage<T, Ref>
where
    Ref: From<usize> + Into<usize> + Clone,
{
    type Item = (Ref, &'s mut T);

    type IntoIter = std::iter::FilterMap<
        std::iter::Enumerate<std::slice::IterMut<'s, Option<T>>>,
        fn((usize, &'s mut Option<T>)) -> Option<(Ref, &'s mut T)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.vec
            .iter_mut()
            .enumerate()
            .filter_map(VecStorage::convert_pair_ref_mut)
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

    fn clone_first_ref((r, x): (&Ref, T)) -> (Ref, T)
    where
        Ref: Clone,
    {
        (r.clone(), x)
    }
}

impl<'r, T: 'r, Ref> Storage<'r, Ref> for HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    type Stored = T;
    type StoredRef = &'r T;
    fn get(&'r self, r: Ref) -> Option<Self::StoredRef> {
        self.hash.get(&r)
    }
}

impl<'r, T: 'r, Ref> StorageMut<'r, Ref> for HashStorage<T, Ref>
where
    Ref: Eq + std::hash::Hash,
{
    type StoredRefMut = &'r mut T;

    fn get_mut(&'r mut self, r: Ref) -> Option<Self::StoredRefMut> {
        self.hash.get_mut(&r)
    }

    fn set(&'r mut self, r: Ref, t: Self::Stored) -> Self::StoredRefMut {
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
    type IntoIter = std::iter::Map<hash_map::Iter<'s, Ref, T>, fn((&Ref, &'s T)) -> (Ref, &'s T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.hash.iter().map(HashStorage::clone_first_ref)
    }
}

// FIXME: Implement actual iterator struct.
impl<'s, T, Ref> IntoIterator for &'s mut HashStorage<T, Ref>
where
    Ref: Clone + Eq + std::hash::Hash,
{
    type Item = (Ref, &'s mut T);
    type IntoIter =
        std::iter::Map<hash_map::IterMut<'s, Ref, T>, fn((&Ref, &'s mut T)) -> (Ref, &'s mut T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.hash.iter_mut().map(HashStorage::clone_first_ref)
    }
}
