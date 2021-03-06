use core::fmt;
use core::ops::{Index, IndexMut};

use crate::arch::intel::x64::address::{PhysAddr, PhysicalAddress};
use crate::arch::intel::x64::paging::{Frame, Page4KB, PageSize};
use crate::arch::intel::x64::paging::flags::PageTableFlags;
use crate::arch::intel::x64::paging::result::FrameError;

use super::PageIndex;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64
}

impl PageTableEntry {
    /// 创建一个空的页表页表项
    pub const fn new() -> Self {
        PageTableEntry { entry: 0 }
    }
    /// 判断页表页表项是否被使用
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }
    /// 将页表页表项设置位未使用
    pub fn set_unused(&mut self) {
        self.entry = 0;
    }
    /// 获取当前页表项的bitmap
    pub const fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.entry)
    }
    /// 获取当前页表项所映射的物理地址
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.entry & 0x000FFFFF_FFFFF000)
    }
    /// 返回当前Entry的页帧
    /// # Error
    /// * `FrameError::FrameNotPresent` 表示当前Entry没有被置`PRESENT`位
    pub fn frame(&self) -> Result<Frame, FrameError> {
        if !self.flags().contains(PageTableFlags::PRESENT) {
            Err(FrameError::FrameNotPresent)
        } else if self.flags().contains(PageTableFlags::HUGE_PAGE) {
            Err(FrameError::HugeFrame)
        } else {
            Ok(Frame::include_address(self.addr()))
        }
    }

    /// 将entry与物理地址做映射
    pub fn set_addr(&mut self, phy: PhysAddr, flags: PageTableFlags) {
        assert!(phy.is_aligned(Page4KB::P_SIZE));
        self.entry = phy.as_u64() | flags.bits();
    }

    /// 将entry与指定的页帧做映射
    pub fn set_frame(&mut self, f: Frame, flags: PageTableFlags) {
        assert!(!flags.contains(PageTableFlags::HUGE_PAGE));
        self.set_addr(f.start_address(), flags)
    }

    /// 为entry设置指定Flags
    pub fn set_flags(&mut self, flags: PageTableFlags) {
        self.entry = self.addr().as_u64() | flags.bits()
    }
}

impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("PageTableEntry");
        f.field("addr", &self.addr());
        f.field("flags", &self.flags());
        f.finish()
    }
}

pub const ENTRY_COUNT: usize = 512;

#[repr(align(4096))]
#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    /// 创建一个空的页表
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::new(); ENTRY_COUNT]
        }
    }
    /// 清空表中所有内容
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
    /// 获取只读迭代器
    pub fn iter(&self) -> impl Iterator<Item=&PageTableEntry> {
        self.entries.iter()
    }
    /// 获取可变迭代器
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut PageTableEntry> {
        self.entries.iter_mut()
    }
}

// ----------------- 为 Page实现索引功能 支持usize索引和PageIndex索引 -----------------
impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Index<PageIndex> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: PageIndex) -> &Self::Output {
        &self.entries[cast::usize(u16::from(index))]
    }
}

impl IndexMut<PageIndex> for PageTable {
    fn index_mut(&mut self, index: PageIndex) -> &mut Self::Output {
        &mut self.entries[cast::usize(u16::from(index))]
    }
}

impl fmt::Debug for PageTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.entries[..].fmt(f)
    }
}