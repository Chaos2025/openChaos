#[cfg(feature = "axstd")]
use axstd::{println, process};

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

pub fn loader(){
    let load_start = PLASH_START as *const u8;
    let load_size = 32000; // Dangerous!!! We need to get accurate size of apps.

    println!("Load payload ...");

    let load_code = unsafe { core::slice::from_raw_parts(load_start, load_size) };
    // println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // va_pa_offset: 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000; // 0x10130

    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size)
    };
    run_code.copy_from_slice(load_code);
    let abi_table = unsafe {
        &ABI_TABLE
    };
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
    // println!("address [{:?}]", run_code.as_ptr());
    // println!("abi_table [{:?}]", abi_table.as_ptr());

    println!("Load payload ok!");

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    println!("Execute app ...");

    // execute app
    unsafe { core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const (RUN_START + 4),
        abi_table = sym ABI_TABLE,

        options(nomem, nostack, preserves_flags)
    )}
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0;16];

fn register_abi(num: usize, handle: usize) {
    unsafe {ABI_TABLE[num] = handle;}
}

fn abi_hello() {
    println!("[ABI:Hello] hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI: Print] {c}");
}

fn abi_terminate() {
    println!("[ABI: Terminate] terminate");
    process::exit(0);
}

pub fn alj(){
    print!("[anlj:] into alj!\n");
    loader();
}