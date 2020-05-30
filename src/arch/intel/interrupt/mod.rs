use alloc::string::String;

use crate::arch::intel::interrupt::x2apic::register::{IpiDestMode, TimerDivide, TimerMode};

pub mod xapic;
pub mod x2apic;
pub mod flags;
pub mod controller;
pub mod pic;

#[doc(hidden)]
macro_rules! set_attr {

    ($name:ident,$attr:ident,$args:ty) => {
        pub fn $name(&mut self, arg: $args) -> &mut Self {
            self.$attr = Some(arg);
            self
        }
    };
}

#[derive(Copy, Clone, Default)]
pub struct ApicInfo {
    // for x2apic
    pub timer_vector: Option<usize>,
    pub error_vector: Option<usize>,
    pub spurious_vector: Option<usize>,
    pub timer_mode: Option<TimerMode>,
    pub timer_divide: Option<TimerDivide>,
    pub timer_initial: Option<u32>,
    pub ipi_destination_mode: Option<IpiDestMode>,
    // for io apic
    pub ioapic_offset: Option<u8>,
    // for 8259
}

impl ApicInfo {
    /// Returns a new local APIC builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets io apic offset
    /// This field is required.
    set_attr!(set_io_apic_offset,ioapic_offset,u8);

    /// Sets the interrupt index of timer interrupts.
    ///
    /// This field is required.
    set_attr!(set_timer_vector,timer_vector,usize);


    /// Sets the interrupt index for internal APIC errors.
    ///
    /// This field is required.
    set_attr!(set_error_vector,error_vector,usize);


    /// Sets the interrupt index for spurious interrupts.
    ///
    /// This field is required.
    set_attr!(set_spurious_vector,spurious_vector,usize);


    /// Sets the timer mode.
    ///
    /// Default: Periodic.
    set_attr!(set_timer_mode,timer_mode,TimerMode);


    /// Sets the timer divide configuration.
    ///
    /// Default: Div256.
    set_attr!(set_timer_divide,timer_divide,TimerDivide);


    /// Sets the timer initial count.
    ///
    /// Default: 10_000_000.
    set_attr!(set_timer_initial,timer_initial,u32);


    /// Sets the IPI destination mode.
    ///
    /// Default: Physical.
    set_attr!(set_ipi_destination_mode,ipi_destination_mode,IpiDestMode);


    /// Builds a new `LocalApic`.
    ///
    /// # Errors
    ///
    /// This function returns an error if any of the required fields are empty.
    pub fn build(self) -> Result<Self, String> {
        if cfg!(x2apic)
            && self.timer_vector.is_none()
            || self.error_vector.is_none()
            || self.spurious_vector.is_none() {
            return Err(String::from("x2apic: required field(s) empty"));
        }
        if cfg!(xapic) && self.ioapic_offset.is_none() {
            return Err(String::from("xapic: required field(s) empty"));
        }
        Ok(self)
    }
}