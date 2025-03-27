use core::arch::asm;

use crate::pmm;

pub struct Msr {}

impl Msr {
    pub const IA32_EFER: u32 = 0xC0000080;
    pub const IA32_STAR: u32 = 0xC0000081;
    pub const IA32_LSTAR: u32 = 0xC0000082;
    pub const IA32_FSTAR: u32 = 0xC0000084;
    pub const IA32_USER_GS_BASE: u32 = 0xC0000101;
    pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;
}

pub unsafe fn wrmsr(msr: u32, value: u64) {
    asm!("wrmsr",
     in("edx") (value >> 32),
     in("eax") (value & 0xffffffff),
     in("ecx") (msr),
     options(nomem, nostack)
    );

    // asm __volatile__(
    //     "wrmsr"
    //     :
    //     : "d"((u32)(value >> 32)), "a"((u32)(value & 0xffffffff)), "c"(msr)
    //     :
    //     );
}

#[allow(unused_assignments)]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let mut low: u32 = 0;
    let mut high: u32 = 0;

    asm!("rdmsr",
     out("eax") (low),
     out("edx") (high),
     in("ecx") (msr),
     options(nomem, nostack)
    );

    return ((high as u64) << 32 | low as u64).into();
}

pub unsafe fn cr3() -> pmm::Frame  {
    let mut cr3: u64 = 0;

    asm!("mov rax, cr3", out("rax")(cr3));

    return pmm::Frame::from(cr3);
}
