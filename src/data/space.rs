use std::{cmp, vec, slice, ops};
use std::iter::{self, Extend, FromIterator, FusedIterator};
use parry3d::partitioning::IndexedData;


const CAPACITY: usize = 4;


#[derive(Clone, Debug)]
pub struct Space<T> {
    items: Vec<Entry<T>>,
    generation: u32,
    list_head: Option<u32>,
    len: usize,
}

#[derive(Clone, Debug)]
enum Entry<T> {
    Free {next_free: Option<u32>},
    Used {generation: u32, val: T}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Index {
    index: u32,
    generation: u32,
}

#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    len: usize,
    inner: vec::IntoIter<Entry<T>>,
}

#[derive(Clone, Debug)]

pub struct Iter<'a, T: 'a> {
    len: usize,
    inner: iter::Enumerate<slice::Iter<'a, Entry<T>>>,
}

impl<T> Space<T> {
    pub fn new() -> Space<T> {
        Space::with_capacity(CAPACITY)
    }

    pub fn with_capacity(x: usize) -> Space<T> {
        let x = cmp::max(x, 1);
        let mut space = Space {
            items: Vec::new(),
            generation: 0,
            list_head: None,
            len: 0,
        };

        space.reserve(x);
        space
    }

    pub fn reserve(&mut self, capacity: usize) {
        let start = self.items.len();
        let end = self.items.len() + capacity;
        let old_head = self.list_head;

        self.items.reserve_exact(capacity);
        self.items.extend((start..end)
            .map(|i| {
               if i == end - 1 {
                   Entry::Free {
                       next_free: old_head,
                   }
               } else { 
                   Entry::Free {
                       next_free: Some(i as u32 + 1),
                   }
               }
            }));
        
        self.list_head = Some(start as u32)
    }

    #[inline]
    fn try_alloc_next_index(&mut self) -> Option<Index> {
        match self.list_head {
            None => None,
            Some(i) => match self.items[i as usize] {
                Entry::Used {..} => panic!("corrupt free list"),
                Entry::Free { next_free } => {
                    self.list_head = next_free;
                    self.len += 1;

                    Some(Index {
                        index: i as u32,
                        generation: self.generation
                    })
                }
            },
        }
    }

    #[inline]
    pub fn try_insert(&mut self, val: T) -> Result<Index, T> {
        match self.try_alloc_next_index() {
            None => Err(val),
            Some(i) => {
                self.items[i.index as usize] = Entry::Used {
                    generation: self.generation,
                    val,
                };

                Ok(i)
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, val: T) -> Index {
        match self.try_insert(val) {
            Ok(i) => i,
            Err(val) => self.insert_slow_path(val),
        }
    }

    #[inline]
    fn insert_slow_path(&mut self, val: T) -> Index {
        let len = self.items.len();

        self.reserve(len);
        self.try_insert(val)
            .map_err(|_| ())
            .expect("error")
    }

    pub fn get(&self, i: Index) -> Option<&T> {
        match self.items.get(i.index as usize) {
            Some(Entry::Used { generation, val }) if *generation == i.generation => {
                Some(val)
            }
            _ => None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            len: self.len,
            inner: self.items.iter().enumerate(),
        }
    }
}

impl Index {
    pub fn from_raw_parts(index: u32, generation: u32) -> Self {
        Index { index, generation }
    }

    pub fn into_raw_parts(self) -> (u32, u32) {
        (self.index, self.generation)
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::from_raw_parts(u32::MAX, u32::MAX)
    }
}

impl IndexedData for Index {
    fn default() -> Self {
        Default::default()
    }

    fn index(&self) -> usize {
        self.into_raw_parts().0 as usize
    }
}

impl<T> IntoIterator for Space<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;


    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            len: self.len,
            inner: self.items.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(Entry::Free { .. }) => continue,
                Some(Entry::Used { val, .. }) => {
                    self.len -= 1;

                    return Some(val);
                }
                None => {
                    debug_assert_eq!(self.len, 0);

                    return None;
                }
            }
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<'a, T> IntoIterator for &'a Space<T> {
    type Item = (Index, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Index, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some((_, &Entry::Free { .. })) => continue,
                Some((
                    index,
                    &Entry::Used {
                        generation,
                        ref val,
                    },
                )) => {
                    self.len -= 1;
                    let idx = Index {
                        index: index as u32,
                        generation,
                    };

                    return Some((idx, val));
                }
                None => {
                    debug_assert_eq!(self.len, 0);

                    return None;
                }
            }
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<T> ops::Index<Index> for Space<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).expect("No element at index")
    }
}