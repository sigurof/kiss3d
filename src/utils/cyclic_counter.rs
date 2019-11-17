pub struct CyclicCounter<T>
    where
        T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + Copy
        + From<u16>,
{
    max: T,
    cur: T,
}

impl<T> CyclicCounter<T>
    where
        T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Rem<Output = T>
        + Copy
        + From<u16>,
{
    pub fn exclusive_max(max: T) -> CyclicCounter<T> {
        CyclicCounter {
            max,
            cur: T::from(0),
        }
    }

    pub fn inclusive_max(max: T) -> CyclicCounter<T> {
        CyclicCounter {
            max: max + T::from(1),
            cur: T::from(0),
        }
    }

    pub fn increment_one(&mut self) {
        self.cur = (self.cur + T::from(1)) % self.max;
    }

    pub fn increment_by(&mut self, by: T) {
        self.cur = (self.cur + by) % self.max;
    }

    pub fn current(&self) -> T {
        self.cur
    }

    pub fn peek_next(&self) -> T {
        (self.cur + T::from(1)) % self.max
    }

    pub fn peek_last(&self) -> T {
        (self.cur + self.max - T::from(1)) % self.max
    }
}
