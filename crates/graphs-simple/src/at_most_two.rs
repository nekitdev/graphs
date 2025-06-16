pub enum AtMostTwoOutput<T> {
    None,
    One(T),
    Two(T, T),
}

mod sealed {
    pub trait Sealed {}
}

pub trait AtMostTwo: sealed::Sealed {
    type Item;

    fn at_most_two(
        &mut self,
        one_index: usize,
        two_index: usize,
    ) -> AtMostTwoOutput<&mut Self::Item>;
}

impl<T> sealed::Sealed for [T] {}

impl<T> AtMostTwo for [T] {
    type Item = T;

    fn at_most_two(
        &mut self,
        one_index: usize,
        two_index: usize,
    ) -> AtMostTwoOutput<&mut Self::Item> {
        let index = one_index.max(two_index);

        if index >= self.len() {
            AtMostTwoOutput::None
        } else if one_index == two_index {
            // SAFETY: already checked that the index is in bounds
            let only = unsafe { self.get_unchecked_mut(index) };

            AtMostTwoOutput::One(only)
        } else {
            // SAFETY: already checked that indexes are in bounds and distinct
            let [one, two] = unsafe { self.get_disjoint_unchecked_mut([one_index, two_index]) };

            AtMostTwoOutput::Two(one, two)
        }
    }
}
