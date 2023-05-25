use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;

// Create a Task State Segment. 
// Contains the Privilege Stack Table (used for switching between privilege levels)
// Contains the Interrupt Stack Table (used for switching stacks on interrupt)
// Contains the I/O Map Base Address Field, which indicates the offset from the TSS base to the I/O Permission Bit Map
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Define a stack for the double fault by writing the top address of the stack to the IST
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK} );
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::gdt::SegmentSelector;

// Create the Global Descriptor Table, which describes memory segments to the CPU
// In 64-bit mode it only stores:
// The Null Descriptor (typically unused, required to be null per Intel docs)
// Kernel Mode Code Segment
// Kernel Mode Data Segment
// User Mode Code Segment
// User Mode Data Segment
// Task State Segment
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); // Create a Kernel Mode Code Segment descriptor
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); // Create Task State Segment descriptor
        (gdt, Selectors{ code_selector, tss_selector })
    };
}
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    
    // TODO move this in to static init
    let gdt: &GlobalDescriptorTable = &GDT.0;
    let selectors: &Selectors = &GDT.1;

    gdt.load(); // Load the GDT
    unsafe {
        // Update the Code Segment Selector register to point to the Kernel Mode Code Segment descriptor we initialized
        CS::set_reg(selectors.code_selector);
        // Load the TSS using the LTR instruction
        load_tss(selectors.tss_selector);
    }
}