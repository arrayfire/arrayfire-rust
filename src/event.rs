extern crate libc;

use self::libc::c_int;
use crate::defines::AfError;
use crate::error::HANDLE_ERROR;
use crate::util::{AfEvent, MutAfEvent};

use std::default::Default;

#[allow(dead_code)]
extern "C" {
    fn af_create_event(out: MutAfEvent) -> c_int;
    fn af_delete_event(out: AfEvent) -> c_int;
    fn af_mark_event(out: AfEvent) -> c_int;
    fn af_enqueue_wait_event(out: AfEvent) -> c_int;
    fn af_block_event(out: AfEvent) -> c_int;
}

/// RAII construct to manage ArrayFire events
pub struct Event {
    event_handle: i64,
}

impl Default for Event {
    fn default() -> Self {
        let mut temp: i64 = 0;
        unsafe {
            let err_val = af_create_event(&mut temp as MutAfEvent);
            HANDLE_ERROR(AfError::from(err_val));
        }
        Self { event_handle: temp }
    }
}

impl Event {
    /// Marks the event on the active computation queue.
    ///
    /// If the event is enqueued/waited on later, any operations that are currently
    /// enqueued on the event queue will be completed before any events that are
    /// enqueued after the call to enqueue
    pub fn mark(&self) {
        unsafe {
            let err_val = af_mark_event(self.event_handle as AfEvent);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Enqueues the event and all enqueued events on the active queue
    ///
    /// All operations enqueued after a call to enqueue will not be executed
    /// until operations on the queue when mark was called are complete
    pub fn enqueue_wait(&self) {
        unsafe {
            let err_val = af_enqueue_wait_event(self.event_handle as AfEvent);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Blocks the calling thread on events until all events on the computation
    /// stream before mark was called are complete
    pub fn block(&self) {
        unsafe {
            let err_val = af_block_event(self.event_handle as AfEvent);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_delete_event(self.event_handle as AfEvent);
            match ret_val {
                0 => (),
                _ => panic!("Failed to delete event resources: {}", ret_val),
            }
        }
    }
}
