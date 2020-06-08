use core::fmt::Debug;

pub mod x32;
pub mod x64;
pub mod instructions;
pub mod chips;
pub mod call_convention;
pub mod interrupt;
pub mod timer;

pub trait DescriptorTable {
    fn load();
}

pub trait Selector: Debug + From<u16> {
    /// 返回当前的描述符索引
    fn index(&self) -> u16;
    /// 返回当前描述符的特权级
    fn rpl(&self) -> PrivilegedLevel;
    /// 索引(16位)
    fn as_u16(&self) -> u16;
    /// 索引(usize)
    fn as_usize(&self) -> usize;
    /// 索引(64位)
    fn as_u64(&self) -> u64;
    /// 索引(32位)
    fn as_u32(&self) -> u32;
}

/// 系统特权级
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PrivilegedLevel {
    /// 特权级0
    Ring0 = 0,
    /// 特权级1
    Ring1 = 1,
    /// 特权级2
    Ring2 = 2,
    /// 特权级3
    Ring3 = 3,
}

impl PrivilegedLevel {
    /// 根据给定额值判断当前特权级，如果不在范围则Panic
    pub fn from_u16(level: u16) -> PrivilegedLevel {
        match level {
            0 => PrivilegedLevel::Ring0,
            1 => PrivilegedLevel::Ring1,
            2 => PrivilegedLevel::Ring2,
            3 => PrivilegedLevel::Ring3,
            other => panic!("invalid privileged level `{}`", other),
        }
    }
}

pub trait ArchIntel {
    const BIT: u64;
    const DISPLAY_STR: &'static str;
}

pub struct IntelX32;

impl ArchIntel for IntelX32 {
    const BIT: u64 = 32;
    const DISPLAY_STR: &'static str = "Intel x32";
}

pub struct IntelX64;

impl ArchIntel for IntelX64 {
    const BIT: u64 = 64;
    const DISPLAY_STR: &'static str = "Intel x64";
}

pub trait TablePointer<A: ArchIntel = IntelX64> {
    fn limit(&self) -> u16;

    fn base(&self) -> u64;
}


