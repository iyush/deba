use crate::asa_limine;
use crate::kprintln;
use core::slice;
use limine;
// use core::error::Error;

pub const FRAME_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Frame {
    phy_ptr: u64,
    size: usize,
}

#[allow(dead_code)]
impl Frame {
    pub fn from_u64(ptr: u64, size: usize) -> Frame {
        return Frame { phy_ptr: ptr, size };
    }

    pub fn to_higher_half_ptr<'a, T>(self) -> &'a T {
        let hddm = asa_limine::HHDM_REQUEST.get_response().unwrap().offset();

        let virt_ptr: *const T = (self.phy_ptr + hddm) as *const T;

        unsafe { &(*virt_ptr) }
    }

    pub fn to_higher_half_slice<'a, T>(self) -> &'a [T] {
        let hddm = asa_limine::HHDM_REQUEST.get_response().unwrap().offset();
        let virt_ptr: *const T = (self.phy_ptr + hddm) as *const T;
        unsafe { slice::from_raw_parts(virt_ptr, self.size / core::mem::size_of::<T>()) }
    }

    pub fn to_higher_half_slice_mut<'a, T>(self) -> &'a mut [T] {
        let hddm = asa_limine::HHDM_REQUEST.get_response().unwrap().offset();
        let virt_ptr: *mut T = (self.phy_ptr + hddm) as *mut T;
        unsafe { slice::from_raw_parts_mut(virt_ptr, self.size / core::mem::size_of::<T>()) }
    }
}

pub struct Pmm {
    bitmap: &'static mut [u64],
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PmmAllocError;

#[allow(dead_code)]
impl Pmm {
    fn set_free(&mut self, base: Frame) {
        let frame_start = base.phy_ptr as usize / FRAME_SIZE;
        let n_frames = base.size  / FRAME_SIZE;

        kprintln!(
            "n_frames: {} phy_ptr: {:#x} bmp_area: {:#x}",
            n_frames,
            base.phy_ptr,
            self.bitmap.len() * 64
        );
        for i in 0..n_frames {
            let frame = frame_start + i;
            let frame_big_idx = frame / 64;
            let frame_sma_idx = frame % 64;


	    kprintln!(
		"bitmap: {:#b}",
		self.bitmap[frame_big_idx as usize],
	    );

            self.bitmap[frame_big_idx as usize] &= !((1 as u64) << frame_sma_idx);
        }
    }

    fn find_free_frame(&self, n_frames: usize) -> Option<Frame> {
        if n_frames == 0 {
            return None;
        }

        let mut zero_count = 0;

        for i in 0..self.bitmap.len() {
            let super_frame = self.bitmap[i];
            if super_frame != 0xffffffffffffffff {
                for j in 0..64 {
                    if ((super_frame >> j) & 1) == 0 {
                        zero_count += 1;
                    } else {
                        zero_count = 0;
                    }

                    if zero_count >= n_frames {

                        let zero_idx: usize = (i * 64 + j) - (n_frames - 1);
			let zero_idx = zero_idx as u64;
			
                        return Some(Frame::from_u64(
                            zero_idx * FRAME_SIZE as u64,
                            (n_frames * FRAME_SIZE) as usize,
                        ));
                    }
                }
            } else {
                zero_count = 0;
            }
        }
        return None;
    }

    fn set_used(&mut self, frame_ptr: &Frame) {
        let frame_start = frame_ptr.phy_ptr / FRAME_SIZE as u64;
        let n_frames = frame_ptr.size as usize / FRAME_SIZE;

        for i in 0..n_frames {
            let frame = frame_start as usize + i;
            let frame_big_index = frame / 64;
            let frame_sma_index = frame % 64;

            self.bitmap[frame_big_index as usize] |= (1 as u64) << frame_sma_index;
        }
    }

    pub fn alloc_frame(&mut self, n_frames: usize) -> Result<Frame, PmmAllocError> {
        match self.find_free_frame(n_frames) {
            None => Err(PmmAllocError {}),
            Some(frame) => {
                self.set_used(&frame);
                Ok(frame)
            }
        }
    }

    pub fn dealloc_frame(&mut self, frame_ptr: Frame) {
        self.set_free(frame_ptr);
    }
}

pub fn init(
    memmap_request: &limine::request::MemoryMapRequest,
    hhdm_request: &limine::request::HhdmRequest,
) -> Pmm {
    let mmap_response = memmap_request.get_response().unwrap();
    let entries = mmap_response.entries();
    print_mmap(entries);

    let mut biggest_usable_base = 0;
    let mut biggest_usable_length = 0;
    let mut highest_frame_top = 0;

    for entry in entries {
        use limine::memory_map::EntryType;
        match entry.entry_type {
            EntryType::USABLE => {
                if entry.length > biggest_usable_length {
                    biggest_usable_base = entry.base;
                    biggest_usable_length = entry.length;
                }

                let frame_top = entry.base + entry.length;
                if frame_top > highest_frame_top {
                    highest_frame_top = frame_top;
                }
            }
            _ => continue,
        }
    }

    let hhdm = hhdm_request.get_response().unwrap();
    let bmp_base = biggest_usable_base + hhdm.offset();

    let total_frames: usize = (highest_frame_top as usize / FRAME_SIZE) + 1; // Total number of frames

    let bmp_len: usize = ((total_frames + 63) / 64).try_into().unwrap();
    let bmp: &'static mut [u64] =
        unsafe { slice::from_raw_parts_mut(bmp_base as *mut u64, bmp_len) };

    kprintln!(
	"highest_frame_top: {:#x} biggest_usable_base: {:#x} biggest_usable_len: {:#x} bmp_len: {:#x}",
	highest_frame_top,
	biggest_usable_base,
	biggest_usable_length,
	bmp_len * 64
    );

    bmp.fill(0xffff_ffff_ffff_ffff);

    let mut pmm = Pmm { bitmap: bmp };

    for entry in entries {
        use core::mem::size_of;
        use limine::memory_map::EntryType;

        match entry.entry_type {
            EntryType::USABLE => {
                let base = entry.base;
                let length = entry.length as usize;

                if base == biggest_usable_base {
                    pmm.set_free(Frame::from_u64(
                        base + (bmp_len as u64) * (size_of::<u64>() as u64),
                        (length - (pmm.bitmap.len() * 64)) / FRAME_SIZE as usize,
                    ));
                } else {
                    pmm.set_free(Frame::from_u64(
                        base,
                        (length / FRAME_SIZE).try_into().unwrap(),
                    ));
                }
            }
            _ => continue,
        }
    }

    // for null frame
    pmm.bitmap[0] = pmm.bitmap[0] | 1;
    kprintln!("{:b}", pmm.bitmap[0]);
    return pmm;
}

fn print_mmap(entries: &[&limine::memory_map::Entry]) {
    for entry in entries {
        use limine::memory_map::EntryType;
        match entry.entry_type {
            EntryType::USABLE => {
                kprintln!("{:#x} {:#x} USABLE", entry.base, entry.length);
            }
            EntryType::RESERVED => {
                kprintln!("{:#x} {:#x} RESERVED", entry.base, entry.length);
            }
            EntryType::ACPI_RECLAIMABLE => {
                kprintln!("{:#x} {:#x} ACPI_RECLAIMABLE ", entry.base, entry.length);
            }
            EntryType::ACPI_NVS => {
                kprintln!("{:#x} {:#x} ACPI_NVS ", entry.base, entry.length);
            }
            EntryType::BAD_MEMORY => {
                kprintln!("{:#x} {:#x} BAD_MEMORY", entry.base, entry.length);
            }
            EntryType::BOOTLOADER_RECLAIMABLE => {
                kprintln!(
                    "{:#x} {:#x} BOOTLOADER_RECLAIMABLE",
                    entry.base,
                    entry.length
                );
            }
            EntryType::KERNEL_AND_MODULES => {
                kprintln!("{:#x} {:#x} KERNEL_AND_MODULES", entry.base, entry.length);
            }
            EntryType::FRAMEBUFFER => {
                kprintln!("{:#x} {:#x} FRAMEBUFFER", entry.base, entry.length);
            }

            _ => {
                todo!("Memory type is unidentifiable!");
            }
        }
    }
}
