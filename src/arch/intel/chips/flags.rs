use bitflags::bitflags;

// Extended Feature Enable Register
bitflags! {
    /// 当IA32-EFER寄存器中的某位被设置并且PAE(Physical Address Extensions，物理地址扩展)模式被启用
    pub struct EferFlags: u64 {
        /// Enables the `syscall` and `sysret` instructions.
        const SYSTEM_CALL_EXTENSIONS = 1 << 0;
        /// Activates long mode, requires activating paging.
        const LONG_MODE_ENABLE = 1 << 8;
        /// Indicates that long mode is active.
        const LONG_MODE_ACTIVE = 1 << 10;
        /// Enables the no-execute page-protection feature.
        const NO_EXECUTE_ENABLE = 1 << 11;
        /// Enables SVM extensions.
        const SECURE_VIRTUAL_MACHINE_ENABLE = 1 << 12;
        /// Enable certain limit checks in 64-bit mode.
        const LONG_MODE_SEGMENT_LIMIT_ENABLE = 1 << 13;
        /// Enable the `fxsave` and `fxrstor` instructions to execute faster in 64-bit mode.
        const FAST_FXSAVE_FXRSTOR = 1 << 14;
        /// Changes how the `invlpg` instruction operates on TLB entries of upper-level entries.
        const TRANSLATION_CACHE_EXTENSION = 1 << 15;
    }
}

// Local APIC
bitflags! {
    pub struct LocalAPICFlags: u8{
        /// 发送校验和错误 LocalAPIC检测到发往APIC总线的中断消息出现校验和错误
        const SEND_CHECKSUM_ERROR = 1 << 0;
        /// 接受校验和错误 LocalAPIC检测到来自APIC总线的中断消息出现校验和错误
        const RECV_CHECKSUM_ERROR = 1 << 1;
        /// 发送中断消息受理错误   LocalAPIC检测到发往APIC总线的中断消息未被其他APIC受理
        const SEND_INTERRUPT_MSG_ACCPT_ERROR = 1 << 2;
        /// 接受中断消息受理错误  LocalAPIC检测到来自APIC总线的中断消息未被其他APIC受理
        const RECV_INTERRUPT_MSG_ACCPT_ERROR = 1 << 3;
        /// IPI无法正确定向 在Local APIC不支持Lowest-Priority投递模式下使用Lowest-Priority投递发送IPI消息
        const IPI_CANNOT_ORIENTED_CORRECTLY = 1 << 4;
        /// 发送的中断向量号不合法 Local APIC通过ICR或SELF IPI寄存器发送的中断消息的中断向量号不合法
        const SEND_INTERRUPT_VECOTR_INVALID = 1 << 5;
        /// 接受的中断向量号不合法 Local APIC接受的中断消息的中断向量号不合法
        const RECV_INTERRUPT_VECOTR_INVALID = 1 << 6;
        /// 寄存器地址不合法 Local APIC处于xAPIC模式下，软件访问了Local APIC寄存器地址空间
        const REGISTER_ADDRESS_INVALID = 1 << 7;
    }
}

// LVT Entry
bitflags! {
    #[allow(non_upper_case_globals)]
    pub struct LVTEntryFlags: u32{
        /// 投递模式标志位
        const DELIVERY_MODE_1 = 1 << 8;
        const DELIVERY_MODE_2 = 1 << 9;
        const DELIVERY_MODE_3 = 1 << 10;
        /// Fixed
        const DELIVERY_FIXED = Self::DELIVERY_MODE_1.bits & Self::DELIVERY_MODE_2.bits & Self::DELIVERY_MODE_3.bits;
        /// SMI
        const DELIVERY_SMI = Self::DELIVERY_MODE_2.bits;
        /// NMI
        const DELIVERY_NMI = Self::DELIVERY_MODE_3.bits;
        /// ExtINT
        const DELIVERY_EXT_INT = Self::DELIVERY_MODE_1.bits | Self::DELIVERY_MODE_2.bits | Self::DELIVERY_MODE_3.bits;
        /// INIT
        const DELIVERY_INIT = Self::DELIVERY_MODE_1.bits | Self::DELIVERY_MODE_3.bits;
        /// 投递状态 0空间 1发送挂起
        const DELIVERY_STATUS = 1 << 12;
        /// 电平触发极性
        const INTERRUPT_INPUT_PIN_POLARITY = 1 << 13;
        /// 远程IRR标志位
        const REMOTE_IRR = 1 << 14;
        /// 触发模式 0边沿触发，1电平触发
        const TRIGGER_MODE = 1<<15;
        /// 屏蔽标志位 1未屏蔽，0为已屏蔽
        const MASK_FLAGS = 1 << 16;
        /// 定时模式
        const TIMER_MODE_1 = 1<<17;
        const TIMER_MODE_2 = 1<<18;
        /// 一次性定时
        const TIMER_ONE_SHOT = Self::TIMER_MODE_1.bits & Self::TIMER_MODE_2.bits;
        /// 周期性定时
        const TIMER_PERIODIC = Self::TIMER_MODE_1.bits;
        /// 定时
        const TIMER_TSC_DEADLINE = Self::TIMER_MODE_2.bits;
    }
}

// RFlags寄存器
bitflags! {
    pub struct RFlags: u64 {
        /// Processor feature identification flag.
        ///
        /// If this flag is modifiable, the CPU supports CPUID.
        const ID = 1 << 21;
        /// Indicates that an external, maskable interrupt is pending.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        /// Virtual image of the INTERRUPT_FLAG bit.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT = 1 << 19;
        /// Enable automatic alignment checking if CR0.AM is set. Only works if CPL is 3.
        const ALIGNMENT_CHECK = 1 << 18;
        /// Enable the virtual-8086 mode.
        const VIRTUAL_8086_MODE = 1 << 17;
        /// Allows to restart an instruction following an instrucion breakpoint.
        const RESUME_FLAG = 1 << 16;
        /// Used by `iret` in hardware task switch mode to determine if current task is nested.
        const NESTED_TASK = 1 << 14;
        /// The high bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_HIGH = 1 << 13;
        /// The low bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_LOW = 1 << 12;
        /// Set by hardware to indicate that the sign bit of the result of the last signed integer
        /// operation differs from the source operands.
        const OVERFLOW_FLAG = 1 << 11;
        /// Determines the order in which strings are processed.
        const DIRECTION_FLAG = 1 << 10;
        /// Enable interrupts.
        const INTERRUPT_FLAG = 1 << 9;
        /// Enable single-step mode for debugging.
        const TRAP_FLAG = 1 << 8;
        /// Set by hardware if last arithmetic operation resulted in a negative value.
        const SIGN_FLAG = 1 << 7;
        /// Set by hardware if last arithmetic operation resulted in a zero value.
        const ZERO_FLAG = 1 << 6;
        /// Set by hardware if last arithmetic operation generated a carry ouf of bit 3 of the
        /// result.
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        /// Set by hardware if last result has an even number of 1 bits (only for some operations).
        const PARITY_FLAG = 1 << 2;
        /// Set by hardware if last arithmetic operation generated a carry out of the
        /// most-significant bit of the result.
        const CARRY_FLAG = 1 << 0;
    }
}

// CR3寄存器用于设置4级页表
bitflags! {
    pub struct CR3Flags: u64 {
        /// Use a writethrough cache policy for the P4 table (else a writeback policy is used).
        const PAGE_LEVEL_WRITETHROUGH = 1 << 3;
        /// Disable caching for the P4 table.
        const PAGE_LEVEL_CACHE_DISABLE = 1 << 4;
    }
}

// CR3寄存器用于设置4级页表
bitflags! {
    pub struct CR2Flags: u64 {
        /// empty
        const PAGE_LEVEL_WRITETHROUGH = 1 << 0;
    }
}
// CR4寄存器 用于设置4级页表
bitflags! {
    /// Controls cache settings for the level 4 page table.
    pub struct CR4Flags: u64 {
        /// Enables hardware-supported performance enhancements for software running in
        /// virtual-8086 mode.
        const VIRTUAL_8086_MODE_EXTENSIONS = 1 << 0;
        /// Enables support for protected-mode virtual interrupts.
        const PROTECTED_MODE_VIRTUAL_INTERRUPTS = 1 << 1;
        /// When set, only privilege-level 0 can execute the RDTSC or RDTSCP instructions.
        const TIMESTAMP_DISABLE = 1 << 2;
        /// Enables I/O breakpoint capability and enforces treatment of DR4 and DR5 registers
        /// as reserved.
        const DEBUGGING_EXTENSIONS = 1 << 3;
        /// Enables the use of 4MB physical frames; ignored in long mode.
        const PAGE_SIZE_EXTENSION = 1 << 4;
        /// Enables physical address extension and 2MB physical frames; required in long mode.
        const PHYSICAL_ADDRESS_EXTENSION = 1 << 5;
        /// Enables the machine-check exception mechanism.
        const MACHINE_CHECK_EXCEPTION = 1 << 6;
        /// Enables the global-page mechanism, which allows to make page translations global
        /// to all processes.
        const PAGE_GLOBAL = 1 << 7;
        /// Allows software running at any privilege level to use the RDPMC instruction.
        const PERFORMANCE_MONITOR_COUNTER = 1 << 8;
        /// Enable the use of legacy SSE instructions; allows using FXSAVE/FXRSTOR for saving
        /// processor state of 128-bit media instructions.
        const OSFXSR = 1 << 9;
        /// Enables the SIMD floating-point exception (#XF) for handling unmasked 256-bit and
        /// 128-bit media floating-point errors.
        const OSXMMEXCPT_ENABLE = 1 << 10;
        /// Prevents the execution of the SGDT, SIDT, SLDT, SMSW, and STR instructions by
        /// user-mode software.
        const USER_MODE_INSTRUCTION_PREVENTION = 1 << 11;
        /// Enables 5-level paging on supported CPUs.
        const L5_PAGING = 1 << 12;
        /// Enables VMX insturctions.
        const VIRTUAL_MACHINE_EXTENSIONS = 1 << 13;
        /// Enables SMX instructions.
        const SAFER_MODE_EXTENSIONS = 1 << 14;
        /// Enables software running in 64-bit mode at any privilege level to read and write
        /// the FS.base and GS.base hidden segment register state.
        const FSGSBASE = 1 << 16;
        /// Enables process-context identifiers (PCIDs).
        const PCID = 1 << 17;
        /// Enables extendet processor state management instructions, including XGETBV and XSAVE.
        const OSXSAVE = 1 << 18;
        /// Prevents the execution of instructions that reside in pages accessible by user-mode
        /// software when the processor is in supervisor-mode.
        const SUPERVISOR_MODE_EXECUTION_PROTECTION = 1 << 20;
        /// Enables restrictions for supervisor-mode software when reading data from user-mode
        /// pages.
        const SUPERVISOR_MODE_ACCESS_PREVENTION = 1 << 21;
        /// Enables 4-level paging to associate each linear address with a protection key.
        const PROTECTION_KEY = 1 << 22;
    }
}

// CR0寄存器.标志位
bitflags! {
    pub struct CR0Flags: u64 {
        /// Enables protected mode.
        const PROTECTED_MODE_ENABLE = 1 << 0;
        /// Enables monitoring of the coprocessor, typical for x87 instructions.
        ///
        /// Controls together with the `TASK_SWITCHED` flag whether a `wait` or `fwait`
        /// instruction should cause a device-not-available exception.
        const MONITOR_COPROCESSOR = 1 << 1;
        /// Force all x87 and MMX instructions to cause an exception.
        const EMULATE_COPROCESSOR = 1 << 2;
        /// Automatically set to 1 on _hardware_ task switch.
        ///
        /// This flags allows lazily saving x87/MMX/SSE instructions on hardware context switches.
        const TASK_SWITCHED = 1 << 3;
        /// Enables the native error reporting mechanism for x87 FPU errors.
        const NUMERIC_ERROR = 1 << 5;
        /// Controls whether supervisor-level writes to read-only pages are inhibited.
        ///
        /// When set, it is not possible to write to read-only pages from ring 0.
        const WRITE_PROTECT = 1 << 16;
        /// Enables automatic alignment checking.
        const ALIGNMENT_MASK = 1 << 18;
        /// Ignored. Used to control write-back/write-through cache strategy on older CPUs.
        const NOT_WRITE_THROUGH = 1 << 29;
        /// Disables internal caches (only for some cases).
        const CACHE_DISABLE = 1 << 30;
        /// Enables page translation.
        const PAGING = 1 << 31;
    }
}