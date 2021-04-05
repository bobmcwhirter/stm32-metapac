#![no_std]
#![doc = "Peripheral access API (generated using svd2rust v0.17.0 (426f4c8 2021-03-17))"]
pub mod usart_v1 {
    use crate::generic::*;
    #[doc = "Universal synchronous asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Usart1(pub *mut u8);
    unsafe impl Send for Usart1 {}
    unsafe impl Sync for Usart1 {}
    impl Usart1 {
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
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "USART enable"]
            pub const fn ue(&self) -> super::vals::Ue {
                let val = (self.0 >> 13u32) & 0x01;
                super::vals::Ue(val as u8)
            }
            #[doc = "USART enable"]
            pub fn set_ue(&mut self, val: super::vals::Ue) {
                self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
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
            #[doc = "Wakeup method"]
            pub const fn wake(&self) -> super::vals::Wake {
                let val = (self.0 >> 11u32) & 0x01;
                super::vals::Wake(val as u8)
            }
            #[doc = "Wakeup method"]
            pub fn set_wake(&mut self, val: super::vals::Wake) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
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
            #[doc = "Parity selection"]
            pub const fn ps(&self) -> super::vals::Ps {
                let val = (self.0 >> 9u32) & 0x01;
                super::vals::Ps(val as u8)
            }
            #[doc = "Parity selection"]
            pub fn set_ps(&mut self, val: super::vals::Ps) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
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
            #[doc = "TXE interrupt enable"]
            pub const fn txeie(&self) -> super::vals::Txeie {
                let val = (self.0 >> 7u32) & 0x01;
                super::vals::Txeie(val as u8)
            }
            #[doc = "TXE interrupt enable"]
            pub fn set_txeie(&mut self, val: super::vals::Txeie) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
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
            #[doc = "RXNE interrupt enable"]
            pub const fn rxneie(&self) -> super::vals::Rxneie {
                let val = (self.0 >> 5u32) & 0x01;
                super::vals::Rxneie(val as u8)
            }
            #[doc = "RXNE interrupt enable"]
            pub fn set_rxneie(&mut self, val: super::vals::Rxneie) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
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
            #[doc = "Transmitter enable"]
            pub const fn te(&self) -> super::vals::Te {
                let val = (self.0 >> 3u32) & 0x01;
                super::vals::Te(val as u8)
            }
            #[doc = "Transmitter enable"]
            pub fn set_te(&mut self, val: super::vals::Te) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
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
            #[doc = "Receiver wakeup"]
            pub const fn rwu(&self) -> super::vals::Rwu {
                let val = (self.0 >> 1u32) & 0x01;
                super::vals::Rwu(val as u8)
            }
            #[doc = "Receiver wakeup"]
            pub fn set_rwu(&mut self, val: super::vals::Rwu) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
            }
            #[doc = "Send break"]
            pub const fn sbk(&self) -> super::vals::Sbk {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Sbk(val as u8)
            }
            #[doc = "Send break"]
            pub fn set_sbk(&mut self, val: super::vals::Sbk) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
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
        #[doc = "Baud rate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "mantissa of USARTDIV"]
            pub const fn div_mantissa(&self) -> u16 {
                let val = (self.0 >> 4u32) & 0x0fff;
                val as u16
            }
            #[doc = "mantissa of USARTDIV"]
            pub fn set_div_mantissa(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 4u32)) | (((val as u32) & 0x0fff) << 4u32);
            }
            #[doc = "fraction of USARTDIV"]
            pub const fn div_fraction(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0x0f;
                val as u8
            }
            #[doc = "fraction of USARTDIV"]
            pub fn set_div_fraction(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "CTS flag"]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 9u32) & 0x01;
                val != 0
            }
            #[doc = "CTS flag"]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
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
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7u32) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
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
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5u32) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
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
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3u32) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
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
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1u32) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
            }
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0u32) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr3(pub u32);
        impl Cr3 {
            #[doc = "CTS interrupt enable"]
            pub const fn ctsie(&self) -> super::vals::Ctsie {
                let val = (self.0 >> 10u32) & 0x01;
                super::vals::Ctsie(val as u8)
            }
            #[doc = "CTS interrupt enable"]
            pub fn set_ctsie(&mut self, val: super::vals::Ctsie) {
                self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
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
            #[doc = "RTS enable"]
            pub const fn rtse(&self) -> super::vals::Rtse {
                let val = (self.0 >> 8u32) & 0x01;
                super::vals::Rtse(val as u8)
            }
            #[doc = "RTS enable"]
            pub fn set_rtse(&mut self, val: super::vals::Rtse) {
                self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
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
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> super::vals::Dmar {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Dmar(val as u8)
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: super::vals::Dmar) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
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
            #[doc = "Smartcard NACK enable"]
            pub const fn nack(&self) -> super::vals::Nack {
                let val = (self.0 >> 4u32) & 0x01;
                super::vals::Nack(val as u8)
            }
            #[doc = "Smartcard NACK enable"]
            pub fn set_nack(&mut self, val: super::vals::Nack) {
                self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
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
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2u32) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
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
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> super::vals::Eie {
                let val = (self.0 >> 0u32) & 0x01;
                super::vals::Eie(val as u8)
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: super::vals::Eie) {
                self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
            }
        }
        impl Default for Cr3 {
            fn default() -> Cr3 {
                Cr3(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> super::vals::Linen {
                let val = (self.0 >> 14u32) & 0x01;
                super::vals::Linen(val as u8)
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: super::vals::Linen) {
                self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
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
            #[doc = "Clock enable"]
            pub const fn clken(&self) -> super::vals::Clken {
                let val = (self.0 >> 11u32) & 0x01;
                super::vals::Clken(val as u8)
            }
            #[doc = "Clock enable"]
            pub fn set_clken(&mut self, val: super::vals::Clken) {
                self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
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
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 9u32) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
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
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> super::vals::Lbdie {
                let val = (self.0 >> 6u32) & 0x01;
                super::vals::Lbdie(val as u8)
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: super::vals::Lbdie) {
                self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
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
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Guard time and prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Gtpr(pub u32);
        impl Gtpr {
            #[doc = "Guard time value"]
            pub const fn gt(&self) -> u8 {
                let val = (self.0 >> 8u32) & 0xff;
                val as u8
            }
            #[doc = "Guard time value"]
            pub fn set_gt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
            }
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u8 {
                let val = (self.0 >> 0u32) & 0xff;
                val as u8
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
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
        pub struct Scen(pub u8);
        impl Scen {
            #[doc = "Smartcard mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Smartcard mode enabled"]
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
        pub struct Te(pub u8);
        impl Te {
            #[doc = "Transmitter disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Transmitter enabled"]
            pub const ENABLED: Self = Self(0x01);
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
        pub struct Re(pub u8);
        impl Re {
            #[doc = "Receiver disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Receiver enabled"]
            pub const ENABLED: Self = Self(0x01);
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
        pub struct Rxneie(pub u8);
        impl Rxneie {
            #[doc = "RXNE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "RXNE interrupt enabled"]
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
        pub struct Tcie(pub u8);
        impl Tcie {
            #[doc = "TC interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "TC interrupt enabled"]
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
        pub struct Ps(pub u8);
        impl Ps {
            #[doc = "Even parity"]
            pub const EVEN: Self = Self(0);
            #[doc = "Odd parity"]
            pub const ODD: Self = Self(0x01);
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
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRST: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECOND: Self = Self(0x01);
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
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "Steady low value on CK pin outside transmission window"]
            pub const LOW: Self = Self(0);
            #[doc = "Steady high value on CK pin outside transmission window"]
            pub const HIGH: Self = Self(0x01);
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
        pub struct Idleie(pub u8);
        impl Idleie {
            #[doc = "IDLE interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "IDLE interrupt enabled"]
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
        pub struct Ctsie(pub u8);
        impl Ctsie {
            #[doc = "CTS interrupt disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CTS interrupt enabled"]
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
        pub struct Lbdl(pub u8);
        impl Lbdl {
            #[doc = "10-bit break detection"]
            pub const LBDL10: Self = Self(0);
            #[doc = "11-bit break detection"]
            pub const LBDL11: Self = Self(0x01);
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
        pub struct Clken(pub u8);
        impl Clken {
            #[doc = "CK pin disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "CK pin enabled"]
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
        pub struct Linen(pub u8);
        impl Linen {
            #[doc = "LIN mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "LIN mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
    }
}
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
    pub struct Gpioa(pub *mut u8);
    unsafe impl Send for Gpioa {}
    unsafe impl Sync for Gpioa {}
    impl Gpioa {
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
        #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Set bit 0"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Set bit 0"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Reset bit 0"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit 0"]
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
        #[doc = "Port bit reset register (GPIOn_BRR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "Reset bit 0"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Reset bit 0"]
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
        #[doc = "Port configuration register (GPIOn_CRx)"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Cr(pub u32);
        impl Cr {
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
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
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
        #[doc = "Port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Lock key"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16u32) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Lock key"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16u32)) | (((val.0 as u32) & 0x01) << 16u32);
            }
            #[doc = "Port A Lock bit 0"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port A Lock bit 0"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0u32 + (n as u32) * 1u32;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
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
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
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
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "No action on the corresponding ODx bit"]
            pub const NOACTION: Self = Self(0);
            #[doc = "Reset the ODx bit"]
            pub const RESET: Self = Self(0x01);
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
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
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
        pub struct Ot(pub u8);
        impl Ot {
            #[doc = "Output push-pull (reset state)"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Output open-drain"]
            pub const OPENDRAIN: Self = Self(0x01);
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
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
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
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
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
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "GPIO port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16u32) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16u32)) | (((val.0 as u32) & 0x01) << 16u32);
            }
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
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "GPIO port bit set/reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
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
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
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
    }
}
