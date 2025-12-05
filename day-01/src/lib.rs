#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Wrapping<const N: usize> ( usize );

impl<const N: usize> Wrapping<N> {
    pub const fn new(n: usize) -> Option<Self> {
        if n >= N {
            None
        } else {
            Some(Wrapping(n))
        }
    }

    pub const unsafe fn new_unchecked(n: usize) -> Self {
        Wrapping(n)
    }

    pub fn wrap_counting_add(self, other: usize) -> (Self, usize) {
        let wrap_count = (self.0 + other) / N;
        (self + other, wrap_count)
    }

    pub fn wrap_counting_sub(self, other: usize) -> (Self, usize) {
        let mut wrap_count = other / N;
        if self.0 < other % N {
            wrap_count += 1
        }
        (self - other, wrap_count)
    }
}

impl<const N: usize> std::ops::Add<usize> for Wrapping<N> {
    type Output = Self;
    fn add(self, other: usize) -> Self {
        Wrapping((self.0 + other) % N)
    }
}

impl<const N: usize> std::ops::AddAssign<usize> for Wrapping<N> {
    fn add_assign(&mut self, other: usize) {
        self.0 = (self.0 + other) % N;
    }
}

impl<const N: usize> std::ops::Sub<usize> for Wrapping<N> {
    type Output = Self;
    fn sub(self, other: usize) -> Self {
        Wrapping((self.0 + N - other % N) % N)
    }
}

impl<const N: usize> std::ops::SubAssign<usize> for Wrapping<N> {
    fn sub_assign(&mut self, other: usize) {
        self.0 = (self.0 + N - other % N) % N;
    }
}

