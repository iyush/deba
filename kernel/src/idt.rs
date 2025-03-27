use crate::{
    kprint::{inb, io_wait, outb},
    kprintln,
};
use core::arch::{asm, global_asm};

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
struct IdtEntry {
    offset_1: u16,       // offset bits 0..15
    selector: u16,       // a code segment selector in GDT or LDT
    ist: u8,             // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    type_attributes: u8, // gate type, dpl, and p fields
    offset_2: u16,       // offset bits 16..31
    offset_3: u32,       // offset bits 32..63
    zero: u32,           // reserved
}

#[repr(align(16))] // Ensures stronger alignment than needed (8-byte)
struct IdtArray {
    entries: [IdtEntry; 256],
}

static mut IDTR: Idtr = unsafe { core::mem::zeroed() };
static mut IDT: IdtArray = unsafe { core::mem::zeroed() };

global_asm!(include_str!("idt.S"));

extern "C" {
    fn int_wrapper_0(r: *mut Regs);
    fn int_wrapper_1(r: *mut Regs);
    fn int_wrapper_2(r: *mut Regs);
    fn int_wrapper_3(r: *mut Regs);
    fn int_wrapper_4(r: *mut Regs);
    fn int_wrapper_5(r: *mut Regs);
    fn int_wrapper_6(r: *mut Regs);
    fn int_wrapper_7(r: *mut Regs);
    fn int_wrapper_8(r: *mut Regs);
    fn int_wrapper_9(r: *mut Regs);
    fn int_wrapper_10(r: *mut Regs);
    fn int_wrapper_11(r: *mut Regs);
    fn int_wrapper_12(r: *mut Regs);
    fn int_wrapper_13(r: *mut Regs);
    fn int_wrapper_14(r: *mut Regs);
    fn int_wrapper_16(r: *mut Regs);
    fn int_wrapper_17(r: *mut Regs);
    fn int_wrapper_18(r: *mut Regs);
    fn int_wrapper_19(r: *mut Regs);
    fn int_wrapper_20(r: *mut Regs);
    fn int_wrapper_21(r: *mut Regs);

    fn int_wrapper_32(r: *mut Regs);
    fn int_wrapper_33(r: *mut Regs);
}

unsafe fn idt_set_handler(
    ist: u8,
    interrupt_vector: usize,
    handler_fn: unsafe extern "C" fn(*mut Regs),
    type_attribute: u8,
) {
    let entry = &mut IDT.entries[interrupt_vector];

    entry.offset_1 = ((handler_fn as u64) & 0xffff) as u16;
    entry.selector = 0x28;
    entry.ist = ist;
    entry.type_attributes = type_attribute;
    entry.offset_2 = (((handler_fn as u64) >> 16) & 0xffff) as u16;
    entry.offset_3 = (((handler_fn as u64) >> 32) & 0xffffffff) as u32;
    entry.zero = 0;
}

pub unsafe fn init() {
    // setup the interrupt descriptor table
    IDTR.limit = (core::mem::size_of::<IdtEntry>() * 256) as u16;
    IDTR.base = ((&IDT.entries[0]) as *const IdtEntry) as u64;

    idt_set_handler(0, 0, int_wrapper_0, 0x8E);
    idt_set_handler(0, 0x1, int_wrapper_1, 0x8E);
    idt_set_handler(0, 0x2, int_wrapper_2, 0x8E);
    idt_set_handler(0, 0x3, int_wrapper_3, 0x8E);
    idt_set_handler(0, 0x4, int_wrapper_4, 0x8E);
    idt_set_handler(0, 0x5, int_wrapper_5, 0x8E);
    idt_set_handler(0, 0x6, int_wrapper_6, 0x8E);
    idt_set_handler(0, 0x7, int_wrapper_7, 0x8E);
    idt_set_handler(0, 0x8, int_wrapper_8, 0x8F); //
    idt_set_handler(0, 0x9, int_wrapper_9, 0x8E);
    idt_set_handler(0, 0xa, int_wrapper_10, 0x8F); //
    idt_set_handler(0, 0xb, int_wrapper_11, 0x8F); //
    idt_set_handler(0, 0xc, int_wrapper_12, 0x8F); //
    idt_set_handler(0, 0xd, int_wrapper_13, 0x8F); //
    idt_set_handler(0, 0xe, int_wrapper_14, 0x8F); //
    idt_set_handler(0, 0xf, int_wrapper_16, 0x8E);
    idt_set_handler(0, 0x10, int_wrapper_17, 0x8F); //
    idt_set_handler(0, 0x11, int_wrapper_18, 0x8E);
    idt_set_handler(0, 0x12, int_wrapper_19, 0x8E);
    idt_set_handler(0, 0x13, int_wrapper_20, 0x8E);
    idt_set_handler(0, 0x14, int_wrapper_21, 0x8E);

    asm!(
        "lidt [{}]",
        in(reg) &raw const IDTR,
        options(readonly, nostack, preserves_flags)
    );

    init_pic();
    asm!("sti"); // set the interrupt flag

    let mut flags_reg: u64;
    asm!("pushf", "pop rax", "and rax, 0x200", out("rax") flags_reg);

    if flags_reg > 0 {
        kprintln!("Interrupts are enabled");
    } else {
        kprintln!("Interrupts are still disabled");
    }
}

#[derive(Debug)]
#[repr(C, packed)]
struct Regs {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,

    rflags: u64,
    interrupt_number: u64,
    error_code: u64,
    rip: u64,
}

fn timer(_r: &mut Regs) {
    // kprintln!("Inside timer");
    pic_send_end_of_interrupt();
}

#[no_mangle]
unsafe extern "C" fn all_interrupts_handler(r: *mut Regs) {
    let regs: &mut Regs = &mut *r;
    match regs.interrupt_number {
        13 => {
            kprintln!("We got a General Protection Fault!");
            kprintln!("{:?}", regs);
            loop {}
        }
        14 => {
            kprintln!("We got a page fault!");
            let mut cr2: u64;
            asm!(
                "mov {0}, cr2",
                out(reg) cr2
            );
            kprintln!("Page fault address: {:x}", cr2);
            kprintln!("{:#x?}", regs);
            loop {}
        }
        32 => {
            timer(regs);
            // break;
        }
        33 => {
            //TODO: keyboard
            // break;
        }
        99 => {
            match regs.rax {
                0 => {
                    // cleanup();
                    // scheduler_cleanup_task();
                }
                1 => {
                    // output_to_console((char*)r->rdi, r->rsi);
                }
                _ => {
                    kprintln!("syscall number not recognized!");
                    loop {}
                }
            }
        }
        _ => {
            let int_number = regs.interrupt_number;
            kprintln!("we received an generic interrupt {}\n", int_number);
            kprintln!("{:?}", regs);
            loop {}
        }
    };
}

pub unsafe fn init_pic() {
    // we need to offset the pic interrupts, as they will overlap over the
    // software interrupts we defined above.
    pic_remap(0x20, 0x28);

    // setup hardware interrupt handlers
    idt_set_handler(1, 0x20, int_wrapper_32, 0x8E);
    idt_set_handler(0, 0x21, int_wrapper_33, 0x8E);

    for vector in 0x22..0x30 {
        idt_set_handler(0, vector, all_interrupts_handler, 0x8E);
    }

    // disable/mask all the hardware interrupts right now.
    // until we implement keyboard drivers.
    // pic_disable_all_interrupts(); // here masking means disabling
}

const PIC_MASTER_COMMAND: i16 = 0x0020;
const PIC_MASTER_DATA: i16 = 0x0021;
const PIC_SLAVE_COMMAND: i16 = 0x00A0;
const PIC_SLAVE_DATA: i16 = 0x00A1;

struct ICW {}

#[allow(dead_code)]
impl ICW {
    const ICW1_IC4: i8 = 1 << 0; // if this is set - icw4 has to be read. if icw4 is not needed
                                 // set ic4=0

    const ICW1_SNGL: i8 = 1 << 1; // Means that this is the only 8259A in the system. If SNGL = 1,
                                  // no ICW3 will be issued. We have 2 PICs in the system.

    const ICW1_ADI: i8 = 1 << 2; // Call address interval, if ADI = 1, then interval = 4, if ADI == 0,
                                 // then interval = 8. This is usually ignored by x86.

    const ICW1_LTIM: i8 = 1 << 3; // If LTIM = 1, then the 8259A will operate in
                                  // the level interrupt mode. Edge detect logic
                                  // on the interrupt inputs will be disabled. By default we are in
                                  // edge triggered mode, so dont use this.

    const ICW1_INIT: i8 = 1 << 4; // Initialization

    const ICW4_UPM: i8 = 1 << 0; // if 1, it is in 80x86 mode, else it is in MCS-80/85 mode

    const ICW4_AEOI: i8 = 1 << 1; // if 1, on the last interrupt acknowledge signal,
                                  // PIC automatically performs End of Interrupt (EOI) operation

    const ICW4_MS: i8 = 1 << 2; // Only use if ICW4_BUF is set. If 1, selects buffer master. if 0, buffer slave.

    const ICW4_BUF: i8 = 1 << 3; // If 1, controller operates in buffered mode.

    const ICW4_SFNM: i8 = 1 << 4; // Special Fully Nested Mode. Used in systems with a large amount of cascaded controllers.
}

unsafe fn pic_remap(master_offset: i8, slave_offset: i8) {
    // save the current state or "masks"
    let master_data: i8 = inb(PIC_MASTER_DATA);
    let slave_data: i8 = inb(PIC_SLAVE_DATA);

    // start the initialization with cascade mode.
    outb(PIC_MASTER_COMMAND, ICW::ICW1_INIT | ICW::ICW1_IC4);
    io_wait();
    outb(PIC_SLAVE_COMMAND, ICW::ICW1_INIT | ICW::ICW1_IC4);
    io_wait();

    // master and slave pic vector offset
    outb(PIC_MASTER_DATA, master_offset);
    io_wait();
    outb(PIC_SLAVE_DATA, slave_offset);
    io_wait();

    // The 8086 architecture uses IRQ line 2 to connect the master PIC to the slave PIC
    // IRQ line 2 is specified in master by 0b00000100
    outb(PIC_MASTER_DATA, 1 << 2);
    io_wait(); // tell the master PIC slave's PIC at IR2.
               // IRQ line 2 is specified in slave by 0b00000010
    outb(PIC_SLAVE_DATA, 1 << 1);
    io_wait(); // tell the slave its cascade identity

    // the pics should use the 8086 mode.
    outb(PIC_MASTER_DATA, ICW::ICW4_UPM);
    io_wait();
    outb(PIC_SLAVE_DATA, ICW::ICW4_UPM);
    io_wait();

    // restore the saved state or 'masks'
    outb(PIC_MASTER_DATA, master_data);
    io_wait();
    outb(PIC_SLAVE_DATA, slave_data);
    io_wait();

    // Important! Limine masks these interrupts by default, we would have to unmask them.
    outb(PIC_MASTER_DATA, inb(PIC_MASTER_DATA) & !(1 << 0)); // Unmask IRQ 0 (Timer)
    outb(PIC_MASTER_DATA, inb(PIC_MASTER_DATA) & !(1 << 1)); // Unmask IRQ 1 (Keyboard)
}

// 'masking' here means disabling by setting the bit to be 1.
#[allow(dead_code)]
fn pic_disable_all_interrupts() {
    outb(PIC_MASTER_DATA, 0xff as u8 as i8);
    outb(PIC_SLAVE_DATA, 0xff as u8 as i8);
}

fn pic_send_end_of_interrupt() {
    // set bit 5 of OCW 2
    outb(PIC_MASTER_COMMAND, 1 << 5);
}
