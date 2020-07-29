use libloading::os::unix::Symbol as RawSymbol;
use libloading::{Library, Symbol};
use std::os::raw::{c_int, c_void};
use std::path::Path;

type Handle = *mut c_void;
type Alloc = fn() -> Handle;
type Clear = fn(Handle) -> ();
type Inc = fn(Handle) -> ();
type Value = fn(Handle) -> c_int;
type Dealloc = fn(Handle) -> ();

#[derive(Clone, Debug)]
struct Table {
    alloc: RawSymbol<Alloc>,
    clear: RawSymbol<Clear>,
    inc: RawSymbol<Inc>,
    value: RawSymbol<Value>,
    dealloc: RawSymbol<Dealloc>,
}

impl Table {
    pub unsafe fn new(lib: &Library) -> Table {
        let alloc: Symbol<Alloc> = lib.get(b"alloc").expect("Error: could not find alloc");
        let clear: Symbol<Clear> = lib.get(b"clear").expect("Error: could not find clear");
        let inc: Symbol<Inc> = lib.get(b"inc").expect("Error: could not find inc");
        let value: Symbol<Value> = lib.get(b"value").expect("Error: could not find value");
        let dealloc: Symbol<Dealloc> = lib.get(b"dealloc").expect("Error: could not find dealloc");
        Table {
            alloc: alloc.into_raw(),
            clear: clear.into_raw(),
            inc: inc.into_raw(),
            value: value.into_raw(),
            dealloc: dealloc.into_raw(),
        }
    }
}

#[derive(Debug)]
pub struct Counter {
    lib: Library,
    table: Table,
    handle: Handle,
}

impl Counter {
    pub fn new(lib: &Path) -> Counter {
        let lib = Library::new(lib).expect("Error: loading library");
        unsafe {
            let table = Table::new(&lib);
            let handle = (table.alloc)();
            Counter { lib, table, handle }
        }
    }

    pub fn clear(&self) {
        (self.table.clear)(self.handle);
    }

    pub fn inc(&self) {
        (self.table.inc)(self.handle);
    }

    pub fn value(&self) -> i32 {
        (self.table.value)(self.handle)
    }

    pub fn dealloc(&self) {
        (self.table.inc)(self.handle);
    }
}

fn main() {
    let lib = Path::new("cpp/lib.so");
    let counter = Counter::new(&lib);
    println!("value after init:{}", counter.value());
    counter.inc();
    counter.inc();
    counter.inc();
    println!("value after some incs:{}", counter.value());
    counter.clear();
    println!("value after clear:{}", counter.value());
}
