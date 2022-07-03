use std::cmp;
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