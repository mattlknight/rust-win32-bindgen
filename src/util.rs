use std::ffi::CString;

pub trait BoolUtil {
    fn as_either<T>(&self, if_true: T, if_false: T) -> T;
}

impl BoolUtil for bool {
    fn as_either<T>(&self, if_true: T, if_false: T) -> T {
        if *self { if_true } else { if_false }
    }
}

pub trait CheckedFrom<T> {
    fn checked_from(T) -> Self;
}

impl CheckedFrom<usize> for u32 {
    fn checked_from(v: usize) -> u32 {
        if ! (::std::u32::MIN as usize <= v) {
            panic!("underflow on conversion from usize {} to u32", v);
        } else if ! (v <= ::std::u32::MAX as usize) {
            panic!("overflow on conversion from usize {} to u32", v);
        } else {
            v as u32
        }
    }
}

impl CheckedFrom<usize> for i32 {
    fn checked_from(v: usize) -> i32 {
        if ! (::std::i32::MIN as usize <= v) {
            panic!("underflow on conversion from usize {} to i32", v);
        } else if ! (v <= ::std::i32::MAX as usize) {
            panic!("overflow on conversion from usize {} to i32", v);
        } else {
            v as i32
        }
    }
}

pub trait CheckedInto<T> {
    fn checked_into(self) -> T;
}

impl<T, U> CheckedInto<U> for T where U: CheckedFrom<T> {
    fn checked_into(self) -> U {
        CheckedFrom::checked_from(self)
    }
}

pub trait ToCStr {
    fn to_c_str(&self) -> CString;
}

impl<'a> ToCStr for &'a str {
    fn to_c_str(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

impl ToCStr for String {
    fn to_c_str(&self) -> CString {
        CString::new(self.clone()).unwrap()
    }
}

pub trait TryFrom<T> {
    fn try_from(T) -> Option<Self>;
}

pub trait TryInto<U> {
    fn try_into(self) -> Option<U>;
}

impl<T, U> TryInto<U> for T where U: TryFrom<T> {
    fn try_into(self) -> Option<U> {
        TryFrom::try_from(self)
    }
}
