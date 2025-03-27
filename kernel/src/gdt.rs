use core::arch::asm;

// use crate::bochs_breakpoint;

static NUM_GDT_ENTRIES: usize = 10;
static NUM_TSS_ENTRIES: usize = 1;

#[repr(C, packed)]
struct GdtFull {
    gdt_entries: [GdtEntry; NUM_GDT_ENTRIES],
    // gdt_entries: [u64; NUM_GDT_ENTRIES],
    tss_entries: [TssEntry; NUM_TSS_ENTRIES],
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct TssEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,

    flags: u8,
    // u8 segment_type: 4;
    // u8 descriptor_type: 1;
    // u8 dpl: 2;
    // u8 present: 1;
    flags2: u8,
    // u8 limit_high: 4;
    // u8 avl: 1;
    // u8 l: 1;
    // u8 db: 1;
    // u8 g: 1;
    base_high: u8,
    base_top: u32,
    reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,

    flags: u8,
    // u8 segment_type: 4;
    // u8 descriptor_type: 1;
    // u8 dpl: 2;
    // u8 present: 1;
    flags2: u8,
    // u8 limit_high: 4;
    // u8 avl: 1;
    // u8 l: 1;
    // u8 db: 1;
    // u8 g: 1;
    base_high: u8,
}

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
struct Tss {
    reserved_0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    reserved_3_: u64,
    ist1: u64,
    ist2: u64,
    ist3: u64,
    ist4: u64,
    ist5: u64,
    ist6: u64,
    ist7: u64,
    reserved_2_: u64,
    reserved_1_: u16,
    i_o_map_base_address: u16,
}

static mut GDT_FULL: GdtFull = GdtFull {
    gdt_entries: [GdtEntry::default(); NUM_GDT_ENTRIES],
    tss_entries: [TssEntry::default(); NUM_TSS_ENTRIES],
};

static mut GDT_POINTER: GdtPointer = GdtPointer { limit: 0, base: 0 };
static mut TSS: Tss = unsafe { core::mem::zeroed() };

impl GdtEntry {
    // this is done solely for the reason that #[deriving(Default)] is not a comp.
    const fn default() -> GdtEntry {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            flags: 0,
            flags2: 0,
            base_high: 0,
        }
    }
}

impl TssEntry {
    const fn default() -> TssEntry {
        TssEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            flags: 0,
            flags2: 0,
            base_high: 0,
            base_top: 0,
            reserved: 0,
        }
    }
}

enum SegmentType {}
#[allow(dead_code)]
impl SegmentType {
    const DATA_READ_ONLY: u8 = 0;
    const DATA_READ_ONLY_ACCESSED: u8 = 1;
    const DATA_READ_WRITE: u8 = 2;
    const DATA_READ_WRITE_ACCESSED: u8 = 3;
    const DATA_READ_ONLY_EXPAND_DOWN: u8 = 4;
    const DATA_READ_ONLY_EXPAND_DOWN_ACCESSED: u8 = 5;
    const DATA_READ_WRITE_EXPAND_DOWN: u8 = 6;
    const DATA_READ_WRITE_EXPAND_DOWN_ACCESSED: u8 = 7;
    const CODE_EXECUTE_ONLY: u8 = 8;
    const CODE_EXECUTE_ONLY_ACCESSED: u8 = 9;
    const CODE_EXECUTE_READ: u8 = 10;
    const CODE_EXECUTE_READ_ACCESSED: u8 = 11;
    const CODE_EXECUTE_ONLY_CONFORMING: u8 = 12;
    const CODE_EXECUTE_ONLY_CONFORMING_ACCESSED: u8 = 13;
    const CODE_EXECUTE_READ_CONFORMING: u8 = 14;
    const CODE_EXECUTE_READ_CONFORMING_ACCESSED: u8 = 15;
    const TSS_64_BIT_AVAILABLE: u8 = 9;
}

enum DescriptorType {}
#[allow(dead_code)]
impl DescriptorType {
    const SYSTEM: u8 = 0;
    const CODE_OR_DATA: u8 = 1;
}

#[warn(dead_code)]
enum GdtLimitGranuality {}
#[allow(dead_code)]
impl GdtLimitGranuality {
    const GRANULARITY_BYTE: u8 = 0;
    const GRANULARITY_4KB: u8 = 1;
}

enum GdtPrivilegeLevel {}
#[allow(dead_code)]
impl GdtPrivilegeLevel {
    const PL_0: u8 = 0;
    const PL_1: u8 = 1;
    const PL_2: u8 = 2;
    const PL_3: u8 = 3;
}

enum GdtPresent {}
#[allow(dead_code)]
impl GdtPresent {
    const NOT_PRESENT: u8 = 0;
    const PRESENT: u8 = 1;
}

unsafe fn gdt_set_gate(
    gdt: *mut GdtFull,
    num: usize,
    base: u32,
    limit: u32,
    segment_type: u8,
    descriptor_type: u8,
    dpl: u8,
    granularity: u8,
) {
    (*gdt).gdt_entries[num].limit_low = (limit & 0xFFFF) as u16;
    (*gdt).gdt_entries[num].base_low = (base & 0xFFFF) as u16;
    (*gdt).gdt_entries[num].base_middle = ((base >> 16) & 0xFF) as u8;

    (*gdt).gdt_entries[num].flags = (segment_type & 0xF)
        | ((descriptor_type & 0x1) << 4)
        | ((dpl & 0b11) << 5)
        | (GdtPresent::PRESENT << 7);

    // gdt.gdt_entries[num].segment_type = segment_type;
    // gdt.gdt_entries[num].descriptor_type = descriptor_type;
    // gdt.gdt_entries[num].dpl = dpl;
    // gdt.gdt_entries[num].present = GDT_PRESENT;
    (*gdt).gdt_entries[num].flags2 = (((limit >> 16) & 0x0F) as u8)
        | (0 << 4)
        | (1 << 5)
        | (0 << 6)
        | ((granularity & 0x1) << 7);

    // gdt.gdt_entries[num].limit_high = (limit >> 16) & 0x0F;
    // gdt.gdt_entries[num].l = 1; // we will always be executing/accessing 64 bit instructions/data.
    // gdt.gdt_entries[num].db = 0; // If the L-bit is set, then the D-bit must be cleared
    // gdt.gdt_entries[num].g = granularity;
    (*gdt).gdt_entries[num].base_high = ((base >> 24) & 0xFF) as u8;
}

unsafe fn gdt_set_tss(
    gdt: *mut GdtFull,
    num: usize,
    base: u64,
    limit: u32,
    segment_type: u8,
    granularity: u8,
) {
    (*gdt).tss_entries[num].limit_low = (limit & 0xFFFF) as u16;
    (*gdt).tss_entries[num].base_low = (base & 0xFFFF) as u16;
    (*gdt).tss_entries[num].base_middle = ((base >> 16) & 0xFF) as u8;

    (*gdt).tss_entries[num].flags = (segment_type & 0xF)
        | ((DescriptorType::SYSTEM & 0x1) << 4)
        | ((GdtPrivilegeLevel::PL_0 & 0b11) << 5)
        | (GdtPresent::PRESENT << 7);

    // (*gdt).tss_entries[num].segment_type = segment_type;
    // (*gdt).tss_entries[num].descriptor_type = GDT_DESCRIPTOR_TYPE_SYSTEM;
    // (*gdt).tss_entries[num].dpl = GDT_PL_0;
    // (*gdt).tss_entries[num].present = GDT_PRESENT;

    (*gdt).tss_entries[num].flags2 = (((limit >> 16) & 0x0F) as u8)
        | (0 << 4)
        | (0 << 5)
        | (0 << 6)
        | ((granularity & 0x1) << 7);

    // (*gdt).tss_entries[num].limit_high = ((limit >> 16) & 0x0F);
    // (*gdt).tss_entries[num].avl = 0;
    // (*gdt).tss_entries[num].l = 0;
    // (*gdt).tss_entries[num].db = 0;
    // (*gdt).tss_entries[num].g = granularity;

    (*gdt).tss_entries[num].base_high = ((base >> 24) & 0xFF) as u8;
    (*gdt).tss_entries[num].base_top = ((base >> 32) & 0xFFFFFFFF) as u32;
    (*gdt).tss_entries[num].reserved = 0;
}



pub unsafe fn init(kernel_stack_ptr: u64, interrupt_stack_ptr: u64) {
    GDT_POINTER.base = (&raw const GDT_FULL) as u64;
    GDT_POINTER.limit = (core::mem::size_of::<GdtFull>() - 1) as u16;

    assert_eq!(GDT_POINTER.base & 0x7, 0, "GDT must be 8-byte aligned");

    /*
    GDT_FULL.gdt_entries[0] = 0x0000000000000000;
    GDT_FULL.gdt_entries[1] = 0x0000000000000000;
    GDT_FULL.gdt_entries[2] = 0x0000000000000000;
    GDT_FULL.gdt_entries[3] = 0x0000000000000000;
    GDT_FULL.gdt_entries[4] = 0x0000000000000000;
    GDT_FULL.gdt_entries[5] = 0x00209b0000000000;
    GDT_FULL.gdt_entries[6] = 0x0020930000000000;
    GDT_FULL.gdt_entries[7] = 0x0020f20000000000;
    GDT_FULL.gdt_entries[8] = 0x0020fa0000000000;
    */

    // Kernel Code segment
    gdt_set_gate(
        &raw mut GDT_FULL,
        5,
        0,
        0,
        SegmentType::CODE_EXECUTE_READ_ACCESSED,
     
   DescriptorType::CODE_OR_DATA,
        GdtPrivilegeLevel::PL_0,
        GdtLimitGranuality::GRANULARITY_BYTE,
    );

    // Kernel Data segment
    gdt_set_gate(
        &raw mut GDT_FULL,
        6,
        0,
        0,
        SegmentType::DATA_READ_WRITE_ACCESSED,
        DescriptorType::CODE_OR_DATA,

        GdtPrivilegeLevel::PL_0,
        GdtLimitGranuality::GRANULARITY_BYTE,
    );

    // User Code segment
    gdt_set_gate(
        &raw mut GDT_FULL,
        7,
        0,
        0,
        SegmentType::CODE_EXECUTE_READ,
        DescriptorType::CODE_OR_DATA,
        GdtPrivilegeLevel::PL_3,
        GdtLimitGranuality::GRANULARITY_BYTE,
    );

    // User Data segment
    gdt_set_gate(
        &raw mut GDT_FULL,
        8,
        0,
        0,
        SegmentType::DATA_READ_WRITE,
        DescriptorType::CODE_OR_DATA,
        GdtPrivilegeLevel::PL_3,
        GdtLimitGranuality::GRANULARITY_BYTE,
    );

    // TSS segment
    gdt_set_tss(
        &raw mut GDT_FULL,
        0,
        (&raw mut TSS) as u64,
        0x67,
        SegmentType::TSS_64_BIT_AVAILABLE,
        GdtLimitGranuality::GRANULARITY_BYTE,
    );

    TSS.rsp0 = kernel_stack_ptr;
    TSS.ist1 = interrupt_stack_ptr;

    asm!(
        "lgdt [{}]",
        in(reg) &raw const GDT_POINTER,
        options(readonly, nostack, preserves_flags)
    );

    TSS.rsp0 = kernel_stack_ptr as u64;
    asm!(
        "ltr {0:x}",
        in(reg) (NUM_GDT_ENTRIES * 8)
    );
}
