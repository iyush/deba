#![no_std]
#![no_main]
#![feature(format_args_nl)]

use core::arch::asm;

mod asa_limine;
mod cpu;
mod elf;
mod gdt;
mod idt;
mod kprint;
mod pmm;

static mut KERNEL_STACK_BASE: [u8; 16384] = unsafe { core::mem::zeroed() };
static mut INTERRUPT_STACK_BASE: [u8; 16384] = unsafe { core::mem::zeroed() };

struct ProcessorContext {
    user_stack_ptr: u64,
    kernel_stack_ptr: u64,
}

static mut PROCESSOR_CONTEXT: ProcessorContext = unsafe { core::mem::zeroed() };

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(asa_limine::BASE_REVISION.is_supported());

    let kernel_stack_ptr: *const u8 = KERNEL_STACK_BASE.as_ptr().byte_add(KERNEL_STACK_BASE.len());
    asm!("mov rsp, {}", in(reg) kernel_stack_ptr as u64);
    let interrupt_stack_ptr: *const u8 = INTERRUPT_STACK_BASE
        .as_ptr()
        .byte_add(INTERRUPT_STACK_BASE.len());

    gdt::init(kernel_stack_ptr as u64, interrupt_stack_ptr as u64);
    idt::init();
    let _ = pmm::init(&asa_limine::MEMMAP_REQUEST, &asa_limine::HHDM_REQUEST);

    // we are enabling fast syscall in the processor.
    cpu::wrmsr(
        cpu::Msr::IA32_EFER,
        cpu::rdmsr(cpu::Msr::IA32_EFER) | ((1 as u64) << 0),
    );
    cpu::wrmsr(cpu::Msr::IA32_FSTAR, 0x43700); // Clear IF,TF,AC, and DF
                                               // kprintln!("cpu::Msr::IA32_FSTAR {:#x}", cpu::rdmsr(cpu::Msr::IA32_FSTAR));

    // this is syscall entry function
    // cpu::wrmsr(cpu::Msr::IA32_LSTAR, (u64) & syscall_handler_wrapper);

    cpu::wrmsr(cpu::Msr::IA32_STAR, 0x0030002800000000);
    // kprintln!("cpu::Msr::IA32_STAR {:#x}", cpu::rdmsr(cpu::Msr::IA32_STAR));

    cpu::wrmsr(cpu::Msr::IA32_KERNEL_GS_BASE, &raw const PROCESSOR_CONTEXT as *const ProcessorContext as u64);
    cpu::wrmsr(cpu::Msr::IA32_USER_GS_BASE,  &raw const PROCESSOR_CONTEXT as *const ProcessorContext as u64);


    let current_page_table_address: &u64 = cpu::cr3().to_higher_half_ptr();
    kprintln!("current_page_table_address: {:p}", current_page_table_address);

    /*
    struct limine_file ** modules = ctx_get_modules();
    Elf64 program_elf = elf_parse(modules[0]->address, modules[0]->size);

    scheduler_init();

    for (int i = 0; i < 100; i++) {
        u64 argc = 3;
        char* argv[] = {"hello-world", "hello darkness", "1000"};
        Task task = task_init(&pmm_allocator, (PageTableEntry*) current_page_table_address, program_elf, argc, argv);
        scheduler_queue_task(task);
    }

    scheduler_idle_loop();
    */

    if let Some(framebuffer_response) = asa_limine::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                // Calculate the pixel offset using the framebuffer information we obtained above.
                // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
                *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("PANIC! {}", info);
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            asm!("nop");
        }
    }
}

#[allow(dead_code)]
unsafe fn bochs_breakpoint() {
    asm!("xchg bx, bx", options(nomem, nostack))
}
