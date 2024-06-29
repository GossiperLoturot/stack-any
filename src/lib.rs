//! # stack-any
//!
//! A library that provides a type that owns same size type on the stack for type erasure.
//!
//! ## Usage
//!
//! ```
//! let mut stacks = [
//!     stack_any::stack_any!(Vec<i32>, vec![]),
//!     stack_any::stack_any!(Vec<char>, vec![]),
//! ];
//!
//! stacks[0].downcast_mut::<Vec<i32>>().unwrap().push(5);
//! stacks[1].downcast_mut::<Vec<char>>().unwrap().push('x');
//!
//! assert_eq!(stacks[0].downcast_ref(), Some(&vec![5]));
//! assert_eq!(stacks[1].downcast_ref(), Some(&vec!['x']));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

/// A convertible type that owns a stack allocation of `N` size.
#[derive(Debug)]
pub struct StackAny<const N: usize> {
    type_id: core::any::TypeId,
    bytes: [core::mem::MaybeUninit<u8>; N],
    drop_fn: fn(*mut std::mem::MaybeUninit<u8>) -> (),
}

impl<const N: usize> StackAny<N> {
    /// Allocates N-size memory on the stack and then places `value` into it.
    /// Returns None if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::StackAny::<{ std::mem::size_of::<i32>() }>::try_new(5);
    /// ```
    pub fn try_new<T>(value: T) -> Option<Self>
    where
        T: core::any::Any,
    {
        let type_id = core::any::TypeId::of::<T>();
        let size = core::mem::size_of::<T>();

        if N < size {
            return None;
        }

        let mut bytes = [core::mem::MaybeUninit::uninit(); N];

        let src = &value as *const _ as *const _;
        let dst = bytes.as_mut_ptr();
        unsafe { core::ptr::copy_nonoverlapping(src, dst, size) };

        let drop_fn = |ptr| unsafe { core::ptr::drop_in_place(ptr as *mut T) };
        core::mem::forget(value);

        Some(Self {
            type_id,
            bytes,
            drop_fn,
        })
    }

    /// Attempt to return reference to the inner value as a concrete type.
    /// Returns None if `T` is not equal to contained value type.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast_ref::<i32>(), Some(&5));
    /// assert_eq!(five.downcast_ref::<i64>(), None);
    /// ```
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: core::any::Any,
    {
        if core::any::TypeId::of::<T>() != self.type_id {
            return None;
        }

        let ptr = self.bytes.as_ptr();
        Some(unsafe { &*(ptr as *const T) })
    }

    /// Attempt to return mutable reference to the inner value as a concrete type.
    /// Returns None if `T` is not equal to contained value type.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast_mut::<i32>(), Some(&mut 5));
    /// assert_eq!(five.downcast_mut::<i64>(), None);
    /// ```
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: core::any::Any,
    {
        if core::any::TypeId::of::<T>() != self.type_id {
            return None;
        }

        let ptr = self.bytes.as_mut_ptr();
        Some(unsafe { &mut *(ptr as *mut T) })
    }

    /// Attempt to downcast the stack to a concrete type.
    /// Returns None if `T` is not equal to contained value type.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast::<i32>(), Some(5));
    /// ```
    pub fn downcast<T>(mut self) -> Option<T>
    where
        T: core::any::Any,
    {
        if core::any::TypeId::of::<T>() != self.type_id {
            return None;
        }

        self.drop_fn = |_| {};

        let ptr = self.bytes.as_ptr();
        Some(unsafe { core::ptr::read(ptr as *const T) })
    }
}

impl<const N: usize> Drop for StackAny<N> {
    fn drop(&mut self) {
        (self.drop_fn)(self.bytes.as_mut_ptr());
    }
}

/// Allocates memory on the stack and then places value based on given type and value.
///
/// # Examples
///
/// ```
/// let five = stack_any::stack_any!(i32, 5);
/// ```
#[macro_export]
macro_rules! stack_any {
    ($type:ty, $init:expr) => {
        $crate::StackAny::<{ std::mem::size_of::<$type>() }>::try_new::<$type>($init).unwrap()
    };
}
