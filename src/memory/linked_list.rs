use core::cell::UnsafeCell;
use core::ptr::{null_mut, write};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::boards::HEAP_SIZE;


unsafe extern "C" {
    static _heap_start: u8;
}

pub fn init_heap() {
    let begin = core::ptr::addr_of!(_heap_start) as usize;
    let size = HEAP_SIZE;

    let begin_list_node : *mut ListNode = begin as *mut ListNode;
    unsafe {
        write(begin_list_node, ListNode::new(size, null_mut()));
    }

    ALLOCATOR.lock(|intern_allocator| {
        intern_allocator.head = begin_list_node;
    });
}

struct ListNode{
    size: usize,
    next: *mut ListNode,
}

pub struct Allocator{
    head: *mut ListNode,
}

impl ListNode {
    fn new(size: usize, list_node: *mut ListNode) -> ListNode {ListNode{size : size, next : list_node}}
}

impl Allocator{
    const fn empty() -> Allocator{return Allocator{head:null_mut()};}
    fn new(list_node: *mut ListNode) -> Allocator {
        Allocator{head : list_node}
    }
    unsafe fn init(&mut self, head : *mut ListNode){
        self.head = head;
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

unsafe impl core::alloc::GlobalAlloc for Locked<Allocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock(|intern_allocator| {
            let mut current = intern_allocator.head;
            let mut prev: *mut ListNode = core::ptr::null_mut();
            while !current.is_null() {
                let node = unsafe{ &mut *current};
                let begin = (current as usize + layout.align() - 1) & !(layout.align() - 1);
                let end = begin + layout.size();
                if end <= current as usize + node.size {
                    if prev == null_mut() {
                        intern_allocator.head = node.next;
                    }
                    else {
                        unsafe{(*prev).next = node.next;}
                    }
                    let remainder = (current as usize + node.size) - end;
                    if remainder >= size_of::<ListNode>() {
                        unsafe{ write (end as * mut ListNode, ListNode::new(remainder, intern_allocator.head));}
                        intern_allocator.head = end as *mut ListNode;
                    }
                    return begin as *mut u8;
                }
                else {
                    prev = current;
                    current = node.next;
                }
            }
            null_mut()
        })
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let ptr_list_node = ptr as *mut ListNode;

        self.lock(|intern_allocator| {
            unsafe {
                write(ptr_list_node, ListNode::new(layout.size(), intern_allocator.head));
                intern_allocator.head = ptr_list_node;
            }
        })
    }
}

#[global_allocator]
pub static ALLOCATOR: Locked<Allocator> = Locked::new(Allocator::empty());