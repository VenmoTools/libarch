use core::marker::PhantomData;

use bit_field::BitField;

use crate::arch::intel::{ArchIntel, PrivilegedLevel, Selector, TablePointer};

pub mod address;
pub mod paging;
pub mod memory;
pub mod descriptor;

/// 名称 	功能描述
/// Index 	用于索引目标段描述符
/// TI 	    目标段描述符所在的描述符表类型
/// RPL 	请求特权级
///
/// 详情查看 Intel 3a, Section 3.4.2 "Segment Selectors"
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    /// 使用给定的索引和特权级创建新的段选择子
    pub fn new(index: u16, rpl: PrivilegedLevel) -> SegmentSelector {
        SegmentSelector(index << 3 | (rpl as u16))
    }
}

impl From<u16> for SegmentSelector {
    fn from(s: u16) -> Self {
        Self(s)
    }
}

impl Selector for SegmentSelector {
    fn index(&self) -> u16 {
        self.0 >> 3
    }
    fn rpl(&self) -> PrivilegedLevel {
        PrivilegedLevel::from_u16(self.0.get_bits(0..2))
    }

    fn as_u16(&self) -> u16 {
        self.0
    }

    fn as_usize(&self) -> usize {
        usize::from(self.0)
    }

    fn as_u64(&self) -> u64 {
        u64::from(self.0)
    }

    fn as_u32(&self) -> u32 {
        u32::from(self.0)
    }
}


/// 用于将GDT，IDT等描述符保存为指针形式
#[derive(Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer<A: ArchIntel> {
    /// 描述符段限长
    limit: u16,
    /// 描述符的内存裸指针
    base: u64,
    _mark: PhantomData<A>,
}


impl<A: ArchIntel> Clone for DescriptorTablePointer<A> {
    fn clone(&self) -> Self {
        DescriptorTablePointer {
            limit: { self.limit },
            base: { self.base },
            _mark: Default::default(),
        }
    }
}

impl<A: ArchIntel> TablePointer for DescriptorTablePointer<A> {
    fn limit(&self) -> u16 {
        self.limit
    }

    fn base(&self) -> u64 {
        self.base
    }
}

impl<A: ArchIntel> DescriptorTablePointer<A> {
    const ARCH: u64 = A::BIT;
    pub fn empty() -> Self {
        Self {
            limit: 0,
            base: 0,
            _mark: PhantomData,
        }
    }
}