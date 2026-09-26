pub fn fence_i() {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => unsafe {
            core::arch::asm!("fence.i", options(nostack))
        },
        _ => unimplemented!(),
    }
}
