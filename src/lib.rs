use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::default::Default;

pub trait Recyclable {
    fn put(self);
    fn get() -> Self;
}

pub struct Recycle<T> where T : Recyclable {
    t : Option<T>,
}

impl<T : Recyclable> Recycle<T> {
    pub fn new() -> Self {
        Recycle {
            t: Some(T::get())
        }
    }
}

impl<T> Drop for Recycle<T>
where T : Recyclable {
    fn drop(&mut self) {
        if let Some(t) = self.t.take() {
            T::put(t)
        }
    }
}

impl<T : Recyclable> Deref for Recycle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.t.as_ref().unwrap()
    }
}

impl<T : Recyclable+'static> DerefMut for Recycle<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.t.as_mut().unwrap()
    }
}

macro_rules! impl_recyclable {
    ($tl:ident, $t:ty) => {
        impl_recyclable!($tl, $t, Default::default());
    };
    ($tl:ident, $t:ty, $constructor:expr) => {
        thread_local! {
            static $tl: RefCell<Vec<$t>> = RefCell::new(Vec::new())
        }

        impl Recyclable for $t {
            fn put(mut self) {
                $tl.with(|p| {
                    let mut p = p.borrow_mut();
                    self.clear();
                    p.push(self);
                })
            }

            fn get() -> Self {
                $tl.with(|p| {
                    let mut p = p.borrow_mut();
                    p.pop()
                }).unwrap_or($constructor)
            }
        }
    };
}

impl_recyclable!(TL_V_U8, Vec<u8>);
impl_recyclable!(TL_V_U16, Vec<u16>);
impl_recyclable!(TL_V_U32, Vec<u32>);
impl_recyclable!(TL_V_U64, Vec<u64>);
impl_recyclable!(TL_V_I8, Vec<i8>);
impl_recyclable!(TL_V_I16, Vec<i16>);
impl_recyclable!(TL_V_I32, Vec<i32>);
impl_recyclable!(TL_V_I64, Vec<i64>);
impl_recyclable!(TL_STR, String);
// No way to reset PathBuf to original value
//impl_recyclable!(TL_PATHBUF, PathBuf, PathBuf::new(), self.);
