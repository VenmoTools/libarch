use bitflags::bitflags;

bitflags! {
    /// Redirection table entry flags.
    pub struct IrqFlags: u32 {
        /// Level-triggered interrupt (vs edge-triggered)
        const LEVEL_TRIGGERED = 0x0000_8000;
        /// Low-polarity interrupt signal (vs high-polarity)
        const LOW_ACTIVE = 0x0000_2000;
        /// Logical destination mode (vs physical)
        const LOGICAL_DEST = 0x0000_0800;
    }
}