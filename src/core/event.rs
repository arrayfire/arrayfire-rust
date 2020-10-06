use super::defines::AfError;
use super::error::HANDLE_ERROR;
use super::util::af_event;

use libc::c_int;
use std::default::Default;

extern "C" {
    fn af_create_event(out: *mut af_event) -> c_int;
    fn af_delete_event(out: af_event) -> c_int;
    fn af_mark_event(out: af_event) -> c_int;
    fn af_enqueue_wait_event(out: af_event) -> c_int;
    fn af_block_event(out: af_event) -> c_int;
}

/// RAII construct to manage ArrayFire events
///
/// ## Sharing Across Threads
///
/// While sharing an Event with other threads, just move it across threads.
pub struct Event {
    event_handle: af_event,
}

unsafe impl Send for Event {}
// No borrowed references are to be shared for Events, hence no sync trait

impl Default for Event {
    fn default() -> Self {
        let mut temp: af_event = std::ptr::null_mut();
        unsafe {
            let err_val = af_create_event(&mut temp as *mut af_event);
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
            let err_val = af_mark_event(self.event_handle as af_event);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Enqueues the event and all enqueued events on the active queue
    ///
    /// All operations enqueued after a call to enqueue will not be executed
    /// until operations on the queue when mark was called are complete
    pub fn enqueue_wait(&self) {
        unsafe {
            let err_val = af_enqueue_wait_event(self.event_handle as af_event);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Blocks the calling thread on events until all events on the computation
    /// stream before mark was called are complete
    pub fn block(&self) {
        unsafe {
            let err_val = af_block_event(self.event_handle as af_event);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_delete_event(self.event_handle as af_event);
            match ret_val {
                0 => (),
                _ => panic!("Failed to delete event resources: {}", ret_val),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::arith::pow;
    use super::super::device::{info, set_device};
    use super::super::event::Event;
    use crate::{af_print, randu};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn event_block() {
        // This code example will try to compute the following expression
        // using data-graph approach using threads, evens for illustration.
        //
        // (a * (b + c))^(d - 2)
        //
        // ANCHOR: event_block

        // Set active GPU/device on main thread on which
        // subsequent Array objects are created
        set_device(0);
        info();

        let a = randu!(10, 10);
        let b = randu!(10, 10);
        let c = randu!(10, 10);
        let d = randu!(10, 10);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            set_device(0);

            let add_event = Event::default();

            let add_res = b + c;

            add_event.mark();
            tx.send((add_res, add_event)).unwrap();

            thread::sleep(std::time::Duration::new(10, 0));

            let sub_event = Event::default();

            let sub_res = d - 2;

            sub_event.mark();
            tx.send((sub_res, sub_event)).unwrap();
        });

        let (add_res, add_event) = rx.recv().unwrap();

        println!("Got first message, waiting for addition result ...");
        thread::sleep(std::time::Duration::new(5, 0));
        // Perhaps, do some other tasks
        add_event.block();

        println!("Got addition result, now scaling it ... ");
        let scaled = a * add_res;

        let (sub_res, sub_event) = rx.recv().unwrap();

        println!("Got message, waiting for subtraction result ...");
        thread::sleep(std::time::Duration::new(5, 0));
        // Perhaps, do some other tasks
        sub_event.block();

        let fin_res = pow(&scaled, &sub_res, false);

        af_print!(
            "Final result of the expression: ((a * (b + c))^(d - 2))",
            &fin_res
        );

        // ANCHOR_END: event_block
    }
}
