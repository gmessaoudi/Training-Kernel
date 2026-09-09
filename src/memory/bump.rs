use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use crate::boards::HEAP_SIZE;

unsafe extern "C" {
    static _heap_start: u8;
}

pub fn init_heap(){
    let begin = core::ptr::addr_of!(_heap_start) as usize;
    ALLOCATOR.lock(|intern_allocator| {
        unsafe {
            intern_allocator.init(begin, HEAP_SIZE);
        }
    });
}

pub struct Allocator{
    next : usize,
    end : usize,
}

impl Allocator{
    const fn empty() -> Allocator{return Allocator{next:0,end:0};}
    pub unsafe fn init(&mut self, begin: usize, size: usize){
        self.next = begin;
        self.end = begin + size;
    }
}

unsafe impl core::alloc::GlobalAlloc for Locked<Allocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock(|intern_allocator| {
            let begin : usize = (intern_allocator.next + layout.align()-1) & !(layout.align()-1);
            let end : usize = begin + layout.size();
            if end > intern_allocator.end {
                return core::ptr::null_mut();
            }
            intern_allocator.next = end;
            return begin as *mut u8;
        })
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {

    }
}

pub struct Locked<A> {
    inner: UnsafeCell<A>,
    lock: AtomicBool,
}

unsafe impl<A> Sync for Locked<A> {}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: UnsafeCell::new(inner),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut A) -> R,
    {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }

        let result = unsafe { f(&mut *self.inner.get()) };

        self.lock.store(false, Ordering::Release);

        return result;
    }
}

//#[global_allocator]
pub static ALLOCATOR: Locked<Allocator> = Locked::new(Allocator::empty());