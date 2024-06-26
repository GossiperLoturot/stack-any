//! # stack-any
//!
//! A library that provides a type that owns same size type on the stack for type erasure.
//!
//! ## Usage
//!
//! ```
//! let stack_0 = stack_any::stack_any!(Vec<i32>, vec![0, 1, 2]);
//! let stack_1 = stack_any::stack_any!(Vec<char>, vec!['a', 'b', 'c']);
//! let mut stacks = [stack_0, stack_1];
//!
//! stacks[0].downcast_mut::<Vec<i32>>().push(3);
//! stacks[1].downcast_mut::<Vec<char>>().push('d');
//!
//! assert_eq!(stacks[0].downcast_ref::<Vec<i32>>(), &vec![0, 1, 2, 3]);
//! assert_eq!(stacks[1].downcast_ref::<Vec<char>>(), &vec!['a', 'b', 'c', 'd']);
//! ```

/// A convertible type that owns a stack allocation of `N` size.
pub struct StackAny<const N: usize> {
    bytes: [std::mem::MaybeUninit<u8>; N],
    size: usize,
}

impl<const N: usize> StackAny<N> {
    /// Allocates N-size memory on the stack and then places `value` into it.
    ///
    /// # Panics
    ///
    /// Panics if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::StackAny::<{ std::mem::size_of::<i32>() }>::new(5);
    /// ```
    #[inline]
    pub fn new<T>(value: T) -> Self
    where
        T: std::any::Any,
    {
        Self::try_new(value).expect("T size is not equal to N")
    }

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
        T: std::any::Any,
    {
        if N != std::mem::size_of::<T>() {
            return None;
        }

        let mut slf = Self {
            bytes: [std::mem::MaybeUninit::uninit(); N],
            size: std::mem::size_of::<T>(),
        };

        let src = &value as *const _ as *const _;
        let dst = slf.bytes.as_mut_ptr();
        unsafe { std::ptr::copy_nonoverlapping(src, dst, N) };
        std::mem::forget(value);

        Some(slf)
    }

    /// Attempt to return reference to the inner value as a concrete type.
    ///
    /// # Panics
    ///
    /// Panics if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast_ref::<i32>(), &5);
    /// ```
    #[inline]
    pub fn downcast_ref<T>(&self) -> &T
    where
        T: std::any::Any,
    {
        self.try_downcast_ref().expect("T size is not equal to N")
    }

    /// Attempt to return reference to the inner value as a concrete type.
    /// Returns None if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.try_downcast_ref::<i32>(), Some(&5));
    /// assert_eq!(five.try_downcast_ref::<i64>(), None);
    /// ```
    pub fn try_downcast_ref<T>(&self) -> Option<&T>
    where
        T: std::any::Any,
    {
        if self.size != std::mem::size_of::<T>() {
            return None;
        }
        let ptr = self.bytes.as_ptr();
        Some(unsafe { &*(ptr as *const T) })
    }

    /// Attempt to return mutable reference to the inner value as a concrete type.
    ///
    /// # Panics
    ///
    /// Panics if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast_mut::<i32>(), &mut 5);
    /// ```
    pub fn downcast_mut<T>(&mut self) -> &mut T
    where
        T: std::any::Any,
    {
        self.try_downcast_mut().expect("T size is not equal to N")
    }

    /// Attempt to return mutable reference to the inner value as a concrete type.
    /// Returns None if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.try_downcast_mut::<i32>(), Some(&mut 5));
    /// assert_eq!(five.try_downcast_mut::<i64>(), None);
    /// ```
    pub fn try_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: std::any::Any,
    {
        if self.size != std::mem::size_of::<T>() {
            return None;
        }
        let ptr = self.bytes.as_mut_ptr();
        Some(unsafe { &mut *(ptr as *mut T) })
    }

    /// Attempt to downcast the stack to a concrete type.
    ///
    /// # Panics
    ///
    /// Panics if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.downcast::<i32>(), 5);
    /// ```
    pub fn downcast<T>(self) -> T
    where
        T: std::any::Any,
    {
        self.try_downcast().expect("T size is not equal to N")
    }

    /// Attempt to downcast the stack to a concrete type.
    /// Returns None if `T` size is larger than N.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = stack_any::stack_any!(i32, 5);
    /// assert_eq!(five.try_downcast::<i32>(), Some(5));
    /// ```
    pub fn try_downcast<T>(self) -> Option<T>
    where
        T: std::any::Any,
    {
        if self.size != std::mem::size_of::<T>() {
            return None;
        }
        let ptr = self.bytes.as_ptr();
        Some(unsafe { std::ptr::read(ptr as *const T) })
    }
}

/// Allocates memory on the stack and then places value based on given type and value.
///
/// # Panics
///
/// Panics if `ty` type size is larger than `expr` type size in `stack_any!(ty, expr)`.
///
/// # Examples
///
/// ```
/// let five = stack_any::stack_any!(i32, 5);
/// ```
#[macro_export]
macro_rules! stack_any {
    ($type:ty, $init:expr) => {
        $crate::StackAny::<{ std::mem::size_of::<$type>() }>::new::<$type>($init)
    };
}
