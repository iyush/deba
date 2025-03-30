#![allow(dead_code)]
// use core::assert;
use core::slice;

type Elf64Off = u64;
type Elf64Addr = u64;
type Elf64Xword = u64;
type Elf64Half = u16;
type Elf64Word = u32;

#[derive(Debug)]
#[repr(u16)]
enum EIdent {
    EiMag0 = 0,       // File identification
    EiMag1 = 1,       //
    EiMag2 = 2,       //
    EiMag3 = 3,       //
    EiClass = 4,      //File class
    EiData = 5,       // Data encoding
    EiVersion = 6,    // File version
    EiOsabi = 7,      // OS/ABI identification
    EiAbiversion = 8, // ABI version
    EiPad = 9,        //Start of padding bytes
    EiNident = 16,    //Size of e_ident[]
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u16)]
enum ElfType {
    EtNone = 0,        // No file type
    EtRel = 1,         // Relocatable object file
    EtExec = 2,        // Executable file
    EtDyn = 3,         // Shared object file
    EtCore = 4,        // Core file
    EtLoos = 0xFE00,   // Environment-specific use
    EtHios = 0xFEFF,   //
    EtLoproc = 0xFF00, // Processor-specific use
    EtHiproc = 0xFFFF, //
}

#[derive(Debug)]
enum ElfClass {
    Elfclass32 = 1,    // 32-bit objects
    Elfclass64 = 2,    // 64-bit objects
    Elfclass288 = 288, // 64-bit objects
}

#[derive(Debug)]
enum ElfData {
    Elfdata2Lsb = 1, // Object file data structures are little-endian
    Elfdata2Msb = 2, // Object file data structures are big-endian
}

#[derive(Debug)]
enum ElfOSAbi {
    ElfosabiSysv = 0,         // System V ABI
    ElfosabiHpux = 1,         // HP-UX operating system
    ElfosabiStandalone = 255, // Standalone (embedded)
}

// ph_type
#[derive(Debug)]
#[repr(u32)]
enum PhType {
    PtNull = 0,              // Unused entry
    PtLoad = 1,              // Loadable segment
    PtDynamic = 2,           // Dynamic linking tables
    PtInterp = 3,            // Program interpreter path name
    PtNote = 4,              // Note sections
    PtShlib = 5,             // Reserved
    PtPhdr = 6,              // Program header table
    PtLoos = 0x60000000,     // Environment-specific use
    PtHios = 0x6FFFFFFF,     //
    PtLoproc = 0x70000000,   // Processor-specific use
    PtHiproc = 0x7FFFFFFF,   //
    PtGnuRelro = 0x6474e552, // some gnu extension
}

// p_flags
#[derive(Debug)]
#[repr(u32)]
enum Pflags {
    PfX = 0x1,               // Execute permission
    PfW = 0x2,               // Write permission
    PfR = 0x4,               // Read permission
    PfMaskos = 0x00FF0000,   // These flag bits are reserved for environment specific use
    PfMaskproc = 0xFF000000, // These flag bits are reserved for processor specific use
}

// Section Types, sh_type
#[derive(Debug)]
#[repr(u32)]
enum ShType {
    ShtNull = 0,            // Marks an unused section header
    ShtProgbits = 1,        // Contains information defined by the program
    ShtSymtab = 2,          // Contains a linker symbol table
    ShtStrtab = 3,          // Contains a string table
    ShtRela = 4,            // Contains “Rela” type relocation entries
    ShtHash = 5,            // Contains a symbol hash table
    ShtDynamic = 6,         // Contains dynamic linking tables
    ShtNote = 7,            // Contains note information
    ShtNobits = 8,          // Contains uninitialized space; does not occupy any space in the file
    ShtRel = 9,             // Contains “Rel” type relocation entries
    ShtShlib = 10,          // Reserved
    ShtDynsym = 11,         // Contains a dynamic loader symbol table
    ShtLoos = 0x60000000,   // Environment-specific use
    ShtHios = 0x6FFFFFFF,   //
    ShtLoproc = 0x70000000, // Processor-specific use
    ShtHiproc = 0x7FFFFFFF, //
}

// Table 9. Section Attributes, sh_flags
#[derive(Debug)]
#[repr(u32)]
enum ShFlags {
    ShfWrite = 0x1,           // Section contains writable data
    ShfAlloc = 0x2,           // Section is allocated in memory image of program
    ShfExecinstr = 0x4,       // Section contains executable instructions
    ShfMaskos = 0x0F000000,   // Environment-specific use
    ShfMaskproc = 0xF0000000, // Processor-specific use
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
struct Elf64Ehdr {
    e_ident: [u8; EIdent::EiNident as usize], /* ELF identification */
    e_type: ElfType,                          /* Object file type */
    e_machine: Elf64Half,                     /* Machine type */
    e_version: Elf64Word,                     /* Object file version */
    e_entry: Elf64Addr,                       /* Entry point address */
    e_phoff: Elf64Off,                        /* Program header offset */
    e_shoff: Elf64Off,                        /* Section header offset */
    e_flags: Elf64Word,                       /* Processor-specific flags */
    e_ehsize: Elf64Half,                      /* ELF header size */
    e_phentsize: Elf64Half,                   /* Size of program header entry */
    e_phnum: Elf64Half,                       /* Number of program header entries */
    e_shentsize: Elf64Half,                   /* Size of section header entry */
    e_shnum: Elf64Half,                       /* Number of section header entries */
    e_shstrndx: Elf64Half,                    /* Section name string table index */
}

#[derive(Debug)]
#[repr(C, packed)]
struct Elf64Shdr {
    sh_name: Elf64Word,       /* Section name */
    sh_type: Elf64Word,       /* Section type */
    sh_flags: Elf64Xword,     /* Section attributes */
    sh_addr: Elf64Addr,       /* Virtual address in memory */
    sh_offset: Elf64Off,      /* Offset in file */
    sh_size: Elf64Xword,      /* Size of section */
    sh_link: Elf64Word,       /* Link to other section */
    sh_info: Elf64Word,       /* Miscellaneous information */
    sh_addralign: Elf64Xword, /* Address alignment boundary */
    sh_entsize: Elf64Xword,   /* Size of entries, if section has table */
}

#[derive(Debug)]
#[repr(C, packed)]
struct Elf64Phdr {
    p_type: Elf64Word,    /* Type of segment */
    p_flags: Elf64Word,   /* Segment attributes */
    p_offset: Elf64Off,   /* Offset in file */
    p_vaddr: Elf64Addr,   /* Virtual address in memory */
    p_paddr: Elf64Addr,   /* Reserved */
    p_filesz: Elf64Xword, /* Size of segment in file */
    p_memsz: Elf64Xword,  /* Size of segment in memory */
    p_align: Elf64Xword,  /* Alignment of segment */
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Elf64<'a> {
    header: Elf64Ehdr,
    sheaders: &'a [Elf64Shdr],
    pheaders: &'a [Elf64Phdr],
}

pub fn parse<'a>(buf: *mut u8, len: u64) -> Elf64<'a> {
    let len = len as usize;
    let buf = unsafe { slice::from_raw_parts(buf, len) };
    
    let start = buf;
    let current = start;

    let sizeof_elf_header = core::mem::size_of::<Elf64Ehdr>();
    assert!(len > sizeof_elf_header, "Not an ELF File (length check)!");

    let (header_bytes, _) = current.split_at(sizeof_elf_header);
    let header = unsafe { (header_bytes.as_ptr() as *const Elf64Ehdr).read_unaligned() };

    assert!(
        header.e_ident[EIdent::EiMag0 as usize] == b'\x7f'
            && header.e_ident[EIdent::EiMag1 as usize] == b'E'
            && header.e_ident[EIdent::EiMag2 as usize] == b'L'
            && header.e_ident[EIdent::EiMag3 as usize] == b'F',
        "Not an ELF file (header check)!"
    );
    assert!(
        header.e_ident[EIdent::EiClass as usize] == ElfClass::Elfclass64 as u8,
        "only ELFCLASS64 supported!"
    );
    assert!(
        header.e_ident[EIdent::EiData as usize] == ElfData::Elfdata2Lsb as u8,
        "only ELFDATA2LSB supported!"
    );
    assert!(
        header.e_ident[EIdent::EiOsabi as usize] == ElfOSAbi::ElfosabiSysv as u8,
        "only ELFOSABI_SYSV supported!"
    );

    let e_type = header.e_type;
    assert!(
        e_type == ElfType::EtExec || e_type == ElfType::EtDyn,
        "we only support ET_EXEC or ET_DYN for now!"
    );
    assert!(header.e_phoff > 0, "we expect ph_off to be present!");
    assert!(header.e_phnum > 0, "we expect ph_num to be present!");
    assert!(header.e_shoff > 0, "we expect sh_off to be present!");
    assert!(header.e_shnum > 0, "we expect sh_num to be present!");
    assert!(
        header.e_phentsize as usize == core::mem::size_of::<Elf64Phdr>(),
        "Elf64_Phdr is not of correct size as in the elf file!"
    );
    assert!(
        header.e_shentsize as usize == core::mem::size_of::<Elf64Shdr>(),
        "Elf64_Shdr is not of correct size as in the elf file!"
    );

    let (_, pheader_and_rest) = current.split_at(header.e_phoff as usize);
    let pheader = pheader_and_rest.as_ptr() as *const Elf64Phdr;
    let pheaders: &[Elf64Phdr] = unsafe { slice::from_raw_parts(pheader, header.e_phnum as usize) };

    let (_, sheader_and_rest) = current.split_at(header.e_shoff as usize);
    let sheader = sheader_and_rest.as_ptr() as *const Elf64Shdr;
    let sheaders: &[Elf64Shdr] = unsafe { slice::from_raw_parts(sheader, header.e_shnum as usize) };


    // Elf64_Shdr* shstr = sheaders + header.e_shstrndx; // get the sheader at the index specified in the header.
    // u64 shstroffset = (u64)start + shstr->sh_offset; // get the section offset and add it the start.

    // for (int i = 0; i < header.e_shnum; i++) {
    //     Elf64_Shdr sheader = sheaders[i];
    //     u8* name = (u8*)(shstroffset + sheader.sh_name);

    //     // printf("(%d/%d) section:[%s] type: 0x%x name: 0x%x flags: 0x%lx virtual: 0x%lx sh_offset: 0x%lx sh_link: 0x%x\n",
    //     //  i,
    //     //  header.e_shnum,
    //     //  name,
    //     //  sheader.sh_type,
    //     //  sheader.sh_name,
    //     //  sheader.sh_flags,
    //     //  sheader.sh_addr,
    //     //  sheader.sh_offset,
    //     //  sheader.sh_link
    //     //  );
    // }

    // printf("diff: 0x%lx\n", (u64) (header_end - start) );
    // printf("e_entry: 0x%lx\n", header.e_entry );
    // printf("e_shoff: 0x%lx\n", header.e_shoff );
    // printf("e_phoff: 0x%lx\n", header.e_phoff );

    /*
    result.header = header;
    result.elf_module_start = (u64) buf;
    result.s_headers = sheaders;
    result.s_headers_len = header.e_phnum;
    result.p_headers = pheaders;
    result.p_headers_len = header.e_shnum;
     */
    Elf64 {
	header,
	pheaders,
	sheaders
    }
}
