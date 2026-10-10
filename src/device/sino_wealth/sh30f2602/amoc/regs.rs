use bitflags::bitflags;

use crate::register::Reg;

/// AMOC register block.
#[derive(Debug)]
#[repr(C)]
pub struct AMOC {
    pub CMP0CON: Reg<CMP0CON>,
    pub CMP1CON: Reg<CMP1CON>,
    pub CMPINTF: Reg<CMPINTF>,
    pub OPCON: Reg<OPCON>,
    pub CMP2CON: Reg<CMP1CON>,
}

bitflags! {
    /// `CMP0CON` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP0CON: u32 {
        /// 内部电压输出到引脚
        const NOUTEN = 1 << 30;

        /// 内部电压选择
        const VREF_MASK = 0xF << 26;

        const TRGPOL = 1 << 22;

        const PWMTRGS_MASK = 0x3 << 20;

        /// VSUM控制
        const VSUMCHS_MASK = 0x3 << 17;

        /// VSUM输出到引脚
        const CHSUMOUTEN = 1 << 16;

        /// 比较器使能
        const EN = 1 << 15;

        /// 结果输出到引脚
        const OUTEN = 1 << 12;

        /// 中断模式选择
        const IES_MASK = 0x3 << 10;

        /// 反相输入端
        const NCHS_MASK = 0x3 << 8;

        /// 同相输入端
        const PCHS_MASK = 0x7 << 4;

        /// 比较器输出
        const OUT = 1 << 3;

        const DEB_HSI_128 = 0b111 << 0;
        const DEB_HSI_96 = 0b110 << 0;
        const DEB_HSI_64 = 0b101 << 0;
        const DEB_HSI_32 = 0b100 << 0;
        const DEB_HSI_16 = 0b011 << 0;
        const DEB_HSI_8 = 0b010 << 0;
        const DEB_HSI_4 = 0b001 << 0;
        const DEB_NONE = 0b000 << 0;
        /// 输出信号滤波
        const DEB_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `CMP1CON` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP1CON: u32 {
        /// 内部电压输出到引脚
        const NOUTEN = 1 << 30;

        /// VSUM控制
        const VRS_MASK = 0xf << 26;

        /// 比较器使能
        const EN = 1 << 15;

        /// 迟滞使能
        const SMT = 1 << 13;

        /// 结果输出到引脚
        const OUTEN = 1 << 12;

        /// 中断模式选择
        const IES_MASK = 0x3 << 10;

        /// 反相输入端
        const NCHS = 1 << 8;

        /// 同相输入端
        const PCHS_MASK = 0xf << 4;

        /// 比较器输出
        const OUT = 1 << 3;

        const DEB_HSI_128 = 0b111 << 0;
        const DEB_HSI_96 = 0b110 << 0;
        const DEB_HSI_64 = 0b101 << 0;
        const DEB_HSI_32 = 0b100 << 0;
        const DEB_HSI_16 = 0b011 << 0;
        const DEB_HSI_8 = 0b010 << 0;
        const DEB_HSI_4 = 0b001 << 0;
        const DEB_NONE = 0b000 << 0;
        /// 输出信号滤波
        const DEB_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `CMPINTF` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMPINTF: u32 {
        const C2IFC = 1 << 18;

        const C1IFC = 1 << 17;

        const C0IFC = 1 << 16;

        const C2IF = 1 << 2;

        const C1IF = 1 << 1;

        const C0IF = 1 << 0;
    }
}

bitflags! {
    /// `OPCON` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPCON: u32 {
        const OP3EN = 1 << 3;

        const OP2EN = 1 << 2;

        const OP1EN = 1 << 1;

        const OP0EN = 1 << 0;
    }
}
