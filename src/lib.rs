struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.slice.split_first() {
            Some((elem, rest)) => {
                self.slice = rest;
                Some(elem)
            }
            None => None,
        }
    }
}

struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = std::mem::replace(&mut self.slice, &mut []);

        match slice.split_first_mut() {
            Some((elem, rest)) => {
                self.slice = rest;
                Some(elem)
            }
            None => None,
        }
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
        assert_eq!(*collection.get(0).unwrap(), 1, "cant mutate in place");

        // empty
        let empty_wrapper = MyMutableIterator::<usize> {
            slice: &mut Vec::new()[..],
        };
        for _ in empty_wrapper {
            assert_eq!(0, 1, "should never run over empty wrapper");
        }
    }
}
