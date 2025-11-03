#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    value: usize,
}

pub const START: usize = 0;

impl Default for Time {
    fn default() -> Self {
        Self::start()
    }
}

impl Time {
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn start() -> Self {
        Self::START
    }

    #[must_use]
    pub const fn increment(&mut self) -> Self {
        let copy = *self;

        self.increment_in_place();

        copy
    }

    pub const fn increment_in_place(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    #[must_use]
    pub const fn get(self) -> usize {
        self.value
    }

    pub const START: Self = Self::new(START);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timed<T> {
    pub value: T,
    pub time: Time,
}

impl<T: Default> Default for Timed<T> {
    fn default() -> Self {
        Self::start(T::default())
    }
}

impl<T> Timed<T> {
    pub const fn new(value: T, time: Time) -> Self {
        Self { value, time }
    }

    pub const fn start(value: T) -> Self {
        Self::new(value, Time::start())
    }
}
