use std::collections::VecDeque;
use ::ResourceState;

pub const SLOT_SHIFT : u32 = 16;
pub const SLOT_MASK : u32 = (1 << SLOT_SHIFT) - 1;
pub const MAX_POOL_SIZE : usize = 1 << SLOT_SHIFT;
pub const DEFAULT_PASS_POOL_SIZE : u32 = 16;

pub struct Pool<T: Sized> {
    resources: Vec<Option<T>>,
    free_queue: VecDeque<u32>,
    unique_counter: u32,
}

impl<T: Sized> Pool<T> {
    pub fn new(num: usize) -> Self {
        assert!(num < MAX_POOL_SIZE);
        assert!(num > 0);

        // 0 is an reserved for 'invalid id', so bump size with one.
        let mut resources = Vec::<Option<T>>::with_capacity(num + 1);
        let mut free_queue = VecDeque::with_capacity(num + 1);
        for i in 1..num+2 {
            resources.push(None);
            free_queue.push_back(i as u32);
        }
        Pool {
            resources: resources,
            free_queue: free_queue,
            unique_counter: 0,
        }   
    }
}

#[derive(Debug)]
pub struct Slot {
    pub id: u32,
    pub state: ResourceState,
}

impl Default for Slot {
    fn default() -> Self {
        Slot {
            id: 0,
            state: ResourceState::default(),
        }
    }
}
