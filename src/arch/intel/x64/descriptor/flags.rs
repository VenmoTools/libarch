use bitflags::bitflags;

bitflags! {
   /// GDT 描述符标志位
   /// 代码段描述符标志位
   /// 位43  42 	41 	40 	描述符类型 	说明
   ///  1 	  0 	0 	0 	代码 	    仅执行
   ///  1 	  0 	0 	1 	代码 	    仅执行，已访问
   ///  1 	  0 	1 	0 	代码 	    执行/可读
   ///  1 	  0 	1 	1 	代码 	    执行/可读，已访问
   ///  1 	  1 	0 	0 	代码 	    一致性段，仅执行
   ///  1 	  1 	0 	1 	代码 	    一致性段，仅执行，已访问
   ///  1 	  1 	1 	0 	代码 	    一致性段，执行/可读
   ///  1 	  1 	1 	1 	代码 	    一致性段，执行/可读，已访问
   /// 代码段描述符
   /// 43  42  41  40 	说明
   /// 0 	0   0   0   16B描述符的高8B
   /// 0 	0   1   0   LDT段描述符
   /// 1 	0 	0 	1 	64位TSS段描述符
   /// 1 	0 	1 	1 	64位TSS段描述符
   /// 1 	1 	1 	0 	64位中断门描述符
   /// 1 	1 	1 	1 	64位陷进门描述符
   pub struct DescriptorFlags: u64 {
        const ACCESSED         = 1 << 40;
        const WRITABLE          = 1 << 41;
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const PRESENT           = 1 << 47;
        const LONG_MODE         = 1 << 53;
        const DPL_RING_0        = 0 << 45;
        const DPL_RING_3        = 3 << 45;
    }
}


bitflags! {
    pub struct GdtAccessFlags: u8 {
        const TSS_AVAIL = 0x9;
        const TSS_BUSY = 0xB;
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SYSTEM = 1 << 4;
        const EXECUTABLE = 1 << 3;
        const CONFORMING = 1 << 2;
        const PRIVILEGE = 1 << 1;
        const DIRTY = 1;

    }
}

bitflags! {
    pub struct GdtFlags: u8{
        const PAGE_SIZE = 1 << 7;
        const PROTECTED_MODE = 1 << 6;
        const LONG_MODE = 1 << 5;
    }
}

bitflags! {
    pub struct IdtFlags: u16 {
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SS = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP = 0xF;
    }
}