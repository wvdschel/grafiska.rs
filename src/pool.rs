// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::VecDeque;
use {ResourceHandle, ResourceState};

pub const SLOT_SHIFT: u32 = 16;
pub const SLOT_MASK: u32 = (1 << SLOT_SHIFT) - 1;
pub const MAX_POOL_SIZE: usize = 1 << SLOT_SHIFT;
pub const DEFAULT_PASS_POOL_SIZE: u32 = 16;

pub struct Pool<R: ResourceHandle + Sized> {
    resources: Vec<Option<R::Resource>>,
    free_queue: VecDeque<u32>,
    unique_counter: u32,
}

impl<R: ResourceHandle + Sized> Pool<R> {
    pub fn new(num: usize) -> Self {
        assert!(num < MAX_POOL_SIZE);
        assert!(num > 0);

        // 0 is an reserved for 'invalid id', so bump size with one.
        let mut resources = Vec::<Option<R::Resource>>::with_capacity(num + 1);
        let mut free_queue = VecDeque::with_capacity(num + 1);
        for i in 1..num + 2 {
            resources.push(None);
            free_queue.push_back(i as u32);
        }
        Pool {
            resources: resources,
            free_queue: free_queue,
            unique_counter: 0,
        }
    }

    pub fn alloc(&mut self) -> Option<R> {
        self.free_queue.pop_front().map(R::with)
    }

    pub fn destroy(&mut self, handle: R, backend: &mut ::backend::Backend) {
        // Make sure that this isn't a double free.
        debug_assert_eq!(self.free_queue.contains(&handle.id()), false);
        if let Some(ref mut r) = self.resources[handle.id() as usize] {
            // backend.destroy(r);
            self.free_queue.push_back(handle.id());
        }
    }

    pub fn lookup(&self, handle: &R) -> Option<&R::Resource> {
        self.resources[handle.id() as usize].as_ref()
    }

    pub fn lookup_mut(&mut self, handle: &R) -> Option<&mut R::Resource> {
        self.resources[handle.id() as usize].as_mut()
    }
}

#[derive(Debug, Default)]
pub struct Slot {
    pub id: u32,
    pub state: ResourceState,
}
