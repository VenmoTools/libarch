use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use bit_field::BitField;

use crate::arch::intel::IntelX64;
use crate::arch::intel::x64::paging::{PageIndex, PageOffset};

use super::{align_down, align_up, VirtualAddress};

/// Virtual Address 虚拟地址
/// IA-32e模型线性地址的寻址能力只有48位，第48位用于线性地址寻址，高16位作为符号扩展
/// 此格式的地址被称为Canonical地址，在IA-32e模式下只有Canonical地址是可用地址空间
/// Non-Canonical地址属于无效地址空间
/// 基本的地址空间划分如下
///
/// +---------------+<- 0xFFFFFFFF_FFFFFFFF
/// |               |
/// |   Canonical   |
/// |               |<-0xFFFF8000_00000000
/// +---------------+<-0xFFFF7FFF_FFFFFFFF
/// |               |
/// | Non-Canonical |
/// |               |<-0x00008000_00000000
/// +---------------+<-0x00007FFF_FFFFFFFF
/// |               |
/// |   Canonical   |
/// |               |
/// +---------------+<-0x00000000_00000000
///
/// Canonical地址结构如下
/// 63        56 55    52 51   48 47     40 39        16 15        0
/// |           ||      ||       ||        ||           ||         |
/// +-----------+-------+--------+---------+------------+----------+
/// |BaseAddr(H)| Attr1 |limit(H)|  Attr2  | BaseAddr(L)| limit(L) |                                                 |
/// +-----------+-------+--------+---------+------------+----------+
///
/// Attribute1结构如下
/// |55| 54|53| 52|
/// +--+---+--+---+
/// |G |D/B|L |AVL|
/// +--+---+--+---+
/// 属性说明：
///       G:
///     D/B: 表示代码段的默认地址位宽和操作数位宽，D=0默认位宽为16位D=1默认位宽为32位,
///          在IA-32e模式下L=1,D=0表明默认操作数位宽为32为，地址宽为64位，在此时D=1则会触发#GP异常
///       L: L=0表明将处理器运行于32位兼容模式
///     AVL:
///
/// Attribute2结构如下
/// |47|46-45|44| 43|42|41|40|
/// +--+-----+--+---+--+--+--+
/// |P |DPL  |S |C/D|C |R |A |
/// +--+-----+--+---+--+--+--+
///       P:
///     DPL:  （特权级）
///       S: （非系统段描述符）
///     C/D: Code/Data标志位（代码段描述符）
///       C:
///       R:
///       A:
///
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtAddr(u64);

/// 无效Canonical地址
#[derive(Debug)]
pub struct NoCanonicalAddr(u64);

impl VirtualAddress<IntelX64> for VirtAddr {
    type BITS = u64;

    fn as_u64(&self) -> u64 {
        self.0
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn from_pointer<T>(pointer: *const T) -> Self {
        Self::new(cast::u64(pointer as usize))
    }

    fn align_up<U>(self, align: U) -> Self where U: Into<u64> {
        VirtAddr(align_down(self.0, align.into()))
    }
    fn align_down<U>(self, align: U) -> Self where U: Into<u64> {
        VirtAddr(align_up(self.0, align.into()))
    }
    fn is_aligned<U>(self, align: U) -> bool where U: Into<u64> {
        self.align_down(align) == self
    }
}

impl VirtAddr {
    /// 创建一个Canonical地址，传入的地址不会进行检查
    /// 该方法会检查第47位（P：已存在标示），48-63位将会被重写
    pub fn new_unchecked(mut addr: u64) -> VirtAddr {
        if addr.get_bit(47) {
            addr.set_bits(48..64, 0xFFFF);
        } else {
            addr.set_bits(48..64, 0);
        }
        VirtAddr(addr)
    }

    /// 该函数尝试创建一个Canonical地址，
    /// 如果48位到64位是正确的符号扩展名（即47位的副本）或全部为空，将成功返回
    pub fn try_new(addr: u64) -> Result<VirtAddr, NoCanonicalAddr> {
        // 获取[47，64)
        match addr.get_bits(47..64) {
            // 这里47位标示内存已存在
            0 | 0x1FFFF => Ok(VirtAddr(addr)),
            1 => Ok(VirtAddr::new_unchecked(addr)),
            other => Err(NoCanonicalAddr(other)),
        }
    }
    /// 使用给定的原始地址虚拟地址结构
    /// 如果给定的虚拟地址不符合Canonical地址将会Panic
    pub fn new(addr: u64) -> VirtAddr {
        // 给定的地址48-64位必须是不包含任何数据的
        Self::try_new(addr).expect("given address can not contain any data in bits 48 to 64")
    }
    /// 创建全0地址
    pub const fn zero() -> VirtAddr {
        VirtAddr(0)
    }


    /// 返回地址的偏移量
    /// IA-32e线性地址结构(4KB分页)
    ///
    /// 47   39 38           30 29       21 20    12 11      0
    /// |      ||              ||          ||       ||       |
    /// +------+---------------+-----------+--------+--------+
    /// | PML4 | Directory Ptr | Directory | Table  | Offset |
    /// +------+---------------+-----------+--------+--------+
    ///
    /// IA-32e线性地址结构(1MB分页)
    /// 47   39 38           30 29       21 20               0
    /// |      ||              ||          ||                |
    /// +------+---------------+-----------+-----------------+
    /// | PML4 | Directory Ptr | Directory |     Offset      |
    /// +------+---------------+-----------+-----------------+
    ///
    /// IA-32e线性地址结构(1GB分页)
    ///
    /// 47   39 38           30 29                           0
    /// |      ||              ||                            |
    /// +------+---------------+-----------------------------+
    /// | PML4 | Directory Ptr |          Offset             |
    /// +------+---------------+-----------------------------+
    ///
    ///
    pub fn page_offset(&self) -> PageOffset {
        PageOffset::new_truncate(self.0 as u16)
    }

    /// 返回一级页表索引（9位）
    pub fn page1_index(&self) -> PageIndex {
        PageIndex::new_truncate((self.0 >> 12) as u16)
    }

    /// 返回二级页表索引（9位）
    pub fn page2_index(&self) -> PageIndex {
        PageIndex::new_truncate((self.0 >> 12 >> 9) as u16)
    }

    /// 返回三级页表索引（9位）
    pub fn page3_index(&self) -> PageIndex {
        PageIndex::new_truncate((self.0 >> 12 >> 9 >> 9) as u16)
    }

    /// 返回四级页表索引（9位）
    pub fn page4_index(&self) -> PageIndex {
        PageIndex::new_truncate((self.0 >> 12 >> 9 >> 9 >> 9) as u16)
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Virtual Address: {:#x}", self.0)
    }
}

/// 地址添加计算
impl Add<u64> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        VirtAddr::new(self.0 + rhs)
    }
}

impl AddAssign<u64> for VirtAddr {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self + cast::u64(rhs)
    }
}

#[cfg(target_pointer_width = "64")]
impl AddAssign<usize> for VirtAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(cast::u64(rhs));
    }
}

impl Sub<u64> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        VirtAddr::new(self.0.checked_sub(rhs).unwrap())
    }
}


impl SubAssign<u64> for VirtAddr {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Sub<usize> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self - cast::u64(rhs)
    }
}

#[cfg(target_pointer_width = "64")]
impl SubAssign<usize> for VirtAddr {
    fn sub_assign(&mut self, rhs: usize) {
        *self = self.sub(cast::u64(rhs))
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = u64;

    fn sub(self, rhs: VirtAddr) -> Self::Output {
        self.as_u64().checked_sub(rhs.as_u64()).unwrap()
    }
}