pub fn mrs<const SYSM: u8>() -> u32 {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            let r: u32;
            core::arch::asm!(
                "mrs {0}, {1}",
                out(reg) r,
                const SYSM,
                options(nomem, nostack, preserves_flags)
            );
            r
        },
        _ => unreachable!(),
    }
}

pub unsafe fn msr<const SYSM: u8>(value: u32) {
    cfg_select! {
        all(target_arch = "arm", target_feature = "v6m") => unsafe {
            core::arch::asm!(
                "msr {1}, {0}",
                in(reg) value,
                const SYSM,
                options(nomem, nostack, preserves_flags)
            );
        },
        _ => unreachable!(),
    }
}
