#![no_std]
#![doc = "Peripheral access API (generated using svd2rust v0.17.0 (8c99911 2021-04-05))"]
pub mod generic {
    use core::marker::PhantomData;
    #[derive(Copy, Clone)]
    pub struct RW;
    #[derive(Copy, Clone)]
    pub struct R;
    #[derive(Copy, Clone)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        pub fn from_ptr(ptr: *mut u8) -> Self {
            Self {
                ptr,
                phantom: PhantomData,
            }
        }
        pub fn ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        pub unsafe fn read(&self) -> T {
            (self.ptr as *mut T).read_volatile()
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        pub unsafe fn write_value(&self, val: T) {
            (self.ptr as *mut T).write_volatile(val)
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        pub unsafe fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        pub unsafe fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod gpio_v1 {
    use crate::generic::*;
    #[doc = "General purpose I/O"]
    #[derive(Copy, Clone)]
    pub struct Gpio(pub *mut u8);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[doc = "Port configuration register low (GPIOn_CRL)"]
        pub fn cr(self, n: usize) -> Reg<regs::Cr, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
        }
        #[doc = "Port input data register (GPIOn_IDR)"]
        pub fn idr(self) -> Reg<regs::Idr, R> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Port output data register (GPIOn_ODR)"]
        pub fn odr(self) -> Reg<regs::Odr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
        pub fn bsrr(self) -> Reg<regs::Bsrr, W> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Port bit reset register (GPIOn_BRR)"]
        pub fn brr(self) -> Reg<regs::Brr, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Port configuration lock register"]
        pub fn lckr(self) -> Reg<regs::Lckr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Port input data register (GPIOn_IDR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idr(pub u32);
        impl Idr {
            #[doc = "Port input data"]
            pub fn idr(&self, n: usize) -> super::vals::Idr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Idr(val as u8)
            }
            #[doc = "Port input data"]
            pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Idr {
            fn default() -> Idr {
                Idr(0)
            }
        }
        #[doc = "Port bit reset register (GPIOn_BRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "Reset bit"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Set bit"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set bit"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Reset bit"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 16u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
            }
        }
        #[doc = "Port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port A Lock bit"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port A Lock bit"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Lock key"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16u32) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Lock key"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16u32)) | (((val.0 as u32) & 0x01) << 16u32);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "Port output data register (GPIOn_ODR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Odr(pub u32);
        impl Odr {
            #[doc = "Port output data"]
            pub fn odr(&self, n: usize) -> super::vals::Odr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Odr(val as u8)
            }
            #[doc = "Port output data"]
            pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Odr {
            fn default() -> Odr {
                Odr(0)
            }
        }
        #[doc = "Port configuration register (GPIOn_CRx)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Port n mode bits"]
            pub fn mode(&self, n: usize) -> super::vals::Mode {
                assert!(n < 8usize);
                let offs = 0u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Mode(val as u8)
            }
            #[doc = "Port n mode bits"]
            pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
                assert!(n < 8usize);
                let offs = 0u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Port n configuration bits"]
            pub fn cnf(&self, n: usize) -> super::vals::Cnf {
                assert!(n < 8usize);
                let offs = 2u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Cnf(val as u8)
            }
            #[doc = "Port n configuration bits"]
            pub fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
                assert!(n < 8usize);
                let offs = 2u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "No action on the corresponding ODx bit"]
            pub const NOACTION: Self = Self(0);
            #[doc = "Reset the ODx bit"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsw(pub u8);
        impl Bsw {
            #[doc = "No action on the corresponding ODx bit"]
            pub const NOACTION: Self = Self(0);
            #[doc = "Sets the corresponding ODRx bit"]
            pub const SET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Mode(pub u8);
        impl Mode {
            #[doc = "Input mode (reset state)"]
            pub const INPUT: Self = Self(0);
            #[doc = "Output mode 10 MHz"]
            pub const OUTPUT: Self = Self(0x01);
            #[doc = "Output mode 2 MHz"]
            pub const OUTPUT2: Self = Self(0x02);
            #[doc = "Output mode 50 MHz"]
            pub const OUTPUT50: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cnf(pub u8);
        impl Cnf {
            #[doc = "Analog mode / Push-Pull mode"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Floating input (reset state) / Open Drain-Mode"]
            pub const OPENDRAIN: Self = Self(0x01);
            #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
            pub const ALTPUSHPULL: Self = Self(0x02);
            #[doc = "Alternate Function Open-Drain Mode"]
            pub const ALTOPENDRAIN: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
        }
    }
}
pub mod usart_v1 {
    use crate::generic::*;
    #[doc = "Universal synchronous asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Usart(pub *mut u8);
    unsafe impl Send for Usart {}
    unsafe impl Sync for Usart {}
    impl Usart {
        #[doc = "Status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Guard time and prescaler register"]
        pub fn gtpr(self) -> Reg<regs::Gtpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
            }
            #[doc = "lin break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5u32) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "lin break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> super::vals::Lbdie {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Lbdie(val as u8)
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: super::vals::Lbdie) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
            }
            #[doc = "Last bit clock pulse"]
            pub const fn lbcl(&self) -> bool {
                let val = (self.0 >> 8u32) & 0x01;
                val != 0
            }
            #[doc = "Last bit clock pulse"]
            pub fn set_lbcl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
            }
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 9u32) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 10u32) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
            }
            #[doc = "Clock enable"]
            pub const fn clken(&self) -> super::vals::Clken {
                let val = (self.0 >> 11u32) & 0x01;
                super::vals::Clken(val as u8)
            }
            #[doc = "Clock enable"]
            pub fn set_clken(&mut self, val: super::vals::Clken) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12u32) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> super::vals::Linen {
                let val = (self.0 >> 14u32) & 0x01;
                super::vals::Linen(val as u8)
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: super::vals::Linen) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr3(pub u32);
        impl Cr3 {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> super::vals::Eie {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Eie(val as u8)
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: super::vals::Eie) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> super::vals::Iren {
                let val = (self.0 >> 1u32) & 0x01;
                super::vals::Iren(val as u8)
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: super::vals::Iren) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2u32) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "Smartcard NACK enable"]
            pub const fn nack(&self) -> super::vals::Nack {
                let val = (self.0 >> 4u32) & 0x01;
                super::vals::Nack(val as u8)
            }
            #[doc = "Smartcard NACK enable"]
            pub fn set_nack(&mut self, val: super::vals::Nack) {
                self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
            }
            #[doc = "Smartcard mode enable"]
            pub const fn scen(&self) -> super::vals::Scen {
                let val = (self.0 >> 5u32) & 0x01;
                super::vals::Scen(val as u8)
            }
            #[doc = "Smartcard mode enable"]
            pub fn set_scen(&mut self, val: super::vals::Scen) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> super::vals::Dmar {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Dmar(val as u8)
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: super::vals::Dmar) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> super::vals::Dmat {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Dmat(val as u8)
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: super::vals::Dmat) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
            #[doc = "RTS enable"]
            pub const fn rtse(&self) -> super::vals::Rtse {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Rtse(val as u8)
            }
            #[doc = "RTS enable"]
            pub fn set_rtse(&mut self, val: super::vals::Rtse) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
            }
            #[doc = "CTS enable"]
            pub const fn ctse(&self) -> super::vals::Ctse {
                let val = (self.0 >> 9u32) & 0x01;
                super::vals::Ctse(val as u8)
            }
            #[doc = "CTS enable"]
            pub fn set_ctse(&mut self, val: super::vals::Ctse) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
            }
            #[doc = "CTS interrupt enable"]
            pub const fn ctsie(&self) -> super::vals::Ctsie {
                let val = (self.0 >> 10u32) & 0x01;
                super::vals::Ctsie(val as u8)
            }
            #[doc = "CTS interrupt enable"]
            pub fn set_ctsie(&mut self, val: super::vals::Ctsie) {
                self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
            }
        }
        impl Default for Cr3 {
            fn default() -> Cr3 {
                Cr3(0)
            }
        }
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Send break"]
            pub const fn sbk(&self) -> super::vals::Sbk {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Sbk(val as u8)
            }
            #[doc = "Send break"]
            pub fn set_sbk(&mut self, val: super::vals::Sbk) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Receiver wakeup"]
            pub const fn rwu(&self) -> super::vals::Rwu {
                let val = (self.0 >> 1u32) & 0x01;
                super::vals::Rwu(val as u8)
            }
            #[doc = "Receiver wakeup"]
            pub fn set_rwu(&mut self, val: super::vals::Rwu) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
            }
            #[doc = "Receiver enable"]
            pub const fn re(&self) -> super::vals::Re {
                let val = (self.0 >> 2u32) & 0x01;
                super::vals::Re(val as u8)
            }
            #[doc = "Receiver enable"]
            pub fn set_re(&mut self, val: super::vals::Re) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
            }
            #[doc = "Transmitter enable"]
            pub const fn te(&self) -> super::vals::Te {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Te(val as u8)
            }
            #[doc = "Transmitter enable"]
            pub fn set_te(&mut self, val: super::vals::Te) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "IDLE interrupt enable"]
            pub const fn idleie(&self) -> super::vals::Idleie {
                let val = (self.0 >> 4u32) & 0x01;
                super::vals::Idleie(val as u8)
            }
            #[doc = "IDLE interrupt enable"]
            pub fn set_idleie(&mut self, val: super::vals::Idleie) {
                self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
            }
            #[doc = "RXNE interrupt enable"]
            pub const fn rxneie(&self) -> super::vals::Rxneie {
                let val = (self.0 >> 5u32) & 0x01;
                super::vals::Rxneie(val as u8)
            }
            #[doc = "RXNE interrupt enable"]
            pub fn set_rxneie(&mut self, val: super::vals::Rxneie) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
            }
            #[doc = "Transmission complete interrupt enable"]
            pub const fn tcie(&self) -> super::vals::Tcie {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Tcie(val as u8)
            }
            #[doc = "Transmission complete interrupt enable"]
            pub fn set_tcie(&mut self, val: super::vals::Tcie) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
            }
            #[doc = "TXE interrupt enable"]
            pub const fn txeie(&self) -> super::vals::Txeie {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Txeie(val as u8)
            }
            #[doc = "TXE interrupt enable"]
            pub fn set_txeie(&mut self, val: super::vals::Txeie) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
            #[doc = "PE interrupt enable"]
            pub const fn peie(&self) -> super::vals::Peie {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Peie(val as u8)
            }
            #[doc = "PE interrupt enable"]
            pub fn set_peie(&mut self, val: super::vals::Peie) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
            }
            #[doc = "Parity selection"]
            pub const fn ps(&self) -> super::vals::Ps {
                let val = (self.0 >> 9u32) & 0x01;
                super::vals::Ps(val as u8)
            }
            #[doc = "Parity selection"]
            pub fn set_ps(&mut self, val: super::vals::Ps) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
            }
            #[doc = "Parity control enable"]
            pub const fn pce(&self) -> super::vals::Pce {
                let val = (self.0 >> 10u32) & 0x01;
                super::vals::Pce(val as u8)
            }
            #[doc = "Parity control enable"]
            pub fn set_pce(&mut self, val: super::vals::Pce) {
                self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
            }
            #[doc = "Wakeup method"]
            pub const fn wake(&self) -> super::vals::Wake {
                let val = (self.0 >> 11u32) & 0x01;
                super::vals::Wake(val as u8)
            }
            #[doc = "Wakeup method"]
            pub fn set_wake(&mut self, val: super::vals::Wake) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
            }
            #[doc = "Word length"]
            pub const fn m(&self) -> super::vals::M {
                let val = (self.0 >> 12u32) & 0x01;
                super::vals::M(val as u8)
            }
            #[doc = "Word length"]
            pub fn set_m(&mut self, val: super::vals::M) {
                self.0 = (self.0 & !(0x01 << 12u32)) | (((val.0 as u32) & 0x01) << 12u32);
            }
            #[doc = "USART enable"]
            pub const fn ue(&self) -> super::vals::Ue {
                let val = (self.0 >> 13u32) & 0x01;
                super::vals::Ue(val as u8)
            }
            #[doc = "USART enable"]
            pub fn set_ue(&mut self, val: super::vals::Ue) {
                self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "Baud rate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "fraction of USARTDIV"]
            pub const fn div_fraction(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0x0f;
                val as u8
            }
            #[doc = "fraction of USARTDIV"]
            pub fn set_div_fraction(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
            }
            #[doc = "mantissa of USARTDIV"]
            pub const fn div_mantissa(&self) -> u16 {
                let val = (self.0 >> 4u32) & 0x0fff;
                val as u16
            }
            #[doc = "mantissa of USARTDIV"]
            pub fn set_div_mantissa(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 4u32)) | (((val as u32) & 0x0fff) << 4u32);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data value"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0x01ff;
                val as u16
            }
            #[doc = "Data value"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0u32)) | (((val as u32) & 0x01ff) << 0u32);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0u32) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1u32) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
            }
            #[doc = "Noise error flag"]
            pub const fn ne(&self) -> bool {
                let val = (self.0 >> 2u32) & 0x01;
                val != 0
            }
            #[doc = "Noise error flag"]
            pub fn set_ne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3u32) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
            }
            #[doc = "IDLE line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4u32) & 0x01;
                val != 0
            }
            #[doc = "IDLE line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6u32) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbd(&self) -> bool {
                let val = (self.0 >> 8u32) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
            }
            #[doc = "CTS flag"]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 9u32) & 0x01;
                val != 0
            }
            #[doc = "CTS flag"]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "Guard time and prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Gtpr(pub u32);
        impl Gtpr {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0xff;
                val as u8
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
            }
            #[doc = "Guard time value"]
            pub const fn gt(&self) -> u8 {
                let val = (self.0 >> 8u32) & 0xff;
                val as u8
            }
            #[doc = "Guard time value"]
            pub fn set_gt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
            }
        }
        impl Default for Gtpr {
            fn default() -> Gtpr {
                Gtpr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ctsie(pub u8);
        impl Ctsie {
            #[doc = "CTS interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CTS interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Stop(pub u8);
        impl Stop {
            #[doc = "1 stop bit"]
            pub const STOP1: Self = Self(0);
            #[doc = "0.5 stop bits"]
            pub const STOP0P5: Self = Self(0x01);
            #[doc = "2 stop bits"]
            pub const STOP2: Self = Self(0x02);
            #[doc = "1.5 stop bits"]
            pub const STOP1P5: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Peie(pub u8);
        impl Peie {
            #[doc = "PE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "PE interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ps(pub u8);
        impl Ps {
            #[doc = "Even parity"]
            pub const EVEN: Self = Self(0);
            #[doc = "Odd parity"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Rtse(pub u8);
        impl Rtse {
            #[doc = "RTS hardware flow control disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "RTS hardware flow control enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lbdie(pub u8);
        impl Lbdie {
            #[doc = "LIN break detection interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "LIN break detection interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Txeie(pub u8);
        impl Txeie {
            #[doc = "TXE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "TXE interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ctse(pub u8);
        impl Ctse {
            #[doc = "CTS hardware flow control disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CTS hardware flow control enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dmat(pub u8);
        impl Dmat {
            #[doc = "DMA mode is disabled for transmission"]
            pub const DISABLED: Self = Self(0);
            #[doc = "DMA mode is enabled for transmission"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Eie(pub u8);
        impl Eie {
            #[doc = "Error interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Error interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct M(pub u8);
        impl M {
            #[doc = "8 data bits"]
            pub const M8: Self = Self(0);
            #[doc = "9 data bits"]
            pub const M9: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dmar(pub u8);
        impl Dmar {
            #[doc = "DMA mode is disabled for reception"]
            pub const DISABLED: Self = Self(0);
            #[doc = "DMA mode is enabled for reception"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Scen(pub u8);
        impl Scen {
            #[doc = "Smartcard mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Smartcard mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Sbk(pub u8);
        impl Sbk {
            #[doc = "No break character is transmitted"]
            pub const NOBREAK: Self = Self(0);
            #[doc = "Break character transmitted"]
            pub const BREAK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Iren(pub u8);
        impl Iren {
            #[doc = "IrDA disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "IrDA enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idleie(pub u8);
        impl Idleie {
            #[doc = "IDLE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "IDLE interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Re(pub u8);
        impl Re {
            #[doc = "Receiver disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Receiver enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Rwu(pub u8);
        impl Rwu {
            #[doc = "Receiver in active mode"]
            pub const ACTIVE: Self = Self(0);
            #[doc = "Receiver in mute mode"]
            pub const MUTE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "Steady low value on CK pin outside transmission window"]
            pub const LOW: Self = Self(0);
            #[doc = "Steady high value on CK pin outside transmission window"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Rxneie(pub u8);
        impl Rxneie {
            #[doc = "RXNE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "RXNE interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tcie(pub u8);
        impl Tcie {
            #[doc = "TC interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "TC interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Hdsel(pub u8);
        impl Hdsel {
            #[doc = "Half duplex mode is not selected"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Half duplex mode is selected"]
            pub const HALFDUPLEX: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Wake(pub u8);
        impl Wake {
            #[doc = "USART wakeup on idle line"]
            pub const IDLELINE: Self = Self(0);
            #[doc = "USART wakeup on address mark"]
            pub const ADDRESSMARK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lbdl(pub u8);
        impl Lbdl {
            #[doc = "10-bit break detection"]
            pub const LBDL10: Self = Self(0);
            #[doc = "11-bit break detection"]
            pub const LBDL11: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Linen(pub u8);
        impl Linen {
            #[doc = "LIN mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "LIN mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Irlp(pub u8);
        impl Irlp {
            #[doc = "Normal mode"]
            pub const NORMAL: Self = Self(0);
            #[doc = "Low-power mode"]
            pub const LOWPOWER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Nack(pub u8);
        impl Nack {
            #[doc = "NACK transmission in case of parity error is disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "NACK transmission during parity error is enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Clken(pub u8);
        impl Clken {
            #[doc = "CK pin disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CK pin enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Pce(pub u8);
        impl Pce {
            #[doc = "Parity control disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Parity control enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ue(pub u8);
        impl Ue {
            #[doc = "USART prescaler and outputs disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "USART enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Te(pub u8);
        impl Te {
            #[doc = "Transmitter disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Transmitter enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRST: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECOND: Self = Self(0x01);
        }
    }
}
pub mod timer_v1 {
    use crate::generic::*;
    #[doc = "General purpose 32-bit timer"]
    #[derive(Copy, Clone)]
    pub struct TimGp32(pub *mut u8);
    unsafe impl Send for TimGp32 {}
    unsafe impl Sync for TimGp32 {}
    impl TimGp32 {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrGp, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt32, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr32, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr32, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    #[doc = "Advanced-timers"]
    #[derive(Copy, Clone)]
    pub struct TimAdv(pub *mut u8);
    unsafe impl Send for TimAdv {}
    unsafe impl Sync for TimAdv {}
    impl TimAdv {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Adv, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrAdv, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerAdv, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "repetition counter register"]
        pub fn rcr(self) -> Reg<regs::Rcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr16, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "break and dead-time register"]
        pub fn bdtr(self) -> Reg<regs::Bdtr, RW> {
            unsafe { Reg::from_ptr(self.0.add(68usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    #[doc = "Basic timer"]
    #[derive(Copy, Clone)]
    pub struct TimBasic(pub *mut u8);
    unsafe impl Send for TimBasic {}
    unsafe impl Sync for TimBasic {}
    impl TimBasic {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Basic, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Basic, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierBasic, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrBasic, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrBasic, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
    }
    #[doc = "General purpose 16-bit timer"]
    #[derive(Copy, Clone)]
    pub struct TimGp16(pub *mut u8);
    unsafe impl Send for TimGp16 {}
    unsafe impl Sync for TimGp16 {}
    impl TimGp16 {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Gp, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "slave mode control register"]
        pub fn smcr(self) -> Reg<regs::Smcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DMA/Interrupt enable register"]
        pub fn dier(self) -> Reg<regs::DierGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::SrGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "event generation register"]
        pub fn egr(self) -> Reg<regs::EgrGp, W> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        pub fn ccmr_input(self, n: usize) -> Reg<regs::CcmrInput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare mode register 1 (output mode)"]
        pub fn ccmr_output(self, n: usize) -> Reg<regs::CcmrOutput, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(24usize + n * 4usize)) }
        }
        #[doc = "capture/compare enable register"]
        pub fn ccer(self) -> Reg<regs::CcerGp, RW> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "counter"]
        pub fn cnt(self) -> Reg<regs::Cnt16, RW> {
            unsafe { Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "prescaler"]
        pub fn psc(self) -> Reg<regs::Psc, RW> {
            unsafe { Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "auto-reload register"]
        pub fn arr(self) -> Reg<regs::Arr16, RW> {
            unsafe { Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "capture/compare register"]
        pub fn ccr(self, n: usize) -> Reg<regs::Ccr16, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(52usize + n * 4usize)) }
        }
        #[doc = "DMA control register"]
        pub fn dcr(self) -> Reg<regs::Dcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(72usize)) }
        }
        #[doc = "DMA address for full transfer"]
        pub fn dmar(self) -> Reg<regs::Dmar, RW> {
            unsafe { Reg::from_ptr(self.0.add(76usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Udis(pub u8);
        impl Udis {
            #[doc = "Update event enabled"]
            pub const ENABLED: Self = Self(0);
            #[doc = "Update event disabled"]
            pub const DISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Arpe(pub u8);
        impl Arpe {
            #[doc = "TIMx_APRR register is not buffered"]
            pub const DISABLED: Self = Self(0);
            #[doc = "TIMx_APRR register is buffered"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ug(pub u8);
        impl Ug {
            #[doc = "Re-initializes the timer counter and generates an update of the registers."]
            pub const UPDATE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Moe(pub u8);
        impl Moe {
            #[doc = "OC/OCN are disabled or forced idle depending on OSSI"]
            pub const DISABLEDIDLE: Self = Self(0);
            #[doc = "OC/OCN are enabled if CCxE/CCxNE are set"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Etf(pub u8);
        impl Etf {
            #[doc = "No filter, sampling is done at fDTS"]
            pub const NOFILTER: Self = Self(0);
            #[doc = "fSAMPLING=fCK_INT, N=2"]
            pub const FCK_INT_N2: Self = Self(0x01);
            #[doc = "fSAMPLING=fCK_INT, N=4"]
            pub const FCK_INT_N4: Self = Self(0x02);
            #[doc = "fSAMPLING=fCK_INT, N=8"]
            pub const FCK_INT_N8: Self = Self(0x03);
            #[doc = "fSAMPLING=fDTS/2, N=6"]
            pub const FDTS_DIV2_N6: Self = Self(0x04);
            #[doc = "fSAMPLING=fDTS/2, N=8"]
            pub const FDTS_DIV2_N8: Self = Self(0x05);
            #[doc = "fSAMPLING=fDTS/4, N=6"]
            pub const FDTS_DIV4_N6: Self = Self(0x06);
            #[doc = "fSAMPLING=fDTS/4, N=8"]
            pub const FDTS_DIV4_N8: Self = Self(0x07);
            #[doc = "fSAMPLING=fDTS/8, N=6"]
            pub const FDTS_DIV8_N6: Self = Self(0x08);
            #[doc = "fSAMPLING=fDTS/8, N=8"]
            pub const FDTS_DIV8_N8: Self = Self(0x09);
            #[doc = "fSAMPLING=fDTS/16, N=5"]
            pub const FDTS_DIV16_N5: Self = Self(0x0a);
            #[doc = "fSAMPLING=fDTS/16, N=6"]
            pub const FDTS_DIV16_N6: Self = Self(0x0b);
            #[doc = "fSAMPLING=fDTS/16, N=8"]
            pub const FDTS_DIV16_N8: Self = Self(0x0c);
            #[doc = "fSAMPLING=fDTS/32, N=5"]
            pub const FDTS_DIV32_N5: Self = Self(0x0d);
            #[doc = "fSAMPLING=fDTS/32, N=6"]
            pub const FDTS_DIV32_N6: Self = Self(0x0e);
            #[doc = "fSAMPLING=fDTS/32, N=8"]
            pub const FDTS_DIV32_N8: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ossi(pub u8);
        impl Ossi {
            #[doc = "When inactive, OC/OCN outputs are disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
            pub const IDLELEVEL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Sms(pub u8);
        impl Sms {
            #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
            pub const DISABLED: Self = Self(0);
            #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
            pub const ENCODER_MODE_1: Self = Self(0x01);
            #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
            pub const ENCODER_MODE_2: Self = Self(0x02);
            #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
            pub const ENCODER_MODE_3: Self = Self(0x03);
            #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
            pub const RESET_MODE: Self = Self(0x04);
            #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
            pub const GATED_MODE: Self = Self(0x05);
            #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
            pub const TRIGGER_MODE: Self = Self(0x06);
            #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
            pub const EXT_CLOCK_MODE: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cen(pub u8);
        impl Cen {
            #[doc = "Counter disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Counter enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tde(pub u8);
        impl Tde {
            #[doc = "Trigger DMA request disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Trigger DMA request enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Etps(pub u8);
        impl Etps {
            #[doc = "Prescaler OFF"]
            pub const DIV1: Self = Self(0);
            #[doc = "ETRP frequency divided by 2"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "ETRP frequency divided by 4"]
            pub const DIV4: Self = Self(0x02);
            #[doc = "ETRP frequency divided by 8"]
            pub const DIV8: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Uif(pub u8);
        impl Uif {
            #[doc = "No update occurred"]
            pub const CLEAR: Self = Self(0);
            #[doc = "Update interrupt pending."]
            pub const UPDATEPENDING: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Mms(pub u8);
        impl Mms {
            #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
            pub const RESET: Self = Self(0);
            #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
            pub const ENABLE: Self = Self(0x01);
            #[doc = "The update event is selected as trigger output"]
            pub const UPDATE: Self = Self(0x02);
            #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
            pub const COMPAREPULSE: Self = Self(0x03);
            #[doc = "OC1REF signal is used as trigger output"]
            pub const COMPAREOC1: Self = Self(0x04);
            #[doc = "OC2REF signal is used as trigger output"]
            pub const COMPAREOC2: Self = Self(0x05);
            #[doc = "OC3REF signal is used as trigger output"]
            pub const COMPAREOC3: Self = Self(0x06);
            #[doc = "OC4REF signal is used as trigger output"]
            pub const COMPAREOC4: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tis(pub u8);
        impl Tis {
            #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
            pub const NORMAL: Self = Self(0);
            #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
            pub const XOR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tifw(pub u8);
        impl Tifw {
            #[doc = "Clear flag"]
            pub const CLEAR: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tgw(pub u8);
        impl Tgw {
            #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
            pub const TRIGGER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccifw(pub u8);
        impl Ccifw {
            #[doc = "Clear flag"]
            pub const CLEAR: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ocm(pub u8);
        impl Ocm {
            #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
            pub const FROZEN: Self = Self(0);
            #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
            pub const ACTIVEONMATCH: Self = Self(0x01);
            #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
            pub const INACTIVEONMATCH: Self = Self(0x02);
            #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
            pub const TOGGLE: Self = Self(0x03);
            #[doc = "OCyREF is forced low"]
            pub const FORCEINACTIVE: Self = Self(0x04);
            #[doc = "OCyREF is forced high"]
            pub const FORCEACTIVE: Self = Self(0x05);
            #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
            pub const PWMMODE1: Self = Self(0x06);
            #[doc = "Inversely to PwmMode1"]
            pub const PWMMODE2: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ece(pub u8);
        impl Ece {
            #[doc = "External clock mode 2 disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ossr(pub u8);
        impl Ossr {
            #[doc = "When inactive, OC/OCN outputs are disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
            pub const IDLELEVEL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccgw(pub u8);
        impl Ccgw {
            #[doc = "If CCx is an output: CCxIF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CCx is an input: The current value of the counter is captured in TIMx_CCR1 register."]
            pub const TRIGGER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Etp(pub u8);
        impl Etp {
            #[doc = "ETR is noninverted, active at high level or rising edge"]
            pub const NOTINVERTED: Self = Self(0);
            #[doc = "ETR is inverted, active at low level or falling edge"]
            pub const INVERTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Opm(pub u8);
        impl Opm {
            #[doc = "Counter is not stopped at update event"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccde(pub u8);
        impl Ccde {
            #[doc = "CCx DMA request disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CCx DMA request enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ocpe(pub u8);
        impl Ocpe {
            #[doc = "Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Preload register on CCR2 enabled. Preload value is loaded into active register on each update event"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Msm(pub u8);
        impl Msm {
            #[doc = "No action"]
            pub const NOSYNC: Self = Self(0);
            #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
            pub const SYNC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ts(pub u8);
        impl Ts {
            #[doc = "Internal Trigger 0 (ITR0)"]
            pub const ITR0: Self = Self(0);
            #[doc = "Internal Trigger 1 (ITR1)"]
            pub const ITR1: Self = Self(0x01);
            #[doc = "Internal Trigger 2 (ITR2)"]
            pub const ITR2: Self = Self(0x02);
            #[doc = "TI1 Edge Detector (TI1F_ED)"]
            pub const TI1F_ED: Self = Self(0x04);
            #[doc = "Filtered Timer Input 1 (TI1FP1)"]
            pub const TI1FP1: Self = Self(0x05);
            #[doc = "Filtered Timer Input 2 (TI2FP2)"]
            pub const TI2FP2: Self = Self(0x06);
            #[doc = "External Trigger input (ETRF)"]
            pub const ETRF: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Urs(pub u8);
        impl Urs {
            #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
            pub const ANYEVENT: Self = Self(0);
            #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
            pub const COUNTERONLY: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccie(pub u8);
        impl Ccie {
            #[doc = "CCx interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CCx interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccofw(pub u8);
        impl Ccofw {
            #[doc = "Clear flag"]
            pub const CLEAR: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Counter used as upcounter"]
            pub const UP: Self = Self(0);
            #[doc = "Counter used as downcounter"]
            pub const DOWN: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tie(pub u8);
        impl Tie {
            #[doc = "Trigger interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Trigger interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cms(pub u8);
        impl Cms {
            #[doc = "The counter counts up or down depending on the direction bit"]
            pub const EDGEALIGNED: Self = Self(0);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
            pub const CENTERALIGNED1: Self = Self(0x01);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
            pub const CENTERALIGNED2: Self = Self(0x02);
            #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
            pub const CENTERALIGNED3: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcmrOutputCcs(pub u8);
        impl CcmrOutputCcs {
            #[doc = "CCx channel is configured as output"]
            pub const OUTPUT: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Tifr(pub u8);
        impl Tifr {
            #[doc = "No trigger event occurred"]
            pub const NOTRIGGER: Self = Self(0);
            #[doc = "Trigger interrupt pending"]
            pub const TRIGGER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccofr(pub u8);
        impl Ccofr {
            #[doc = "The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
            pub const OVERCAPTURE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccds(pub u8);
        impl Ccds {
            #[doc = "CCx DMA request sent when CCx event occurs"]
            pub const ONCOMPARE: Self = Self(0);
            #[doc = "CCx DMA request sent when update event occurs"]
            pub const ONUPDATE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Uie(pub u8);
        impl Uie {
            #[doc = "Update interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Update interrupt enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ude(pub u8);
        impl Ude {
            #[doc = "Update DMA request disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Update DMA request enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcmrInputCcs(pub u8);
        impl CcmrInputCcs {
            #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
            pub const TI4: Self = Self(0x01);
            #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
            pub const TI3: Self = Self(0x02);
            #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
            pub const TRC: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccifr(pub u8);
        impl Ccifr {
            #[doc = "If CCx is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CCx is an input: The counter value has been captured in TIMx_CCR1 register."]
            pub const MATCH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Icf(pub u8);
        impl Icf {
            #[doc = "No filter, sampling is done at fDTS"]
            pub const NOFILTER: Self = Self(0);
            #[doc = "fSAMPLING=fCK_INT, N=2"]
            pub const FCK_INT_N2: Self = Self(0x01);
            #[doc = "fSAMPLING=fCK_INT, N=4"]
            pub const FCK_INT_N4: Self = Self(0x02);
            #[doc = "fSAMPLING=fCK_INT, N=8"]
            pub const FCK_INT_N8: Self = Self(0x03);
            #[doc = "fSAMPLING=fDTS/2, N=6"]
            pub const FDTS_DIV2_N6: Self = Self(0x04);
            #[doc = "fSAMPLING=fDTS/2, N=8"]
            pub const FDTS_DIV2_N8: Self = Self(0x05);
            #[doc = "fSAMPLING=fDTS/4, N=6"]
            pub const FDTS_DIV4_N6: Self = Self(0x06);
            #[doc = "fSAMPLING=fDTS/4, N=8"]
            pub const FDTS_DIV4_N8: Self = Self(0x07);
            #[doc = "fSAMPLING=fDTS/8, N=6"]
            pub const FDTS_DIV8_N6: Self = Self(0x08);
            #[doc = "fSAMPLING=fDTS/8, N=8"]
            pub const FDTS_DIV8_N8: Self = Self(0x09);
            #[doc = "fSAMPLING=fDTS/16, N=5"]
            pub const FDTS_DIV16_N5: Self = Self(0x0a);
            #[doc = "fSAMPLING=fDTS/16, N=6"]
            pub const FDTS_DIV16_N6: Self = Self(0x0b);
            #[doc = "fSAMPLING=fDTS/16, N=8"]
            pub const FDTS_DIV16_N8: Self = Self(0x0c);
            #[doc = "fSAMPLING=fDTS/32, N=5"]
            pub const FDTS_DIV32_N5: Self = Self(0x0d);
            #[doc = "fSAMPLING=fDTS/32, N=6"]
            pub const FDTS_DIV32_N6: Self = Self(0x0e);
            #[doc = "fSAMPLING=fDTS/32, N=8"]
            pub const FDTS_DIV32_N8: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ckd(pub u8);
        impl Ckd {
            #[doc = "t_DTS = t_CK_INT"]
            pub const DIV1: Self = Self(0);
            #[doc = "t_DTS = 2 × t_CK_INT"]
            pub const DIV2: Self = Self(0x01);
            #[doc = "t_DTS = 4 × t_CK_INT"]
            pub const DIV4: Self = Self(0x02);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr2Basic(pub u32);
        impl Cr2Basic {
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4u32) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4u32)) | (((val.0 as u32) & 0x07) << 4u32);
            }
        }
        impl Default for Cr2Basic {
            fn default() -> Cr2Basic {
                Cr2Basic(0)
            }
        }
        #[doc = "capture/compare mode register 2 (output mode)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcmrOutput(pub u32);
        impl CcmrOutput {
            #[doc = "Capture/Compare 3 selection"]
            pub fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
                assert!(n < 2usize);
                let offs = 0u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::CcmrOutputCcs(val as u8)
            }
            #[doc = "Capture/Compare 3 selection"]
            pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
                assert!(n < 2usize);
                let offs = 0u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Output compare 3 fast enable"]
            pub fn ocfe(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 2u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output compare 3 fast enable"]
            pub fn set_ocfe(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 2u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Output compare 3 preload enable"]
            pub fn ocpe(&self, n: usize) -> super::vals::Ocpe {
                assert!(n < 2usize);
                let offs = 3u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ocpe(val as u8)
            }
            #[doc = "Output compare 3 preload enable"]
            pub fn set_ocpe(&mut self, n: usize, val: super::vals::Ocpe) {
                assert!(n < 2usize);
                let offs = 3u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Output compare 3 mode"]
            pub fn ocm(&self, n: usize) -> super::vals::Ocm {
                assert!(n < 2usize);
                let offs = 4u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x07;
                super::vals::Ocm(val as u8)
            }
            #[doc = "Output compare 3 mode"]
            pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
                assert!(n < 2usize);
                let offs = 4u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x07 << offs)) | (((val.0 as u32) & 0x07) << offs);
            }
            #[doc = "Output compare 3 clear enable"]
            pub fn occe(&self, n: usize) -> bool {
                assert!(n < 2usize);
                let offs = 7u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output compare 3 clear enable"]
            pub fn set_occe(&mut self, n: usize, val: bool) {
                assert!(n < 2usize);
                let offs = 7u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcmrOutput {
            fn default() -> CcmrOutput {
                CcmrOutput(0)
            }
        }
        #[doc = "capture/compare enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcerAdv(pub u32);
        impl CcerAdv {
            #[doc = "Capture/Compare 1 output enable"]
            pub fn cce(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output enable"]
            pub fn set_cce(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 complementary output enable"]
            pub fn ccne(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 2u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 complementary output enable"]
            pub fn set_ccne(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 2u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccnp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccnp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcerAdv {
            fn default() -> CcerAdv {
                CcerAdv(0)
            }
        }
        #[doc = "break and dead-time register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bdtr(pub u32);
        impl Bdtr {
            #[doc = "Dead-time generator setup"]
            pub const fn dtg(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0xff;
                val as u8
            }
            #[doc = "Dead-time generator setup"]
            pub fn set_dtg(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
            }
            #[doc = "Lock configuration"]
            pub const fn lock(&self) -> u8 {
                let val = (self.0 >> 8u32) & 0x03;
                val as u8
            }
            #[doc = "Lock configuration"]
            pub fn set_lock(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 8u32)) | (((val as u32) & 0x03) << 8u32);
            }
            #[doc = "Off-state selection for Idle mode"]
            pub const fn ossi(&self) -> super::vals::Ossi {
                let val = (self.0 >> 10u32) & 0x01;
                super::vals::Ossi(val as u8)
            }
            #[doc = "Off-state selection for Idle mode"]
            pub fn set_ossi(&mut self, val: super::vals::Ossi) {
                self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
            }
            #[doc = "Off-state selection for Run mode"]
            pub const fn ossr(&self) -> super::vals::Ossr {
                let val = (self.0 >> 11u32) & 0x01;
                super::vals::Ossr(val as u8)
            }
            #[doc = "Off-state selection for Run mode"]
            pub fn set_ossr(&mut self, val: super::vals::Ossr) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
            }
            #[doc = "Break enable"]
            pub const fn bke(&self) -> bool {
                let val = (self.0 >> 12u32) & 0x01;
                val != 0
            }
            #[doc = "Break enable"]
            pub fn set_bke(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
            }
            #[doc = "Break polarity"]
            pub const fn bkp(&self) -> bool {
                let val = (self.0 >> 13u32) & 0x01;
                val != 0
            }
            #[doc = "Break polarity"]
            pub fn set_bkp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
            }
            #[doc = "Automatic output enable"]
            pub const fn aoe(&self) -> bool {
                let val = (self.0 >> 14u32) & 0x01;
                val != 0
            }
            #[doc = "Automatic output enable"]
            pub fn set_aoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
            }
            #[doc = "Main output enable"]
            pub const fn moe(&self) -> super::vals::Moe {
                let val = (self.0 >> 15u32) & 0x01;
                super::vals::Moe(val as u8)
            }
            #[doc = "Main output enable"]
            pub fn set_moe(&mut self, val: super::vals::Moe) {
                self.0 = (self.0 & !(0x01 << 15u32)) | (((val.0 as u32) & 0x01) << 15u32);
            }
        }
        impl Default for Bdtr {
            fn default() -> Bdtr {
                Bdtr(0)
            }
        }
        #[doc = "counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cnt32(pub u32);
        impl Cnt32 {
            #[doc = "counter value"]
            pub const fn cnt(&self) -> u32 {
                let val = (self.0 >> 0u32) & 0xffff_ffff;
                val as u32
            }
            #[doc = "counter value"]
            pub fn set_cnt(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
            }
        }
        impl Default for Cnt32 {
            fn default() -> Cnt32 {
                Cnt32(0)
            }
        }
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct DierGp(pub u32);
        impl DierGp {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> super::vals::Uie {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uie(val as u8)
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: super::vals::Uie) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn ccie(&self, n: usize) -> super::vals::Ccie {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ccie(val as u8)
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn set_ccie(&mut self, n: usize, val: super::vals::Ccie) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Trigger interrupt enable"]
            pub const fn tie(&self) -> super::vals::Tie {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Tie(val as u8)
            }
            #[doc = "Trigger interrupt enable"]
            pub fn set_tie(&mut self, val: super::vals::Tie) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> super::vals::Ude {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Ude(val as u8)
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: super::vals::Ude) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn ccde(&self, n: usize) -> super::vals::Ccde {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ccde(val as u8)
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn set_ccde(&mut self, n: usize, val: super::vals::Ccde) {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Trigger DMA request enable"]
            pub const fn tde(&self) -> super::vals::Tde {
                let val = (self.0 >> 14u32) & 0x01;
                super::vals::Tde(val as u8)
            }
            #[doc = "Trigger DMA request enable"]
            pub fn set_tde(&mut self, val: super::vals::Tde) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
            }
        }
        impl Default for DierGp {
            fn default() -> DierGp {
                DierGp(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct SrBasic(pub u32);
        impl SrBasic {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> super::vals::Uif {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uif(val as u8)
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: super::vals::Uif) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
        }
        impl Default for SrBasic {
            fn default() -> SrBasic {
                SrBasic(0)
            }
        }
        #[doc = "capture/compare register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccr16(pub u32);
        impl Ccr16 {
            #[doc = "Capture/Compare 1 value"]
            pub const fn ccr(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0xffff;
                val as u16
            }
            #[doc = "Capture/Compare 1 value"]
            pub fn set_ccr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
            }
        }
        impl Default for Ccr16 {
            fn default() -> Ccr16 {
                Ccr16(0)
            }
        }
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct DierBasic(pub u32);
        impl DierBasic {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> super::vals::Uie {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uie(val as u8)
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: super::vals::Uie) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> super::vals::Ude {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Ude(val as u8)
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: super::vals::Ude) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
            }
        }
        impl Default for DierBasic {
            fn default() -> DierBasic {
                DierBasic(0)
            }
        }
        #[doc = "capture/compare mode register 1 (input mode)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcmrInput(pub u32);
        impl CcmrInput {
            #[doc = "Capture/Compare 1 selection"]
            pub fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
                assert!(n < 2usize);
                let offs = 0u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::CcmrInputCcs(val as u8)
            }
            #[doc = "Capture/Compare 1 selection"]
            pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
                assert!(n < 2usize);
                let offs = 0u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
            #[doc = "Input capture 1 prescaler"]
            pub fn icpsc(&self, n: usize) -> u8 {
                assert!(n < 2usize);
                let offs = 2u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x03;
                val as u8
            }
            #[doc = "Input capture 1 prescaler"]
            pub fn set_icpsc(&mut self, n: usize, val: u8) {
                assert!(n < 2usize);
                let offs = 2u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
            }
            #[doc = "Input capture 1 filter"]
            pub fn icf(&self, n: usize) -> super::vals::Icf {
                assert!(n < 2usize);
                let offs = 4u32 + (n as u32) * 8u32;
                let val = (self.0 >> offs) & 0x0f;
                super::vals::Icf(val as u8)
            }
            #[doc = "Input capture 1 filter"]
            pub fn set_icf(&mut self, n: usize, val: super::vals::Icf) {
                assert!(n < 2usize);
                let offs = 4u32 + (n as u32) * 8u32;
                self.0 = (self.0 & !(0x0f << offs)) | (((val.0 as u32) & 0x0f) << offs);
            }
        }
        impl Default for CcmrInput {
            fn default() -> CcmrInput {
                CcmrInput(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr2Gp(pub u32);
        impl Cr2Gp {
            #[doc = "Capture/compare DMA selection"]
            pub const fn ccds(&self) -> super::vals::Ccds {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Ccds(val as u8)
            }
            #[doc = "Capture/compare DMA selection"]
            pub fn set_ccds(&mut self, val: super::vals::Ccds) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4u32) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4u32)) | (((val.0 as u32) & 0x07) << 4u32);
            }
            #[doc = "TI1 selection"]
            pub const fn ti1s(&self) -> super::vals::Tis {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Tis(val as u8)
            }
            #[doc = "TI1 selection"]
            pub fn set_ti1s(&mut self, val: super::vals::Tis) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
        }
        impl Default for Cr2Gp {
            fn default() -> Cr2Gp {
                Cr2Gp(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr1Basic(pub u32);
        impl Cr1Basic {
            #[doc = "Counter enable"]
            pub const fn cen(&self) -> super::vals::Cen {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Cen(val as u8)
            }
            #[doc = "Counter enable"]
            pub fn set_cen(&mut self, val: super::vals::Cen) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Update disable"]
            pub const fn udis(&self) -> super::vals::Udis {
                let val = (self.0 >> 1u32) & 0x01;
                super::vals::Udis(val as u8)
            }
            #[doc = "Update disable"]
            pub fn set_udis(&mut self, val: super::vals::Udis) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
            }
            #[doc = "Update request source"]
            pub const fn urs(&self) -> super::vals::Urs {
                let val = (self.0 >> 2u32) & 0x01;
                super::vals::Urs(val as u8)
            }
            #[doc = "Update request source"]
            pub fn set_urs(&mut self, val: super::vals::Urs) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
            }
            #[doc = "One-pulse mode"]
            pub const fn opm(&self) -> super::vals::Opm {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Opm(val as u8)
            }
            #[doc = "One-pulse mode"]
            pub fn set_opm(&mut self, val: super::vals::Opm) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "Auto-reload preload enable"]
            pub const fn arpe(&self) -> super::vals::Arpe {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Arpe(val as u8)
            }
            #[doc = "Auto-reload preload enable"]
            pub fn set_arpe(&mut self, val: super::vals::Arpe) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
        }
        impl Default for Cr1Basic {
            fn default() -> Cr1Basic {
                Cr1Basic(0)
            }
        }
        #[doc = "capture/compare enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct CcerGp(pub u32);
        impl CcerGp {
            #[doc = "Capture/Compare 1 output enable"]
            pub fn cce(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output enable"]
            pub fn set_cce(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn ccnp(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 output Polarity"]
            pub fn set_ccnp(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for CcerGp {
            fn default() -> CcerGp {
                CcerGp(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct EgrBasic(pub u32);
        impl EgrBasic {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> super::vals::Ug {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Ug(val as u8)
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: super::vals::Ug) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
        }
        impl Default for EgrBasic {
            fn default() -> EgrBasic {
                EgrBasic(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct EgrAdv(pub u32);
        impl EgrAdv {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> super::vals::Ug {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Ug(val as u8)
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: super::vals::Ug) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn ccg(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn set_ccg(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare control update generation"]
            pub const fn comg(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare control update generation"]
            pub fn set_comg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Trigger generation"]
            pub const fn tg(&self) -> bool {
                let val = (self.0 >> 6u32) & 0x01;
                val != 0
            }
            #[doc = "Trigger generation"]
            pub fn set_tg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
            }
            #[doc = "Break generation"]
            pub const fn bg(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Break generation"]
            pub fn set_bg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
        }
        impl Default for EgrAdv {
            fn default() -> EgrAdv {
                EgrAdv(0)
            }
        }
        #[doc = "DMA/Interrupt enable register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct DierAdv(pub u32);
        impl DierAdv {
            #[doc = "Update interrupt enable"]
            pub const fn uie(&self) -> super::vals::Uie {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uie(val as u8)
            }
            #[doc = "Update interrupt enable"]
            pub fn set_uie(&mut self, val: super::vals::Uie) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn ccie(&self, n: usize) -> super::vals::Ccie {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ccie(val as u8)
            }
            #[doc = "Capture/Compare 1 interrupt enable"]
            pub fn set_ccie(&mut self, n: usize, val: super::vals::Ccie) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt enable"]
            pub const fn comie(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt enable"]
            pub fn set_comie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Trigger interrupt enable"]
            pub const fn tie(&self) -> super::vals::Tie {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Tie(val as u8)
            }
            #[doc = "Trigger interrupt enable"]
            pub fn set_tie(&mut self, val: super::vals::Tie) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
            }
            #[doc = "Break interrupt enable"]
            pub const fn bie(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt enable"]
            pub fn set_bie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
            #[doc = "Update DMA request enable"]
            pub const fn ude(&self) -> super::vals::Ude {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Ude(val as u8)
            }
            #[doc = "Update DMA request enable"]
            pub fn set_ude(&mut self, val: super::vals::Ude) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn ccde(&self, n: usize) -> super::vals::Ccde {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ccde(val as u8)
            }
            #[doc = "Capture/Compare 1 DMA request enable"]
            pub fn set_ccde(&mut self, n: usize, val: super::vals::Ccde) {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "COM DMA request enable"]
            pub const fn comde(&self) -> bool {
                let val = (self.0 >> 13u32) & 0x01;
                val != 0
            }
            #[doc = "COM DMA request enable"]
            pub fn set_comde(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
            }
            #[doc = "Trigger DMA request enable"]
            pub const fn tde(&self) -> super::vals::Tde {
                let val = (self.0 >> 14u32) & 0x01;
                super::vals::Tde(val as u8)
            }
            #[doc = "Trigger DMA request enable"]
            pub fn set_tde(&mut self, val: super::vals::Tde) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
            }
        }
        impl Default for DierAdv {
            fn default() -> DierAdv {
                DierAdv(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr2Adv(pub u32);
        impl Cr2Adv {
            #[doc = "Capture/compare preloaded control"]
            pub const fn ccpc(&self) -> bool {
                let val = (self.0 >> 0u32) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare preloaded control"]
            pub fn set_ccpc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/compare control update selection"]
            pub const fn ccus(&self) -> bool {
                let val = (self.0 >> 2u32) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare control update selection"]
            pub fn set_ccus(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
            }
            #[doc = "Capture/compare DMA selection"]
            pub const fn ccds(&self) -> super::vals::Ccds {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Ccds(val as u8)
            }
            #[doc = "Capture/compare DMA selection"]
            pub fn set_ccds(&mut self, val: super::vals::Ccds) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "Master mode selection"]
            pub const fn mms(&self) -> super::vals::Mms {
                let val = (self.0 >> 4u32) & 0x07;
                super::vals::Mms(val as u8)
            }
            #[doc = "Master mode selection"]
            pub fn set_mms(&mut self, val: super::vals::Mms) {
                self.0 = (self.0 & !(0x07 << 4u32)) | (((val.0 as u32) & 0x07) << 4u32);
            }
            #[doc = "TI1 selection"]
            pub const fn ti1s(&self) -> super::vals::Tis {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Tis(val as u8)
            }
            #[doc = "TI1 selection"]
            pub fn set_ti1s(&mut self, val: super::vals::Tis) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
            #[doc = "Output Idle state 1"]
            pub fn ois(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 8u32 + (n as u32) * 2u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 1"]
            pub fn set_ois(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 8u32 + (n as u32) * 2u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Output Idle state 1"]
            pub const fn ois1n(&self) -> bool {
                let val = (self.0 >> 9u32) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 1"]
            pub fn set_ois1n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
            }
            #[doc = "Output Idle state 2"]
            pub const fn ois2n(&self) -> bool {
                let val = (self.0 >> 11u32) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 2"]
            pub fn set_ois2n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
            }
            #[doc = "Output Idle state 3"]
            pub const fn ois3n(&self) -> bool {
                let val = (self.0 >> 13u32) & 0x01;
                val != 0
            }
            #[doc = "Output Idle state 3"]
            pub fn set_ois3n(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
            }
        }
        impl Default for Cr2Adv {
            fn default() -> Cr2Adv {
                Cr2Adv(0)
            }
        }
        #[doc = "capture/compare register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ccr32(pub u32);
        impl Ccr32 {
            #[doc = "Capture/Compare 1 value"]
            pub const fn ccr(&self) -> u32 {
                let val = (self.0 >> 0u32) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Capture/Compare 1 value"]
            pub fn set_ccr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
            }
        }
        impl Default for Ccr32 {
            fn default() -> Ccr32 {
                Ccr32(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct SrGp(pub u32);
        impl SrGp {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> super::vals::Uif {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uif(val as u8)
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: super::vals::Uif) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn ccif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn set_ccif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt flag"]
            pub const fn comif(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt flag"]
            pub fn set_comif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Trigger interrupt flag"]
            pub const fn tif(&self) -> bool {
                let val = (self.0 >> 6u32) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt flag"]
            pub fn set_tif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
            }
            #[doc = "Break interrupt flag"]
            pub const fn bif(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt flag"]
            pub fn set_bif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn ccof(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn set_ccof(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for SrGp {
            fn default() -> SrGp {
                SrGp(0)
            }
        }
        #[doc = "prescaler"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Psc(pub u32);
        impl Psc {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0xffff;
                val as u16
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
            }
        }
        impl Default for Psc {
            fn default() -> Psc {
                Psc(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct SrAdv(pub u32);
        impl SrAdv {
            #[doc = "Update interrupt flag"]
            pub const fn uif(&self) -> super::vals::Uif {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Uif(val as u8)
            }
            #[doc = "Update interrupt flag"]
            pub fn set_uif(&mut self, val: super::vals::Uif) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn ccif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 interrupt flag"]
            pub fn set_ccif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "COM interrupt flag"]
            pub const fn comif(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "COM interrupt flag"]
            pub fn set_comif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Trigger interrupt flag"]
            pub const fn tif(&self) -> bool {
                let val = (self.0 >> 6u32) & 0x01;
                val != 0
            }
            #[doc = "Trigger interrupt flag"]
            pub fn set_tif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
            }
            #[doc = "Break interrupt flag"]
            pub const fn bif(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Break interrupt flag"]
            pub fn set_bif(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn ccof(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare 1 overcapture flag"]
            pub fn set_ccof(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 9u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for SrAdv {
            fn default() -> SrAdv {
                SrAdv(0)
            }
        }
        #[doc = "repetition counter register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Rcr(pub u32);
        impl Rcr {
            #[doc = "Repetition counter value"]
            pub const fn rep(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0xff;
                val as u8
            }
            #[doc = "Repetition counter value"]
            pub fn set_rep(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
            }
        }
        impl Default for Rcr {
            fn default() -> Rcr {
                Rcr(0)
            }
        }
        #[doc = "DMA control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dcr(pub u32);
        impl Dcr {
            #[doc = "DMA base address"]
            pub const fn dba(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0x1f;
                val as u8
            }
            #[doc = "DMA base address"]
            pub fn set_dba(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
            }
            #[doc = "DMA burst length"]
            pub const fn dbl(&self) -> u8 {
                let val = (self.0 >> 8u32) & 0x1f;
                val as u8
            }
            #[doc = "DMA burst length"]
            pub fn set_dbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1f << 8u32)) | (((val as u32) & 0x1f) << 8u32);
            }
        }
        impl Default for Dcr {
            fn default() -> Dcr {
                Dcr(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr1Gp(pub u32);
        impl Cr1Gp {
            #[doc = "Counter enable"]
            pub const fn cen(&self) -> super::vals::Cen {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Cen(val as u8)
            }
            #[doc = "Counter enable"]
            pub fn set_cen(&mut self, val: super::vals::Cen) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Update disable"]
            pub const fn udis(&self) -> super::vals::Udis {
                let val = (self.0 >> 1u32) & 0x01;
                super::vals::Udis(val as u8)
            }
            #[doc = "Update disable"]
            pub fn set_udis(&mut self, val: super::vals::Udis) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
            }
            #[doc = "Update request source"]
            pub const fn urs(&self) -> super::vals::Urs {
                let val = (self.0 >> 2u32) & 0x01;
                super::vals::Urs(val as u8)
            }
            #[doc = "Update request source"]
            pub fn set_urs(&mut self, val: super::vals::Urs) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
            }
            #[doc = "One-pulse mode"]
            pub const fn opm(&self) -> super::vals::Opm {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Opm(val as u8)
            }
            #[doc = "One-pulse mode"]
            pub fn set_opm(&mut self, val: super::vals::Opm) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
            }
            #[doc = "Direction"]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 4u32) & 0x01;
                super::vals::Dir(val as u8)
            }
            #[doc = "Direction"]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
            }
            #[doc = "Center-aligned mode selection"]
            pub const fn cms(&self) -> super::vals::Cms {
                let val = (self.0 >> 5u32) & 0x03;
                super::vals::Cms(val as u8)
            }
            #[doc = "Center-aligned mode selection"]
            pub fn set_cms(&mut self, val: super::vals::Cms) {
                self.0 = (self.0 & !(0x03 << 5u32)) | (((val.0 as u32) & 0x03) << 5u32);
            }
            #[doc = "Auto-reload preload enable"]
            pub const fn arpe(&self) -> super::vals::Arpe {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Arpe(val as u8)
            }
            #[doc = "Auto-reload preload enable"]
            pub fn set_arpe(&mut self, val: super::vals::Arpe) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
            #[doc = "Clock division"]
            pub const fn ckd(&self) -> super::vals::Ckd {
                let val = (self.0 >> 8u32) & 0x03;
                super::vals::Ckd(val as u8)
            }
            #[doc = "Clock division"]
            pub fn set_ckd(&mut self, val: super::vals::Ckd) {
                self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
            }
        }
        impl Default for Cr1Gp {
            fn default() -> Cr1Gp {
                Cr1Gp(0)
            }
        }
        #[doc = "counter"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cnt16(pub u32);
        impl Cnt16 {
            #[doc = "counter value"]
            pub const fn cnt(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0xffff;
                val as u16
            }
            #[doc = "counter value"]
            pub fn set_cnt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
            }
        }
        impl Default for Cnt16 {
            fn default() -> Cnt16 {
                Cnt16(0)
            }
        }
        #[doc = "auto-reload register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Arr16(pub u32);
        impl Arr16 {
            #[doc = "Auto-reload value"]
            pub const fn arr(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0xffff;
                val as u16
            }
            #[doc = "Auto-reload value"]
            pub fn set_arr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
            }
        }
        impl Default for Arr16 {
            fn default() -> Arr16 {
                Arr16(0)
            }
        }
        #[doc = "auto-reload register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Arr32(pub u32);
        impl Arr32 {
            #[doc = "Auto-reload value"]
            pub const fn arr(&self) -> u32 {
                let val = (self.0 >> 0u32) & 0xffff_ffff;
                val as u32
            }
            #[doc = "Auto-reload value"]
            pub fn set_arr(&mut self, val: u32) {
                self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
            }
        }
        impl Default for Arr32 {
            fn default() -> Arr32 {
                Arr32(0)
            }
        }
        #[doc = "DMA address for full transfer"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Dmar(pub u32);
        impl Dmar {
            #[doc = "DMA register for burst accesses"]
            pub const fn dmab(&self) -> u16 {
                let val = (self.0 >> 0u32) & 0xffff;
                val as u16
            }
            #[doc = "DMA register for burst accesses"]
            pub fn set_dmab(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
            }
        }
        impl Default for Dmar {
            fn default() -> Dmar {
                Dmar(0)
            }
        }
        #[doc = "event generation register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct EgrGp(pub u32);
        impl EgrGp {
            #[doc = "Update generation"]
            pub const fn ug(&self) -> super::vals::Ug {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Ug(val as u8)
            }
            #[doc = "Update generation"]
            pub fn set_ug(&mut self, val: super::vals::Ug) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn ccg(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Capture/compare 1 generation"]
            pub fn set_ccg(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 1u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Capture/Compare control update generation"]
            pub const fn comg(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "Capture/Compare control update generation"]
            pub fn set_comg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
            }
            #[doc = "Trigger generation"]
            pub const fn tg(&self) -> bool {
                let val = (self.0 >> 6u32) & 0x01;
                val != 0
            }
            #[doc = "Trigger generation"]
            pub fn set_tg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
            }
            #[doc = "Break generation"]
            pub const fn bg(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Break generation"]
            pub fn set_bg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
            }
        }
        impl Default for EgrGp {
            fn default() -> EgrGp {
                EgrGp(0)
            }
        }
        #[doc = "slave mode control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Smcr(pub u32);
        impl Smcr {
            #[doc = "Slave mode selection"]
            pub const fn sms(&self) -> super::vals::Sms {
                let val = (self.0 >> 0u32) & 0x07;
                super::vals::Sms(val as u8)
            }
            #[doc = "Slave mode selection"]
            pub fn set_sms(&mut self, val: super::vals::Sms) {
                self.0 = (self.0 & !(0x07 << 0u32)) | (((val.0 as u32) & 0x07) << 0u32);
            }
            #[doc = "Trigger selection"]
            pub const fn ts(&self) -> super::vals::Ts {
                let val = (self.0 >> 4u32) & 0x07;
                super::vals::Ts(val as u8)
            }
            #[doc = "Trigger selection"]
            pub fn set_ts(&mut self, val: super::vals::Ts) {
                self.0 = (self.0 & !(0x07 << 4u32)) | (((val.0 as u32) & 0x07) << 4u32);
            }
            #[doc = "Master/Slave mode"]
            pub const fn msm(&self) -> super::vals::Msm {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Msm(val as u8)
            }
            #[doc = "Master/Slave mode"]
            pub fn set_msm(&mut self, val: super::vals::Msm) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
            }
            #[doc = "External trigger filter"]
            pub const fn etf(&self) -> super::vals::Etf {
                let val = (self.0 >> 8u32) & 0x0f;
                super::vals::Etf(val as u8)
            }
            #[doc = "External trigger filter"]
            pub fn set_etf(&mut self, val: super::vals::Etf) {
                self.0 = (self.0 & !(0x0f << 8u32)) | (((val.0 as u32) & 0x0f) << 8u32);
            }
            #[doc = "External trigger prescaler"]
            pub const fn etps(&self) -> super::vals::Etps {
                let val = (self.0 >> 12u32) & 0x03;
                super::vals::Etps(val as u8)
            }
            #[doc = "External trigger prescaler"]
            pub fn set_etps(&mut self, val: super::vals::Etps) {
                self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
            }
            #[doc = "External clock enable"]
            pub const fn ece(&self) -> super::vals::Ece {
                let val = (self.0 >> 14u32) & 0x01;
                super::vals::Ece(val as u8)
            }
            #[doc = "External clock enable"]
            pub fn set_ece(&mut self, val: super::vals::Ece) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
            }
            #[doc = "External trigger polarity"]
            pub const fn etp(&self) -> super::vals::Etp {
                let val = (self.0 >> 15u32) & 0x01;
                super::vals::Etp(val as u8)
            }
            #[doc = "External trigger polarity"]
            pub fn set_etp(&mut self, val: super::vals::Etp) {
                self.0 = (self.0 & !(0x01 << 15u32)) | (((val.0 as u32) & 0x01) << 15u32);
            }
        }
        impl Default for Smcr {
            fn default() -> Smcr {
                Smcr(0)
            }
        }
    }
}
pub mod gpio_v2 {
    use crate::generic::*;
    #[doc = "General-purpose I/Os"]
    #[derive(Copy, Clone)]
    pub struct Gpio(pub *mut u8);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[doc = "GPIO port mode register"]
        pub fn moder(self) -> Reg<regs::Moder, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "GPIO port output type register"]
        pub fn otyper(self) -> Reg<regs::Otyper, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "GPIO port output speed register"]
        pub fn ospeedr(self) -> Reg<regs::Ospeedr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        pub fn pupdr(self) -> Reg<regs::Pupdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "GPIO port input data register"]
        pub fn idr(self) -> Reg<regs::Idr, R> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "GPIO port output data register"]
        pub fn odr(self) -> Reg<regs::Odr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "GPIO port bit set/reset register"]
        pub fn bsrr(self) -> Reg<regs::Bsrr, W> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "GPIO port configuration lock register"]
        pub fn lckr(self) -> Reg<regs::Lckr, RW> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "GPIO alternate function register (low, high)"]
        pub fn afr(self, n: usize) -> Reg<regs::Afr, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "GPIO port output speed register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ospeedr(pub u32);
        impl Ospeedr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ospeedr(&self, n: usize) -> super::vals::Ospeedr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Ospeedr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ospeedr(&mut self, n: usize, val: super::vals::Ospeedr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Ospeedr {
            fn default() -> Ospeedr {
                Ospeedr(0)
            }
        }
        #[doc = "GPIO port mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Moder(pub u32);
        impl Moder {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn moder(&self, n: usize) -> super::vals::Moder {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Moder(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_moder(&mut self, n: usize, val: super::vals::Moder) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Moder {
            fn default() -> Moder {
                Moder(0)
            }
        }
        #[doc = "GPIO port input data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idr(pub u32);
        impl Idr {
            #[doc = "Port input data (y = 0..15)"]
            pub fn idr(&self, n: usize) -> super::vals::Idr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Idr(val as u8)
            }
            #[doc = "Port input data (y = 0..15)"]
            pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Idr {
            fn default() -> Idr {
                Idr(0)
            }
        }
        #[doc = "GPIO port output type register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Otyper(pub u32);
        impl Otyper {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ot(&self, n: usize) -> super::vals::Ot {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ot(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ot(&mut self, n: usize, val: super::vals::Ot) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Otyper {
            fn default() -> Otyper {
                Otyper(0)
            }
        }
        #[doc = "GPIO port bit set/reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 16u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
            }
        }
        #[doc = "GPIO port output data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Odr(pub u32);
        impl Odr {
            #[doc = "Port output data (y = 0..15)"]
            pub fn odr(&self, n: usize) -> super::vals::Odr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Odr(val as u8)
            }
            #[doc = "Port output data (y = 0..15)"]
            pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Odr {
            fn default() -> Odr {
                Odr(0)
            }
        }
        #[doc = "GPIO alternate function register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Afr(pub u32);
        impl Afr {
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn afr(&self, n: usize) -> super::vals::Afr {
                assert!(n < 8usize);
                let offs = 0u32 + (n as u32) * 4u32;
                let val = (self.0 >> offs) & 0x0f;
                super::vals::Afr(val as u8)
            }
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn set_afr(&mut self, n: usize, val: super::vals::Afr) {
                assert!(n < 8usize);
                let offs = 0u32 + (n as u32) * 4u32;
                self.0 = (self.0 & !(0x0f << offs)) | (((val.0 as u32) & 0x0f) << offs);
            }
        }
        impl Default for Afr {
            fn default() -> Afr {
                Afr(0)
            }
        }
        #[doc = "GPIO port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16u32) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16u32)) | (((val.0 as u32) & 0x01) << 16u32);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Pupdr(pub u32);
        impl Pupdr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn pupdr(&self, n: usize) -> super::vals::Pupdr {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Pupdr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_pupdr(&mut self, n: usize, val: super::vals::Pupdr) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 2u32;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Pupdr {
            fn default() -> Pupdr {
                Pupdr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Moder(pub u8);
        impl Moder {
            #[doc = "Input mode (reset state)"]
            pub const INPUT: Self = Self(0);
            #[doc = "General purpose output mode"]
            pub const OUTPUT: Self = Self(0x01);
            #[doc = "Alternate function mode"]
            pub const ALTERNATE: Self = Self(0x02);
            #[doc = "Analog mode"]
            pub const ANALOG: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "Resets the corresponding ODRx bit"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ospeedr(pub u8);
        impl Ospeedr {
            #[doc = "Low speed"]
            pub const LOWSPEED: Self = Self(0);
            #[doc = "Medium speed"]
            pub const MEDIUMSPEED: Self = Self(0x01);
            #[doc = "High speed"]
            pub const HIGHSPEED: Self = Self(0x02);
            #[doc = "Very high speed"]
            pub const VERYHIGHSPEED: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsw(pub u8);
        impl Bsw {
            #[doc = "Sets the corresponding ODRx bit"]
            pub const SET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Pupdr(pub u8);
        impl Pupdr {
            #[doc = "No pull-up, pull-down"]
            pub const FLOATING: Self = Self(0);
            #[doc = "Pull-up"]
            pub const PULLUP: Self = Self(0x01);
            #[doc = "Pull-down"]
            pub const PULLDOWN: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Afr(pub u8);
        impl Afr {
            #[doc = "AF0"]
            pub const AF0: Self = Self(0);
            #[doc = "AF1"]
            pub const AF1: Self = Self(0x01);
            #[doc = "AF2"]
            pub const AF2: Self = Self(0x02);
            #[doc = "AF3"]
            pub const AF3: Self = Self(0x03);
            #[doc = "AF4"]
            pub const AF4: Self = Self(0x04);
            #[doc = "AF5"]
            pub const AF5: Self = Self(0x05);
            #[doc = "AF6"]
            pub const AF6: Self = Self(0x06);
            #[doc = "AF7"]
            pub const AF7: Self = Self(0x07);
            #[doc = "AF8"]
            pub const AF8: Self = Self(0x08);
            #[doc = "AF9"]
            pub const AF9: Self = Self(0x09);
            #[doc = "AF10"]
            pub const AF10: Self = Self(0x0a);
            #[doc = "AF11"]
            pub const AF11: Self = Self(0x0b);
            #[doc = "AF12"]
            pub const AF12: Self = Self(0x0c);
            #[doc = "AF13"]
            pub const AF13: Self = Self(0x0d);
            #[doc = "AF14"]
            pub const AF14: Self = Self(0x0e);
            #[doc = "AF15"]
            pub const AF15: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Ot(pub u8);
        impl Ot {
            #[doc = "Output push-pull (reset state)"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Output open-drain"]
            pub const OPENDRAIN: Self = Self(0x01);
        }
    }
}
