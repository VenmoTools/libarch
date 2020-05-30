use super::consts::*;
use super::flags::{
    EferFlags,
    LocalAPICFlags,
};
use super::super::{
    instructions::{
        register::{rdmsr, wrmsr}
    }
};

macro_rules! impl_msr_set {
    ($name:ident,$msr_value:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name(MSR);

        impl $name{
            pub fn new() -> Self{
                Self(MSR::new($msr_value))
            }
            pub unsafe fn write_raw(&mut self,data:u64){
                self.0.write_raw(data)
            }
            pub fn read_raw(&self) -> u64{
                self.0.read_raw()
            }
        }
    };
}

/// structure fot msr chip set
#[derive(Debug, Clone)]
pub struct MSR(u32);

impl MSR {
    pub fn new(n: u32) -> Self {
        MSR(n)
    }

    pub fn read_raw(&self) -> u64 {
        rdmsr(self.0)
    }

    pub unsafe fn write_raw(&mut self, data: u64) {
        wrmsr(self.0, data)
    }

    pub unsafe fn write_msr(msr: u32, value: u64) {
        wrmsr(msr, value)
    }

    pub fn read_msr(msr: u32) -> u64 {
        rdmsr(msr)
    }
}

/// The Extended Feature Enable Register.
impl_msr_set!(Efer,IA32_EFER);


impl Efer {
    pub fn is_in_long_mode(&self) -> bool {
        self.read().contains(EferFlags::LONG_MODE_ENABLE)
    }
    pub unsafe fn write(&mut self, flags: EferFlags) {
        let old_value = self.read_raw();
        let reserved = old_value & !(EferFlags::all().bits());
        let new_value = reserved | flags.bits();
        self.write_raw(new_value);
    }
    pub fn read(&self) -> EferFlags {
        EferFlags::from_bits_truncate(self.read_raw())
    }
}

/// FS.Base Model Specific Register.
impl_msr_set!(FsBase,IA32_FS_BASE);


/// GS.Base Model Specific Register.
impl_msr_set!(GsBase,IA32_GS_BASE);


/// KernelGsBase Model Specific Register.
impl_msr_set!(KernelGsBase,IA32_KERNEL_GS_BASE);


/// 错误状态寄存器
impl_msr_set!(ESR,IA32_X2APIC_ESR);

impl ESR {
    pub fn flags(&self) -> LocalAPICFlags {
        LocalAPICFlags::from_bits_truncate(self.read_raw() as u8)
    }
}

/// Task Priority Register 任务优先权寄存器
impl_msr_set!(TPR,IA32_X2APIC_TPR);

/// Processor Priority Register 处理器优先权寄存器 只读
impl_msr_set!(PPR,IA32_X2APIC_PPR);

/// Interrupt Request Register 中断请求处理器 256位
impl_msr_set!(IRR0,IA32_X2APIC_IRR0);
impl_msr_set!(IRR1,IA32_X2APIC_IRR1);
impl_msr_set!(IRR2,IA32_X2APIC_IRR2);
impl_msr_set!(IRR3,IA32_X2APIC_IRR3);
impl_msr_set!(IRR4,IA32_X2APIC_IRR4);
impl_msr_set!(IRR5,IA32_X2APIC_IRR5);
impl_msr_set!(IRR6,IA32_X2APIC_IRR6);
impl_msr_set!(IRR7,IA32_X2APIC_IRR7);

/// In-Service Register 正在服务寄存器 256位
impl_msr_set!(ISR0,IA32_X2APIC_ISR0);
impl_msr_set!(ISR1,IA32_X2APIC_ISR1);
impl_msr_set!(ISR2,IA32_X2APIC_ISR2);
impl_msr_set!(ISR3,IA32_X2APIC_ISR3);
impl_msr_set!(ISR4,IA32_X2APIC_ISR4);
impl_msr_set!(ISR5,IA32_X2APIC_ISR5);
impl_msr_set!(ISR6,IA32_X2APIC_ISR6);
impl_msr_set!(ISR7,IA32_X2APIC_ISR7);


/// Trigger Mode Register 触发模式寄存器 256位
impl_msr_set!(TMR0,IA32_X2APIC_TMR0);
impl_msr_set!(TMR1,IA32_X2APIC_TMR1);
impl_msr_set!(TMR2,IA32_X2APIC_TMR2);
impl_msr_set!(TMR3,IA32_X2APIC_TMR3);
impl_msr_set!(TMR4,IA32_X2APIC_TMR4);
impl_msr_set!(TMR5,IA32_X2APIC_TMR5);
impl_msr_set!(TMR6,IA32_X2APIC_TMR6);
impl_msr_set!(TMR7,IA32_X2APIC_TMR7);


/// End Of Interrupt 中断结束寄存器 32位
impl_msr_set!(EOI,IA32_X2APIC_EOI);


/// Spurious Interrupt Vector Register 伪中断向量寄存器 32位
impl_msr_set!(SIVR,IA32_X2APIC_SIVR);

///LDR
impl_msr_set!(LDR,IA32_X2APIC_LDR);

/// ICR
impl_msr_set!(ICR,IA32_X2APIC_ICR);
/// LVT timer
impl_msr_set!(LvtTimer,IA32_X2APIC_LVT_TIMER);
/// LVT Theraml
impl_msr_set!(LvtThermal, IA32_X2APIC_LVT_THERMAL);
/// LVT Perf
impl_msr_set!(LvtPerf, IA32_X2APIC_LVT_PMI);
/// LVT Lint0
impl_msr_set!(LvtLint0, IA32_X2APIC_LVT_LINT0);
/// LVT Lint1
impl_msr_set!(LvtLint1, IA32_X2APIC_LVT_LINT1);
/// LVT Error
impl_msr_set!(LvtError, IA32_X2APIC_LVT_ERROR);
/// TICR
impl_msr_set!(TICR, IA32_X2APIC_INIT_COUNT);
/// TCCR
impl_msr_set!(TCCR, IA32_X2APIC_CUR_COUNT);
/// TDCR
impl_msr_set!(TDCR, IA32_X2APIC_DIV_CONF);
/// IPI
impl_msr_set!(IPI, IA32_X2APIC_SELF_IPI);