#[derive(Clone, Debug)]
pub struct Counter {
    value: u32,
}

impl Default for Counter {
    fn default() -> Counter {
        Counter { value: 0 }
    }
}

impl Counter {
    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

/// This function allocates the counter
#[no_mangle]
pub extern "C" fn alloc() -> *mut Counter {
    let boxed = Box::new(Counter::default());
    Box::into_raw(boxed)
}

/// # Safety
///
/// This function sets the counter value to zero
/// This function should be called only after allocating a counter
#[no_mangle]
pub unsafe extern "C" fn clear(counter: *mut Counter) {
    let counter = &mut *counter;
    counter.clear();
}

/// # Safety
///
/// This function increments the counter by one
/// This function should be called only after allocating a counter
#[no_mangle]
pub unsafe extern "C" fn inc(counter: *mut Counter) {
    let counter = &mut *counter;
    counter.inc();
}

/// # Safety
///
/// This function returns the current value of the counter
/// This function should be called only after allocating a counter
#[no_mangle]
pub unsafe extern "C" fn value(counter: *mut Counter) -> u32 {
    let counter = &*counter;
    counter.value()
}

/// # Safety
///
/// This function deallocates the counter
/// This function should be called only after allocating a counter
#[no_mangle]
pub unsafe extern "C" fn dealloc(counter: *mut Counter) {
    Box::from_raw(counter);
}
