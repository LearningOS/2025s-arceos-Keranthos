mod hashmap;

extern crate alloc;

pub use alloc::collections::{
    binary_heap::BinaryHeap,
    btree_map::BTreeMap,
    btree_set::BTreeSet,
    linked_list::LinkedList,
    vec_deque::VecDeque,
};

#[doc(hidden)]
pub use self::hashmap::HashMap;