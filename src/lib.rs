struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (elem, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(elem)
    }
}

struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // lifetime hack
        let slice = std::mem::replace(&mut self.slice, &mut []);

        let (elem, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(elem)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MyIterWrapper, MyMutableIterator};

    #[test]
    fn wrapper_iterate() {
        // can iterate
        let collection = vec![0, 1, 2, 3, 4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };
        for (i, elem) in wrapper.enumerate() {
            assert_eq!(i, *elem);
        }
        // empty
        let empty_wrapper = MyIterWrapper::<usize> {
            slice: &Vec::new()[..],
        };
        for _ in empty_wrapper {
            assert_eq!(0, 1, "should never run over empty wrapper");
        }
    }

    #[test]
    fn mutable_wrapper_iterate() {
        // can iterate
        let collection = &mut vec![0, 1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for (i, elem) in wrapper.enumerate() {
            assert_eq!(i, *elem);
        }

        // can mutate
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for elem in wrapper {
            *elem = *elem + 1;
        }
        assert_eq!(*collection.get(0).unwrap(), 1, "can mutate in place");

        // empty
        let empty_wrapper = MyMutableIterator::<usize> {
            slice: &mut Vec::new()[..],
        };
        for _ in empty_wrapper {
            assert_eq!(0, 1, "should never run over empty wrapper");
        }
    }
}
