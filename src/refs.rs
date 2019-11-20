use std::marker::PhantomData;

use crate::storage::*;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref<T>(PhantomData<*const T>, usize);

impl<T: std::fmt::Debug + Default> std::fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "Ref({:#?},{:#?})", T::default(), self.1)
        } else {
            write!(f, "Ref({:?},{:?})", T::default(), self.1)
        }
    }
}

impl<T> From<usize> for Ref<T> {
    fn from(u: usize) -> Self {
        Ref(PhantomData, u)
    }
}

impl<T> Into<usize> for Ref<T> {
    fn into(self) -> usize {
        self.1
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Expr;
pub type ExprRef = Ref<Expr>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Patn;
pub type PatnRef = Ref<Patn>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Decl;
pub type DeclRef = Ref<Decl>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cons;
pub type ConsRef = Ref<Cons>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Modl;
pub type ModlRef = Ref<Modl>;

#[derive(Debug, Clone)]
pub struct IdCounter {
    pub expr: RefCounter<ExprRef>,
    pub patn: RefCounter<PatnRef>,
    pub decl: RefCounter<DeclRef>,
    pub cons: RefCounter<ConsRef>,
    pub modl: RefCounter<ModlRef>,
}

impl IdCounter {
    pub fn new() -> Self {
        IdCounter {
            expr: RefCounter::new(),
            patn: RefCounter::new(),
            decl: RefCounter::new(),
            cons: RefCounter::new(),
            modl: RefCounter::new(),
        }
    }
}
