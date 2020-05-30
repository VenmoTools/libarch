use crate::arch::intel::{
    chips::flags::{CR0Flags, CR2Flags, CR3Flags, CR4Flags},
    instructions::register::{
        read_cr0, read_cr2, read_cr3, read_cr4,
        write_cr0, write_cr2, write_cr3, write_cr4,
    },
    x64::{
        address::{PhysAddr, PhysicalAddress, VirtAddr},
        paging::Frame,
    },
};

macro_rules! impl_crn {
    ($cr:ident,$flags:ident,$read:ident,$write:ident) => {
        pub struct $cr;

        impl $cr {
            pub fn flags() -> $flags {
                $flags::from_bits_truncate(Self::read_raw())
            }
            pub fn read_raw() -> u64 {
                unsafe {
                    $read()
                }
            }
            pub unsafe fn write_raw(data: u64) {
                $write(data)
            }
        }
    };
}


// CR0
impl_crn!(CR0,CR0Flags,read_cr0,write_cr0);

impl CR0 {
    pub fn is_enable_protected_mode() -> bool {
        Self::flags().contains(CR0Flags::PROTECTED_MODE_ENABLE)
    }

    pub fn is_enable_paging() -> bool {
        Self::flags().contains(CR0Flags::PAGING)
    }

    pub unsafe fn write(flags: CR0Flags) {
        let old = Self::read_raw();
        let reserved = old & !(CR0Flags::all().bits());
        let new = reserved | flags.bits();
        write_cr0(new)
    }
}

impl_crn!(CR2,CR2Flags,read_cr2,write_cr2);

impl CR2 {
    /// 从当前的CR2寄存器中读取线性地址
    pub fn read() -> VirtAddr {
        VirtAddr::new(Self::read_raw())
    }
}

// CR3
impl_crn!(CR3,CR3Flags,read_cr3,write_cr3);

impl CR3 {
    /// 从CR3寄存器中读取读取当前P4页表的地址
    pub fn read() -> (Frame, CR3Flags) {
        let data = Self::read_raw();
        let flags = CR3Flags::from_bits_truncate(data);
        let addr = PhysAddr::new(data & 0x00F_FFFF_FFFF_F000);
        let frame = Frame::include_address(addr);
        (frame, flags)
    }

    /// 将新的P4页表地址写入CR3寄存器中
    pub unsafe fn write(frame: Frame, flags: CR3Flags) {
        let addr = frame.start_address();
        let data = addr.as_u64() | flags.bits();
        Self::write_raw(data)
    }
}

// CR4
impl_crn!(CR4,CR4Flags,read_cr4,write_cr4);
impl CR4 {
    #[allow(non_snake_case)]
    pub fn is_enable_PAE() -> bool {
        Self::flags().contains(CR4Flags::PHYSICAL_ADDRESS_EXTENSION)
    }

    /// 向CR4寄存器写入 Flags数据
    pub unsafe fn write(flags: CR4Flags) {
        let old = Self::read_raw();
        let reserved = old & !(CR4Flags::all().bits());
        let new = reserved | flags.bits();
        Self::write_raw(new)
    }
}
