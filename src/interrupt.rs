use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Mutex;
pub use x86_64::instructions::interrupts::enable as interrupt_enable;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{global::DOUBLE_FAULT_IST_INDEX, print, println};

/// Offset of primary PIC
pub const PIC_1_OFFSET: usize = 32;
/// Offset of secondary PIC
pub const PIC_2_OFFSET: usize = PIC_1_OFFSET + 8;

/// Index of interrupt
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET as u8,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    pub static ref PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET as u8, PIC_2_OFFSET as u8) });
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16)
        };

        // interrupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub fn init_idt() {
    IDT.load();
}

pub fn init_pic() {
    unsafe {
        PICS.lock().initialize();
    }
}

/// Notify that end of the given interrupt
fn end_of_interrupt(interrupt_id: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_id.as_u8());
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    end_of_interrupt(InterruptIndex::Timer);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    end_of_interrupt(InterruptIndex::Keyboard);
}

// extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
//     print!("k");

//     use x86_64::instructions::port::Port;

//     let mut port = Port::new(0x60);
//     let scancode: u8 = unsafe { port.read() };
//     print!("{}", scancode);

//     let key = match scancode {
//         0x02 => Some('1'),
//         0x03 => Some('2'),
//         0x04 => Some('3'),
//         0x05 => Some('4'),
//         0x06 => Some('5'),
//         0x07 => Some('6'),
//         0x08 => Some('7'),
//         0x09 => Some('8'),
//         0x0a => Some('9'),
//         0x0b => Some('0'),
//         _ => None,
//     };
//     if let Some(key) = key {
//         print!("{}", key);
//     }

//     end_of_interrupt(InterruptIndex::Keyboard);
// }

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
