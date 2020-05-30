/// this code base on https://github.com/kwzhao/x2apic-rs
use core::ops::Range;

use bit::BitIndex;
use paste;

use crate::arch::intel::chips::consts::{IA32_APIC_BASE, IA32_X2APIC_APICID, IA32_X2APIC_VERSION};
use crate::arch::intel::chips::msr_set::{EOI, ESR, ICR, IPI, IRR0, IRR1, IRR2, IRR3, IRR4, IRR5, IRR6, IRR7, ISR0, ISR1, ISR2, ISR3, ISR4, ISR5, ISR6, ISR7, LDR, LvtError, LvtLint0, LvtLint1, LvtPerf, LvtThermal, LvtTimer, MSR, PPR, SIVR, TCCR, TDCR, TICR, TMR0, TMR1, TMR2, TMR3, TMR4, TMR5, TMR6, TMR7, TPR};

#[derive(Debug)]
pub struct LocalApicRegisters {
    base: MSR,
    id: MSR,
    version: MSR,
    tpr: TPR,
    ppr: PPR,
    eoi: EOI,
    ldr: LDR,
    sivr: SIVR,
    isr0: ISR0,
    isr1: ISR1,
    isr2: ISR2,
    isr3: ISR3,
    isr4: ISR4,
    isr5: ISR5,
    isr6: ISR6,
    isr7: ISR7,
    tmr0: TMR0,
    tmr1: TMR1,
    tmr2: TMR2,
    tmr3: TMR3,
    tmr4: TMR4,
    tmr5: TMR5,
    tmr6: TMR6,
    tmr7: TMR7,
    irr0: IRR0,
    irr1: IRR1,
    irr2: IRR2,
    irr3: IRR3,
    irr4: IRR4,
    irr5: IRR5,
    irr6: IRR6,
    irr7: IRR7,
    error: ESR,
    icr: ICR,
    lvt_timer: LvtTimer,
    lvt_thermal: LvtThermal,
    lvt_perf: LvtPerf,
    lvt_lint0: LvtLint0,
    lvt_lint1: LvtLint1,
    lvt_error: LvtError,
    ticr: TICR,
    tccr: TCCR,
    tdcr: TDCR,
    self_ipi: IPI,
}

macro_rules! read {
    ($name:ident) => {
        paste::item! {
            pub unsafe fn $name(&self) -> u64 {
                self.$name.read_raw()
            }

            pub unsafe fn [<$name _bit>](&self, bit: usize) -> bool {
                self.$name().bit(bit)
            }

            pub unsafe fn [<$name _bit_range>](
                &self,
                pos: Range<usize>,
            ) -> u64 {
                self.$name().bit_range(pos)
            }
        }
    };
}

macro_rules! write {
    ($name:ident) => {
        paste::item! {
            pub unsafe fn [<write_ $name>](&mut self, value: u64) {
                self.$name.write_raw(value);
            }
        }
    };
}

macro_rules! read_write {
    ($name:ident) => {
        read!($name);
        write!($name);

        paste::item! {
            pub unsafe fn [<set_ $name _bit>](
                &mut self,
                bit: usize,
                val: bool,
            ) {
                let mut reg_val = self.$name();

                reg_val.set_bit(bit, val);

                self.[<write_ $name>](reg_val);
            }

            pub unsafe fn [<set_ $name _bit_range>](
                &mut self,
                pos: Range<usize>,
                val: u64,
            ) {
                let mut reg_val = self.$name();

                reg_val.set_bit_range(pos, val);

                self.[<write_ $name>](reg_val);
            }
        }
    };
}

impl LocalApicRegisters {
    pub fn new() -> Self {
        LocalApicRegisters {
            base: MSR::new(IA32_APIC_BASE),
            id: MSR::new(IA32_X2APIC_APICID),
            version: MSR::new(IA32_X2APIC_VERSION),
            tpr: TPR::new(),
            ppr: PPR::new(),
            eoi: EOI::new(),
            ldr: LDR::new(),
            sivr: SIVR::new(),
            isr0: ISR0::new(),
            isr1: ISR1::new(),
            isr2: ISR2::new(),
            isr3: ISR3::new(),
            isr4: ISR4::new(),
            isr5: ISR5::new(),
            isr6: ISR6::new(),
            isr7: ISR7::new(),
            tmr0: TMR0::new(),
            tmr1: TMR1::new(),
            tmr2: TMR2::new(),
            tmr3: TMR3::new(),
            tmr4: TMR4::new(),
            tmr5: TMR5::new(),
            tmr6: TMR6::new(),
            tmr7: TMR7::new(),
            irr0: IRR0::new(),
            irr1: IRR1::new(),
            irr2: IRR2::new(),
            irr3: IRR3::new(),
            irr4: IRR4::new(),
            irr5: IRR5::new(),
            irr6: IRR6::new(),
            irr7: IRR7::new(),
            error: ESR::new(),
            icr: ICR::new(),
            lvt_timer: LvtTimer::new(),
            lvt_thermal: LvtThermal::new(),
            lvt_perf: LvtPerf::new(),
            lvt_lint0: LvtLint0::new(),
            lvt_lint1: LvtLint1::new(),
            lvt_error: LvtError::new(),
            ticr: TICR::new(),
            tccr: TCCR::new(),
            tdcr: TDCR::new(),
            self_ipi: IPI::new(),
        }
    }

    read_write!(base);
    read!(id);
    read!(version);
    read_write!(tpr);
    read!(ppr);
    write!(eoi);
    read_write!(ldr);
    read_write!(sivr);
    read!(isr0);
    read!(isr1);
    read!(isr2);
    read!(isr3);
    read!(isr4);
    read!(isr5);
    read!(isr6);
    read!(isr7);
    read!(tmr0);
    read!(tmr1);
    read!(tmr2);
    read!(tmr3);
    read!(tmr4);
    read!(tmr5);
    read!(tmr6);
    read!(tmr7);
    read!(irr0);
    read!(irr1);
    read!(irr2);
    read!(irr3);
    read!(irr4);
    read!(irr5);
    read!(irr6);
    read!(irr7);
    read!(error);
    read_write!(icr);
    read_write!(lvt_timer);
    read_write!(lvt_thermal);
    read_write!(lvt_perf);
    read_write!(lvt_lint0);
    read_write!(lvt_lint1);
    read_write!(lvt_error);
    read_write!(ticr);
    read!(tccr);
    read_write!(tdcr);
    write!(self_ipi);
}

/// Local APIC timer modes.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum TimerMode {
    /// Timer only fires once.
    OneShot = 0b00,
    /// Timer fires periodically.
    Periodic = 0b01,
    /// Timer fires at an absolute time.
    TscDeadline = 0b10,
}

impl Into<u64> for TimerMode {
    fn into(self) -> u64 {
        self as u64
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum TimerDivide {
    /// Divide by 2.
    Div2 = 0b0000,
    /// Divide by 4.
    Div4 = 0b0001,
    /// Divide by 8.
    Div8 = 0b0010,
    /// Divide by 16.
    Div16 = 0b0011,
    /// Divide by 32.
    Div32 = 0b1000,
    /// Divide by 64.
    Div64 = 0b1001,
    /// Divide by 128.
    Div128 = 0b1010,
    /// Divide by 256.
    Div256 = 0b1011,
}


impl Into<u64> for TimerDivide {
    fn into(self) -> u64 {
        self as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum IpiDestMode {
    /// Physical destination mode.
    Physical = 0,
    /// Logical destination mode.
    Logical = 1,
}

impl Into<u64> for IpiDestMode {
    fn into(self) -> u64 {
        self as u64
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum IpiDeliveryMode {
    /// Delivers to the processors specified in the vector field.
    Fixed = 0b000,
    /// Same as fixed, except interrupt is delivered to the processor with the
    /// lowest priority.
    LowestPriority = 0b001,
    /// Delivers a system management interrupt to the target processors.
    SystemManagement = 0b010,
    /// Delivers a non-maskable interrupt to the target processors.
    NonMaskable = 0b100,
    /// Delivers an INIT interrupt to the target processor(s).
    Init = 0b101,
    /// Delivers a start-up IPI to the target processor(s).
    StartUp = 0b110,
}

impl Into<u64> for IpiDeliveryMode {
    fn into(self) -> u64 {
        self as u64
    }
}

/// Specifies the destination when calling `send_ipi_all`.
#[derive(Debug)]
#[repr(u8)]
pub enum IpiAllShorthand {
    /// Send inter-processor interrupt all processors.
    AllIncludingSelf = 0b10,
    /// Send inter-processor interrupt to all processor except this one.
    AllExcludingSelf = 0b11,
}

impl Into<u64> for IpiAllShorthand {
    fn into(self) -> u64 {
        self as u64
    }
}
