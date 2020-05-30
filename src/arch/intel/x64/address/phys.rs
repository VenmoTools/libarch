use core::fmt::{self, Debug};
use core::ops::{Add, AddAssign, Sub, SubAssign};

use bit_field::BitField;

use crate::arch::intel::IntelX64;
use crate::arch::intel::x64::address::PhysicalAddress;

use super::{align_down, align_up};

/// 64物理地址结构
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(u64);

/// 无效的64位物理地址
#[derive(Debug)]
pub struct NoInvalidPhysAddr(u64);

impl PhysAddr {
    /// 根据给定原始地址创建物理地址，如果 52 置64位没有被置位则会Panic
    pub fn new(addr: u64) -> PhysAddr {
        assert_eq!(addr.get_bits(52..64), 0, "physical addresses must not have any bits in the range 52 to 64 set");
        PhysAddr(addr)
    }
    ///  根据给定原始地址创建物理地址，如果 52 置64位没有被置位则会返回Err(NoInvalidPhysAddr)
    pub fn try_new(addr: u64) -> Result<PhysAddr, NoInvalidPhysAddr> {
        match addr.get_bits(52..64) {
            0 => Ok(PhysAddr(addr)),
            other => Err(NoInvalidPhysAddr(other)),
        }
    }
}

impl PhysicalAddress<IntelX64> for PhysAddr {
    type BITS = u64;

    fn as_u64(self) -> u64 {
        self.0
    }
    fn as_usize(self) -> usize {
        self.0 as usize
    }
    fn is_null(&self) -> bool {
        self.0 == 0
    }
    fn align_up<U>(self, align: U) -> Self where U: Into<u64>,
    {
        PhysAddr(align_up(self.0, align.into()))
    }
    fn align_down<U>(self, align: U) -> Self where U: Into<u64>,
    {
        PhysAddr(align_down(self.0, align.into()))
    }
    fn is_aligned<U>(self, align: U) -> bool where U: Into<u64>,
    {
        self.align_down(align) == self
    }

    fn as_mut(&self) -> *mut u64 {
        self.0 as *mut u64
    }
    /// 转换为裸指针
    fn as_ptr(&self) -> *const u64 {
        self.0 as *const u64
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Physical Address({:#x})", self.0)
    }
}

impl Add<u64> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        PhysAddr::new(self.0 + rhs)
    }
}

impl AddAssign<u64> for PhysAddr {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Add<usize> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        self + cast::u64(rhs)
    }
}

#[cfg(target_pointer_width = "64")]
impl AddAssign<usize> for PhysAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(cast::u64(rhs))
    }
}

impl Sub<u64> for PhysAddr {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        PhysAddr::new(self.0.checked_sub(rhs).unwrap())
    }
}

impl SubAssign<u64> for PhysAddr {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Sub<usize> for PhysAddr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        self - cast::u64(rhs)
    }
}

#[cfg(target_pointer_width = "64")]
impl SubAssign<usize> for PhysAddr {
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(cast::u64(rhs))
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = u64;
    fn sub(self, rhs: PhysAddr) -> Self::Output {
        self.as_u64().checked_sub(rhs.as_u64()).unwrap()
    }
}
