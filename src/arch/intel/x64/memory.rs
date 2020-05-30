use alloc::vec::Vec;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MEMORY_AREA:[MemoryArea; 512] = [MemoryArea::default();512];
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MemoryType {
    EmptyArea,
    FreeArea,
    UsedArea,
    ReservedArea,
    ACPIArea,
    ACPIReservedArea,
    ReservedHibernate,
    Defective,
    UefiRunTimeCode,
    UefiRunTimeData,
    MMIO,
    MMIOPortArea,
    ErrorArea,
}

impl Default for MemoryType {
    fn default() -> Self {
        MemoryType::EmptyArea
    }
}

/// memory map area
#[derive(Copy, Clone, Debug, Default)]
pub struct MemoryArea {
    /// area start address
    pub start_addr: u64,
    pub end_addr: u64,
    pub length: u64,
    pub ty: MemoryType,
}

impl MemoryArea {
    pub fn new(start_addr: u64, end_addr: u64, ty: MemoryType, len: u64) -> Self {
        Self {
            start_addr,
            end_addr,
            ty,
            length: len,
        }
    }

    pub fn size(&self) -> u64 {
        self.length
    }
    pub fn start_address(&self) -> u64 {
        self.start_addr
    }
}

pub struct MemorySpace {
    pub(crate) space: Vec<MemoryArea>,
}

impl MemorySpace {
    pub fn new() -> Self {
        Self {
            space: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&MemoryArea> + '_ {
        self.space.iter()
    }

    pub fn add_area(&mut self, start_addr: u64, end_addr: u64, ty: MemoryType, len: u64) {
        self.space.push(MemoryArea::new(start_addr, end_addr, ty, len))
    }
}

/// 遍历指定类型的内存区域
#[derive(Clone)]
pub struct MemoryAreaIter {
    ty: MemoryType,
    index: usize,
}

impl MemoryAreaIter {
    pub fn new(ty: MemoryType) -> Self {
        Self {
            ty,
            index: 0,
        }
    }
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < MEMORY_AREA.len() {
            let entry = &MEMORY_AREA[self.index];
            self.index += 1;
            if self.ty == entry.ty {
                return Some(entry);
            }
        }
        None
    }
}