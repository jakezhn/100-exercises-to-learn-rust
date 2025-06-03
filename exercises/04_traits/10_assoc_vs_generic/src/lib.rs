// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

pub trait Power {
    type Output;

    // Separate methods with distinct names
    fn power_u16(self, n: u16) -> Self::Output;
    fn power_u32(self, n: u32) -> Self::Output;
    fn power_ref_u32(self, n: &u32) -> Self::Output;
    
    // Generic method that dispatches to the appropriate implementation
    fn power<T>(self, n: T) -> Self::Output
    where
        Self: Sized,
        T: PowerArg<Self>;
}

// Helper trait to handle dispatching
pub trait PowerArg<P: Power> {
    fn apply_power(self, base: P) -> P::Output;
}

// Implement the PowerArg trait for each argument type
impl<P: Power> PowerArg<P> for u16 {
    fn apply_power(self, base: P) -> P::Output {
        base.power_u16(self)
    }
}

impl<P: Power> PowerArg<P> for u32 {
    fn apply_power(self, base: P) -> P::Output {
        base.power_u32(self)
    }
}

impl<P: Power> PowerArg<P> for &u32 {
    fn apply_power(self, base: P) -> P::Output {
        base.power_ref_u32(self)
    }
}

impl Power for u32 {
    type Output = u32;

    fn power_u16(self, n: u16) -> Self::Output {
        self.pow(n as u32)
    }

    fn power_u32(self, n: u32) -> Self::Output {
        self.pow(n)
    }

    fn power_ref_u32(self, n: &u32) -> Self::Output {
        self.pow(*n)
    }
    
    fn power<T>(self, n: T) -> Self::Output
    where
        Self: Sized,
        T: PowerArg<Self>
    {
        n.apply_power(self)
    }
}

// We can implement for other types too if needed
impl Power for u64 {
    type Output = u64;

    fn power_u16(self, n: u16) -> Self::Output {
        self.pow(n as u32)
    }

    fn power_u32(self, n: u32) -> Self::Output {
        self.pow(n)
    }

    fn power_ref_u32(self, n: &u32) -> Self::Output {
        self.pow(*n)
    }
    
    fn power<T>(self, n: T) -> Self::Output
    where
        Self: Sized,
        T: PowerArg<Self>
    {
        n.apply_power(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
