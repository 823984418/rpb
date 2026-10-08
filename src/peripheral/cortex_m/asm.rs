/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
pub fn bkpt() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!("bkpt", options(nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Blocks the program for *at least* `cycles` CPU cycles.
///
/// This is implemented in assembly as a fixed number of iterations of a loop, so that execution
/// time is independent of the optimization level.
///
/// The loop code is the same for all architectures, however the number of CPU cycles required for
/// one iteration varies substantially between architectures.  This means that with a 48MHz CPU
/// clock, a call to `delay(48_000_000)` is guaranteed to take at least 1 second, but for example
/// could take 3 or more seconds.
///
/// In particular, Cortex-M7 cores can sometimes retire two instructions per clock cycle, leading
/// to this particular loop generally taking one clock cycle per iteration. Most other Cortex-M
/// cores will take three cycles per iteration.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution and the
/// execution time may vary with other factors. This delay is mainly useful for simple timer-less
/// initialization of peripherals if and only if accurate timing is not essential. In any other case
/// please use a more accurate method to produce a delay.
pub fn delay(cycles: u32) {
    _ = cycles;
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            let real_cyc = cycles.saturating_add(1);
            core::arch::asm!(
                // The `bne` on some cores (eg Cortex-M4) will take a different number of instructions
                // depending on the alignment of the branch target.  Set the alignment of the top of the
                // loop to prevent surprising timing changes when the alignment of the delay() changes.
                ".p2align 3",
                // Use local labels to avoid R_ARM_THM_JUMP8 relocations which fail on thumbv6m.
                "2:", // not 1 or 0 because of https://github.com/llvm/llvm-project/issues/99547
                "subs {}, #1", // subtract 1 from real_cyc
                "bne 2b",      // branch to 2 if result is non-zero
                inout(reg) real_cyc => _,
                options(nomem, nostack),
            );
        },
        _ => unreachable!(),
    }
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
pub fn nop() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            // NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
            // the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
            // nops when they call `nop` N times, let's not add that option.
            core::arch::asm!("nop", options(nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Generate an Undefined Instruction exception.
///
/// Can be used as a stable alternative to `core::intrinsics::abort`.
pub fn udf() -> ! {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!("udf #0", options(noreturn, nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Wait For Event
pub fn wfe() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Wait For Interrupt
pub fn wfi() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Send Event
pub fn sev() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!("sev", options(nomem, nostack, preserves_flags));
        },
        _ => unreachable!(),
    }
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
pub fn isb() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::arch::asm!("isb", options(nostack, preserves_flags));
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        },
        _ => unreachable!(),
    }
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
pub fn dsb() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::arch::asm!("dsb", options(nostack, preserves_flags));
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        },
        _ => unreachable!(),
    }
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
pub fn dmb() {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::arch::asm!("dmb", options(nostack, preserves_flags));
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        },
        _ => unreachable!(),
    }
}
