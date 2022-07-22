use std::any::Any;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct BoxId<T>
where
    T: ?Sized,
{
    hash: usize,
    inner: Box<T>,
}

impl<T> BoxId<T>
where
    T: 'static,
{
    pub fn new(inner: T) -> Self {
        let ptr = Box::into_raw(inner.into());

        Self {
            hash: ptr as usize,
            inner: unsafe { Box::from_raw(ptr) },
        }
    }
}

impl<T> BoxId<T>
where
    T: ?Sized,
{
    pub fn from_box(inner: Box<T>) -> Self {
        let ptr = Box::into_raw(inner);

        Self {
            hash: ptr as *const usize as usize,
            inner: unsafe { Box::from_raw(ptr) },
        }
    }

    pub fn from_raw(hash: usize, inner: Box<T>) -> Self {
        Self { hash, inner }
    }

    pub fn hash(&self) -> usize {
        self.hash
    }

    pub fn into_raw(self) -> (usize, Box<T>) {
        (self.hash, self.inner)
    }
}

impl BoxId<dyn Any> {
    pub fn downcast<T>(self) -> Result<BoxId<T>, BoxId<dyn Any + 'static>>
    where
        T: Any,
    {
        Ok(BoxId {
            hash: self.hash,
            inner: match self.inner.downcast::<T>() {
                Ok(inner) => inner,
                Err(inner) => {
                    return Err(BoxId {
                        hash: self.hash,
                        inner,
                    })
                }
            },
        })
    }
}

impl<T> Hash for BoxId<T>
where
    T: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.hash)
    }
}

impl<T> PartialEq for BoxId<T>
where
    T: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl<T> Eq for BoxId<T> where T: ?Sized {}

impl<T> PartialOrd for BoxId<T>
where
    T: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl<T> Ord for BoxId<T>
where
    T: ?Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl<T> Deref for BoxId<T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> DerefMut for BoxId<T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T> AsRef<T> for BoxId<T> {
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T> AsMut<T> for BoxId<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut()
    }
}
