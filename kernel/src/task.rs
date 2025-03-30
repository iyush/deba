use crate::elf::Elf64;
use crate::pmm::Pmm;
use crate::kprintln;
use crate::cpu;
use core::ops::Index;

use bitvec::prelude::*;

struct RegsWithoutError {}

#[repr(transparent)]
struct PageTable<'a> {
    raw: &'a [PageTableEntry<'a>]
}

impl<'a> PageTable<'a> {
    fn new(entries: &'a mut [u64]) -> PageTable<'a> {
	assert!(entries.len() == 512);

        PageTable {
            raw: unsafe { core::mem::transmute(entries) }
        }
    }

}


#[repr(transparent)]
struct PageTableEntry<'a> {
    raw: &'a mut u64
}

impl<'a> PageTableEntry<'a> {
    fn new(entry: &'a mut u64) -> PageTableEntry<'a> {
        PageTableEntry {
            raw: entry
        }
    }

    fn is_present(&self) -> bool {
	*self.raw.view_bits::<Lsb0>().get(0).unwrap()
    }
}


impl<'a> Index<usize> for PageTable<'a> {
    type Output = PageTableEntry<'a>;
    
    fn index(&self, idx: usize) -> &Self::Output {
	&self.raw[idx]
    }
    
}

enum TaskState {
    Queued,
    Running,
    Finished,
    Paused,
}

pub struct Task<'a> {
    id: u64, // set by scheduler
    page_table_address: PageTable<'a>,
    stack_address: &'a [u64],
    entry_address: *const u8,
    state: TaskState, // set by scheduler
    regs: RegsWithoutError,
}

struct Args {}

impl<'a> Task<'a> {
    pub fn new(
        pmm: &mut Pmm,
        //_current_page_table_address: &'a PageTable<'a>,
        //_program_elf: Elf64,
        //_args: &[Args],
    ) {
        // Task<'a> {

	unsafe {
	    let current_page_table: &mut [u64] = cpu::cr3().to_higher_half_slice_mut();
	    let page_table = PageTable::new(current_page_table);
	    for i in 0..512 {
		kprintln!("{} {}", i, page_table[i].is_present());
	    }
	}

        // let page_table_address: &[u64] =
        //     pmm.alloc_frame(1).unwrap().to_higher_half_slice();
	// let page_table = PageTable::new(page_table_address);
    }
}
