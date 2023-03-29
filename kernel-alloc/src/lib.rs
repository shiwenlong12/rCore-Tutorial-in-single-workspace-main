//! 内存分配。

#![no_std]
//#![deny(warnings, missing_docs)]

extern crate alloc;

use alloc::alloc::handle_alloc_error;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};
use customizable_buddy::{BuddyAllocator, LinkedListBuddy, UsizeBuddy};

/// 初始化内存分配。
///
/// 参数 `base_address` 表示动态内存区域的起始位置。
#[inline]
pub fn init(base_address: usize) {
    unsafe {
        HEAP.init(
            core::mem::size_of::<usize>().trailing_zeros() as _,
            NonNull::new(base_address as *mut u8).unwrap(),
        )
    };
}

/// 将一个内存块托管到内存分配器。
///
/// # Safety
///
/// `region` 内存块的所有权将转移到分配器，
/// 因此需要调用者确保这个内存块与已经转移到分配器的内存块都不重叠，且未被其他对象引用。
/// 并且这个内存块必须位于初始化时传入的起始位置之后。
#[inline]
pub unsafe fn transfer(region: &'static mut [u8]) {
    let ptr = NonNull::new(region.as_mut_ptr()).unwrap();
    HEAP.transfer(ptr, region.len());
}

/// 堆分配器。/伙伴分配器
///
/// 最大容量：6 + 21 + 3 = 30 -> 1 GiB。
/// 不考虑并发使用，因此没有加锁。
static mut HEAP: BuddyAllocator<21, UsizeBuddy, LinkedListBuddy> = BuddyAllocator::new();

struct Global;

#[global_allocator]
static GLOBAL: Global = Global;

unsafe impl GlobalAlloc for Global {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Ok((ptr, _)) = HEAP.allocate_layout::<u8>(layout) {
            ptr.as_ptr()
        } else {
            handle_alloc_error(layout)
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.deallocate_layout(NonNull::new(ptr).unwrap(), layout)
    }
}

#[cfg(test)]
mod tests {
    
    use crate::{init,transfer};
    use crate::Global;
    use crate::Layout;
    use core::alloc::GlobalAlloc;
    // 物理内存容量 = 24 MiB。
    const MEMORY: usize = 24 << 20;

    // pub struct KernelLayout {
    //     text: usize,
    //     rodata: usize,
    //     data: usize,
    //     sbss: usize,
    //     ebss: usize,
    //     boot: usize,
    //     end: usize,
    // }
    
    // impl KernelLayout {
    //     /// 非零初始化，避免 bss。
    //     pub const INIT: Self = Self {
    //         text: usize::MAX,
    //         rodata: usize::MAX,
    //         data: usize::MAX,
    //         sbss: usize::MAX,
    //         ebss: usize::MAX,
    //         boot: usize::MAX,
    //         end: usize::MAX,
    //     };
    // }

    // const PAGE: Layout =
    //     unsafe { Layout::from_size_align_unchecked(2 << Sv39::PAGE_BITS, 1 << Sv39::PAGE_BITS) };

    use alloc::alloc::{alloc_zeroed, dealloc};
    #[test]
    fn test_alloc() {
        init(8000_1000);

        unsafe {
            let layout = Layout::new::<u16>();
            let ptr = alloc_zeroed(layout);

            assert_eq!(*(ptr as *mut u16), 0);

            dealloc(ptr, layout);
        }
        //let a = Alignment::new(1024).unwrap();
        //我们的需求是分配一块连续的、大小至少为 size 字节的虚拟内存，且对齐要求为 align

        // unsafe {
        //     transfer(core::slice::from_raw_parts_mut(
        //         layout.end() as _,
        //         MEMORY - layout.len(),
        //     ))
        // };

        // let layout = Layout {
        //     //size 表示要分配的字节数，
        //     size: 512,
        //     //align 则表示分配的虚拟地址的最小对齐要求，即分配的地址要求是 align 的倍数。
        //     //这里的 align 必须是2的幂次。
        //     align: 1024,
        // };
        // let global = Global{};

        // let a = Global::alloc(&global,layout);
    }
}
