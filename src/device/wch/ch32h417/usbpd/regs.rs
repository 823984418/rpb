use crate::register::Reg32;

#[derive(Debug)]
#[repr(C)]
pub struct USBPD {
    pub USBPD_CONFIG: Reg32,
    pub USBPD_CONTROL: Reg32,
    pub USBPD_STATUS: Reg32,
    pub USBPD_PORT: Reg32,
    pub USBPD_DMA: Reg32,
}
