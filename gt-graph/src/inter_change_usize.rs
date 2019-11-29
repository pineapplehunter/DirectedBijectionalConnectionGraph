pub trait InterChangeUsize {
    fn to_usize(&self) -> usize;
    fn from_usize(val: usize) -> Self;
}

impl InterChangeUsize for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(val: usize) -> Self {
        debug_assert!(
            val <= Self::max_value() as usize,
            "usize value is too big to fit in u8"
        );

        val as Self
    }
}

impl InterChangeUsize for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(val: usize) -> Self {
        debug_assert!(
            val <= Self::max_value() as usize,
            "usize value is too big to fit in u16"
        );

        val as Self
    }
}

impl InterChangeUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(val: usize) -> Self {
        debug_assert!(
            val <= Self::max_value() as usize,
            "usize value is too big to fit in u32"
        );
        val as Self
    }
}

impl InterChangeUsize for u64 {
    fn to_usize(&self) -> usize {
        debug_assert!(
            *self > usize::max_value() as Self,
            "u64 value is too big to fit in usize"
        );

        *self as usize
    }

    fn from_usize(val: usize) -> Self {
        val as Self
    }
}
