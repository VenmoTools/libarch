pub use page::RecursivePageTable;

use crate::arch::intel::instructions::page_table::flush;
use crate::arch::intel::x64::address::{PhysAddr, PhysicalAddress, VirtAddr};
use crate::arch::intel::x64::paging::{Frame, FrameAllocator, Page, Page1GB, Page2MB, Page4KB, PageSize};
use crate::arch::intel::x64::paging::flags::PageTableFlags;
use crate::arch::intel::x64::paging::result::{FlagUpdateError, MapToError, TranslateError, TranslationResult, UnmapError};

// mod map_pt;
// mod pt_offset;
// mod recursive_table;
mod page;

#[derive(Debug)]
#[must_use = "Page Table changes must be flushed or ignored."]
pub struct MapperFlush<S: PageSize>(Page<S>);

impl<S: PageSize> MapperFlush<S> {
    pub fn new(page: Page<S>) -> Self {
        Self(page)
    }

    pub fn flush(self) {
        unsafe {
            flush(self.0.start_address());
        }
    }

    pub fn ignore(self) {}
}

pub trait Mapper<S: PageSize> {
    /// 在页表中创建一个新的映射。
    /// 此函数需要其他物理帧才能创建新的页表。
    /// 帧的分配由`allocator`参数完成
    unsafe fn map_to<A>(&mut self, page: Page<S>, frame: Frame<S>, flags: PageTableFlags, allocator: &mut A)
                        -> Result<MapperFlush<S>, MapToError<S>>
        where A: FrameAllocator<Page4KB>, Self: Sized;

    /// 从页表中解除映射关系，并返回被解除关系的frame。
    /// frame没有被释放
    fn unmap(&mut self, page: Page<S>) -> Result<(Frame<S>, MapperFlush<S>), UnmapError>;

    /// 更新现有映射的flags。
    unsafe fn update_flags(&mut self, page: Page<S>, flags: PageTableFlags) -> Result<MapperFlush<S>, FlagUpdateError>;

    /// 返回给定的页面与之映射的物理帧
    fn translate_page(&mut self, page: Page<S>) -> Result<Frame<S>, TranslateError>;

    /// 将给定的frame映射到相同虚拟地址的页面
    /// 此函数假定页面已映射到大小为`S`的frame，否则会返回错误。
    unsafe fn identity_map<A>(&mut self, frame: Frame<S>, flags: PageTableFlags, allocator: &mut A)
                              -> Result<MapperFlush<S>, MapToError<S>>
        where A: FrameAllocator<Page4KB>, Self: Sized, S: PageSize, Self: Mapper<S> {
        let page = Page::include_address(VirtAddr::new(frame.start_address().as_u64()));
        self.map_to(page, frame, flags, allocator)
    }
}

pub trait MapAllSize: Mapper<Page4KB> + Mapper<Page1GB> + Mapper<Page2MB> {
    /// 返回给定虚拟地址所映射的帧以及对应的帧内的偏移量。
    /// 如果给定的是有效虚拟地址，则返回映射的帧和该帧内的偏移量。 否则，将返回错误值。
    /// 此功能适用于各种种类的较大页面。
    fn translate(&self, addr: VirtAddr) -> TranslationResult;
    /// 将给定的虚拟地址转换为它映射到的物理地址。
    /// 如果给定地址没有有效的映射，则返回 None。
    fn translate_addr(&self, addr: VirtAddr) -> Option<PhysAddr> {
        match self.translate(addr) {
            TranslationResult::Frame4KB { frame, offset } => Some(frame.start_address() + offset),
            TranslationResult::Frame2MB { frame, offset } => Some(frame.start_address() + offset),
            TranslationResult::Frame1GB { frame, offset } => Some(frame.start_address() + offset),
            TranslationResult::PageNotMapped | TranslationResult::InvalidFrameAddress(_) => None,
        }
    }
}