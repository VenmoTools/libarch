///! 封装关于cs tss等段操作指令
use crate::arch::intel::Selector;

/// 加载cs段选择子
/// 在这里没有直接使用mov指令加载到cs寄存器，把新的选择子
/// 压到栈中，并且使用lretq重新加载cs寄存器，并在1:处继续
pub unsafe fn set_cs<S: Selector>(selector: S) {
    #[inline(always)]
    unsafe fn inner(selector: u64) {
        llvm_asm!(
            "pushq $0;\
            leaq 1f(%rip), %rax;\
            pushq %rax;\
            lretq;\
            1:"
            :
            : "ri"(selector)
            : "rax" "memory"
        );
    }
    inner(selector.as_u64());
}

/// 加载ss段选择子
pub unsafe fn load_ss<S: Selector>(selector: S) {
    llvm_asm!(
        "movw $0, %ss"
        :
        : "r"(selector.as_u16())
        :"memory"
    );
}

/// 加载ds段选择子
pub unsafe fn load_ds<S: Selector>(selector: S) {
    llvm_asm!(
        "movw $0,%ds"
        :
        :"r"(selector.as_u16())
        :"memory"
    );
}

/// 加载es段选择子
pub unsafe fn load_es<S: Selector>(selector: S) {
    llvm_asm!(
        "movw $0,%es"
        :
        :"r"(selector.as_u16())
        :"memory"
    );
}

/// 加载fs段选择子
pub unsafe fn load_fs<S: Selector>(selector: S) {
    llvm_asm!(
        "movw $0, %fs"
        :
        :"r"(selector.as_u16())
        :"memory"
    );
}

/// 加载gs段选择子
pub unsafe fn load_gs<S: Selector>(selector: S) {
    llvm_asm!(
        "movw $0, %gs"
        :
        :"r"(selector.as_u16())
        :"memory"
    );
}

/// swapgs指令
pub unsafe fn swap_gs() {
    llvm_asm!(
        "swapgs"
        :
        :
        :"memory"
        :"volatile"
    );
}

/// 获取当前的代码段选择子
/// 获取失败的时候会返回0
#[allow(unused_assignments)]
pub fn cs<S: Selector>() -> S {
    let mut segment: u16 = 0;

    unsafe {
        llvm_asm!(
            "mov %cs, $0"
            :"=r"(segment)
        );
    }
    S::from(segment)
}
