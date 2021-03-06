use std::slice;
use std::option;
use std::default::Default;
use std::fmt;

use clear::Clear;


pub struct SingularField<T> {
    value: Option<Box<T>>,
    set: bool,
}

impl<T> SingularField<T> {
    #[inline]
    pub fn some(value: T) -> SingularField<T> {
        SingularField {
            value: Some(box value),
            set: true,
        }
    }

    #[inline]
    pub fn none() -> SingularField<T> {
        SingularField {
            value: None,
            set: false,
        }
    }

    #[inline]
    pub fn from_option(option: Option<T>) -> SingularField<T> {
        match option {
            Some(x) => SingularField::some(x),
            None => SingularField::none(),
        }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.set
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.set {
            Some(*self.value.unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ref<'a>(&'a self) -> Option<&'a T> {
        if self.set {
            Some(&**self.value.get_ref())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        if self.set {
            Some(&mut **self.value.get_mut_ref())
        } else {
            None
        }
    }

    #[inline]
    pub fn get_ref<'a>(&'a self) -> &'a T {
        self.as_ref().unwrap()
    }

    #[inline]
    pub fn get_mut_ref<'a>(&'a mut self) -> &'a mut T {
        self.as_mut().unwrap()
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        match self.as_ref() {
            Some(x) => slice::ref_slice(x),
            None => &[]
        }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        match self.as_mut() {
            Some(x) => slice::mut_ref_slice(x),
            None => &mut []
        }
    }

    #[inline]
    pub fn unwrap(self) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            fail!();
        }
    }

    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            def
        }
    }

    #[inline]
    pub fn unwrap_or_else(self, f: || -> T) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            f()
        }
    }

    #[inline]
    pub fn map<U>(self, f: |T| -> U) -> SingularField<U> {
        SingularField::from_option(self.into_option().map(f))
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> option::Item<&'a T> {
        self.as_ref().move_iter()
    }

    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> option::Item<&'a mut T> {
        self.as_mut().move_iter()
    }

    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.set {
            self.set = false;
            Some(*self.value.take().unwrap())
        } else {
            None
        }
    }

//}

//impl<T> Clear for SingularField<T> {
    #[inline]
    pub fn clear(&mut self) {
        self.set = false;
    }
}

impl<T : Default+Clear> SingularField<T> {
    #[inline]
    pub fn unwrap_or_default(mut self) -> T {
        if self.set {
            self.unwrap()
        } else if self.value.is_some() {
            self.value.clear();
            *self.value.unwrap()
        } else {
            Default::default()
        }
    }

    #[inline]
    pub fn set_default<'a>(&'a mut self) -> &'a mut T {
        self.set = true;
        if self.value.is_some() {
            self.value.get_mut_ref().clear();
        } else {
            self.value = Some(box Default::default());
        }
        self.get_mut_ref()
    }
}

impl<T> Default for SingularField<T> {
    #[inline]
    fn default() -> SingularField<T> {
        SingularField::none()
    }
}

impl<T : Clone> Clone for SingularField<T> {
    #[inline]
    fn clone(&self) -> SingularField<T> {
        if self.set {
            SingularField::some(self.get_ref().clone())
        } else {
            SingularField::none()
        }
    }
}

impl<T : fmt::Show> fmt::Show for SingularField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({})", *self.get_ref())
        } else {
            write!(f, "None")
        }
    }
}

impl<T : PartialEq> PartialEq for SingularField<T> {
    #[inline]
    fn eq(&self, other: &SingularField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for SingularField<T> {}

impl<T : PartialOrd> PartialOrd for SingularField<T> {
    #[inline]
    fn lt(&self, other: &SingularField<T>) -> bool {
        self.as_ref() < other.as_ref()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use clear::Clear;

    #[test]
    fn test_set_default_clears() {
        #[deriving(Default)]
        struct Foo {
            b: int,
        }

        impl Clear for Foo {
            fn clear(&mut self) {
                self.b = 0;
            }
        }

        let mut x = SingularField::some(Foo { b: 10 });
        x.clear();
        x.set_default();
        assert_eq!(0, x.get_ref().b);

        x.get_mut_ref().b = 11;
        // without clear
        x.set_default();
        assert_eq!(0, x.get_ref().b);
    }
}
