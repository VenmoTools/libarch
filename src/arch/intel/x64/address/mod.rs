pub use phys::{NoInvalidPhysAddr, PhysAddr};
pub use virt::{NoCanonicalAddr, VirtAddr};

use crate::arch::intel::Arch;

mod virt;
mod phys;

pub fn align_down(addr: u64, align: u64) -> u64 {
    assert_eq!(align & (align - 1), 0, "`align` must be a power of two");
    addr & !(align - 1)
}

pub fn align_up(addr: u64, align: u64) -> u64 {
    assert_eq!(align & (align - 1), 0, "`align` must be a power of two");

    let mask = align - 1;
    if addr & mask == 0 {
        addr
    } else {
        (addr | mask) + 1
    }
}

pub trait VirtualAddress<A: Arch> {
    type BITS;

    /// 将虚拟地址结构转为u64类型
    fn as_u64(&self) -> u64;
    /// 将虚拟地址结构转为usize类型
    fn as_usize(&self) -> usize;
    /// 从给定的指针中创建虚拟地址
    fn from_pointer<T>(pointer: *const T) -> Self;
    /// 将虚拟地址转为64位宽的原始指针
    fn as_ptr<T>(self) -> *const T where Self: core::marker::Sized {
        cast::usize(self.as_u64()) as *const T
    }
    /// 将虚拟地址转为64位宽的可变原始指针
    fn as_mut_ptr<T>(self) -> *mut T where Self: core::marker::Sized {
        self.as_ptr::<T>() as *mut T
    }
    /// 将虚拟地址向上对齐
    fn align_up<U>(self, align: U) -> Self where U: Into<Self::BITS>;
    /// 将虚拟地址向下对齐
    fn align_down<U>(self, align: U) -> Self where U: Into<Self::BITS>;
    /// 判断虚拟地址是否被对齐
    fn is_aligned<U>(self, align: U) -> bool where U: Into<Self::BITS>;
}


pub trait PhysicalAddress<A: Arch> {
    type BITS;

    fn as_u64(self) -> u64;
    fn as_usize(self) -> usize;
    /// 用于判断物理地址是否是零地址
    fn is_null(&self) -> bool;
    /// 页表向上对齐
    fn align_up<U>(self, align: U) -> Self where U: Into<u64>;
    /// 页表向下对齐
    fn align_down<U>(self, align: U) -> Self where U: Into<u64>;
    /// 判断当前地址是否已经被对齐
    fn is_aligned<U>(self, align: U) -> bool where U: Into<u64>;
    /// 转换为可变裸指针
    fn as_mut(&self) -> *mut Self::BITS;
    /// 转换为裸指针
    fn as_ptr(&self) -> *const Self::BITS;
}