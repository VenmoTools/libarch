///! 用于加载GDT IDT TSS的相关指令
use crate::arch::intel::{IntelX64, Selector};
use crate::arch::intel::x64::DescriptorTablePointer;

/// 使用`lgdt`加载GDT描述符
#[inline]
pub unsafe fn ldgt(gdt: &DescriptorTablePointer<IntelX64>) {
    llvm_asm!("lgdt ($0)" :: "r" (gdt) : "memory");
}

/// 使用`sgdt`取出GDTR寄存器的数据
#[inline]
pub fn sgdt() -> DescriptorTablePointer<IntelX64> {
    let gdt = DescriptorTablePointer::empty();
    unsafe {
        llvm_asm!(
            "sgdt ($0)":"=r"(&gdt) : :"memory"
        )
    }
    gdt
}

/// 使用`sgdt`取出IDTR寄存器的数据
#[inline]
pub fn sidt() -> DescriptorTablePointer<IntelX64> {
    let idt = DescriptorTablePointer::empty();
    unsafe {
        llvm_asm!(
            "sidt ($0)":"=r"(&idt)::"memory"
        )
    }
    idt
}


/// 使用`lidt`加载IDT描述符
#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer<IntelX64>) {
    llvm_asm!("lidt ($0)" :: "r" (idt) : "memory");
}

/// 使用`ltr`加载TSS描述符
#[inline]
pub unsafe fn load_tss<T: Selector>(sel: T) {
    llvm_asm!("ltr $0" :: "r" (sel.as_u16()));
}

#[inline]
pub unsafe fn load_tr<T: Selector>(sel: T) {
    llvm_asm!("ltr $0" :: "r" (sel.as_u16()));
}