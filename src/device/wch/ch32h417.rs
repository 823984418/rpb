use crate::peripheral::Block;
use crate::peripheral::riscv32::qingke::hsem::HSEM;
use crate::peripheral::riscv32::qingke::ipc::IPC;
use crate::peripheral::riscv32::qingke::systick::SysTick;

pub struct CH32H417 {
    pub HSEM: Block<HSEM, 0xE000C000>,
    pub IPC: Block<IPC, 0xE000F000>,
    pub SysTick0: Block<SysTick, 0xE000F000>,
    pub SysTick1: Block<SysTick, 0xE000F080>,
}
