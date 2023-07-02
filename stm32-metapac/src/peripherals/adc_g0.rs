#[doc = "Analog to Digital Converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "ADC control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "ADC sampling time register"]
    #[inline(always)]
    pub const fn smpr(self) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd1tr(self) -> crate::common::Reg<regs::Awd1tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd2tr(self) -> crate::common::Reg<regs::Awd2tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "channel selection register"]
    #[inline(always)]
    pub const fn chselr(self) -> crate::common::Reg<regs::Chselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[inline(always)]
    pub const fn chselr_1(self) -> crate::common::Reg<regs::Chselr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "watchdog threshold register"]
    #[inline(always)]
    pub const fn awd3tr(self) -> crate::common::Reg<regs::Awd3tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "ADC group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "ADC calibration factors register"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr6(self) -> crate::common::Reg<regs::Hwcfgr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(984usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr5(self) -> crate::common::Reg<regs::Hwcfgr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(988usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr4(self) -> crate::common::Reg<regs::Hwcfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(992usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr3(self) -> crate::common::Reg<regs::Hwcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(996usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr2(self) -> crate::common::Reg<regs::Hwcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1000usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr1(self) -> crate::common::Reg<regs::Hwcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1004usize) as _) }
    }
    #[doc = "Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hwcfgr0(self) -> crate::common::Reg<regs::Hwcfgr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1008usize) as _) }
    }
    #[doc = "EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::Verr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1012usize) as _) }
    }
    #[doc = "EXTI Identification register"]
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::Ipidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1016usize) as _) }
    }
    #[doc = "EXTI Size ID register"]
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::Sidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1020usize) as _) }
    }
}
pub mod regs {
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd1tr(pub u32);
    impl Awd1tr {
        #[doc = "ADC analog watchdog 1 threshold low"]
        #[inline(always)]
        pub const fn lt1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 1 threshold low"]
        #[inline(always)]
        pub fn set_lt1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 1 threshold high"]
        #[inline(always)]
        pub const fn ht1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 1 threshold high"]
        #[inline(always)]
        pub fn set_ht1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd1tr {
        #[inline(always)]
        fn default() -> Awd1tr {
            Awd1tr(0)
        }
    }
    #[doc = "ADC analog watchdog 2 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "ADC analog watchdog 2 monitored channel selection"]
        #[inline(always)]
        pub const fn awd2ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ADC analog watchdog 2 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd2ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for Awd2cr {
        #[inline(always)]
        fn default() -> Awd2cr {
            Awd2cr(0)
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2tr(pub u32);
    impl Awd2tr {
        #[doc = "ADC analog watchdog 2 threshold low"]
        #[inline(always)]
        pub const fn lt2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 2 threshold low"]
        #[inline(always)]
        pub fn set_lt2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 2 threshold high"]
        #[inline(always)]
        pub const fn ht2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 2 threshold high"]
        #[inline(always)]
        pub fn set_ht2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd2tr {
        #[inline(always)]
        fn default() -> Awd2tr {
            Awd2tr(0)
        }
    }
    #[doc = "ADC analog watchdog 3 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "ADC analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub const fn awd3ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "ADC analog watchdog 3 monitored channel selection"]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for Awd3cr {
        #[inline(always)]
        fn default() -> Awd3cr {
            Awd3cr(0)
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3tr(pub u32);
    impl Awd3tr {
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub const fn lt3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub fn set_lt3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub const fn ht3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC analog watchdog 3 threshold high"]
        #[inline(always)]
        pub fn set_ht3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awd3tr {
        #[inline(always)]
        fn default() -> Awd3tr {
            Awd3tr(0)
        }
    }
    #[doc = "ADC calibration factors register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "ADC calibration factor in single-ended mode"]
        #[inline(always)]
        pub const fn calfact(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "ADC calibration factor in single-ended mode"]
        #[inline(always)]
        pub fn set_calfact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Calfact {
        #[inline(always)]
        fn default() -> Calfact {
            Calfact(0)
        }
    }
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub fn set_vbaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "ADC configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "ADC DMA transfer enable"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC DMA transfer enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC DMA transfer configuration"]
        #[inline(always)]
        pub const fn dmacfg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC DMA transfer configuration"]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Scan sequence direction"]
        #[inline(always)]
        pub const fn scandir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Scan sequence direction"]
        #[inline(always)]
        pub fn set_scandir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC data resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "ADC data resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "ADC data alignement"]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC data alignement"]
        #[inline(always)]
        pub fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC group regular external trigger source"]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "ADC group regular external trigger source"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "ADC group regular external trigger polarity"]
        #[inline(always)]
        pub const fn exten(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "ADC group regular external trigger polarity"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC group regular overrun configuration"]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun configuration"]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC group regular continuous conversion mode"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular continuous conversion mode"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Wait conversion mode"]
        #[inline(always)]
        pub const fn wait(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wait conversion mode"]
        #[inline(always)]
        pub fn set_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Auto-off mode"]
        #[inline(always)]
        pub const fn autoff(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-off mode"]
        #[inline(always)]
        pub fn set_autoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular sequencer discontinuous mode"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Mode selection of the ADC_CHSELR register"]
        #[inline(always)]
        pub const fn chselrmod(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Mode selection of the ADC_CHSELR register"]
        #[inline(always)]
        pub fn set_chselrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ADC analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub const fn awdch1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC analog watchdog 1 monitored channel selection"]
        #[inline(always)]
        pub fn set_awdch1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "ADC configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "ADC oversampler enable on scope ADC group regular"]
        #[inline(always)]
        pub const fn ovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC oversampler enable on scope ADC group regular"]
        #[inline(always)]
        pub fn set_ovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC oversampling ratio"]
        #[inline(always)]
        pub const fn ovsr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "ADC oversampling ratio"]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "ADC oversampling shift"]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC oversampling shift"]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
        #[inline(always)]
        pub const fn tovs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
        #[inline(always)]
        pub fn set_tovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low frequency trigger mode enable"]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Low frequency trigger mode enable"]
        #[inline(always)]
        pub fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub const fn ckmode(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "channel selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselr(pub u32);
    impl Chselr {
        #[doc = "Channel-x selection"]
        #[inline(always)]
        pub const fn chsel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Channel-x selection"]
        #[inline(always)]
        pub fn set_chsel(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for Chselr {
        #[inline(always)]
        fn default() -> Chselr {
            Chselr(0)
        }
    }
    #[doc = "channel selection register CHSELRMOD = 1 in ADC_CFGR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselr1(pub u32);
    impl Chselr1 {
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq6(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub const fn sq8(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "conversion of the sequence"]
        #[inline(always)]
        pub fn set_sq8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Chselr1 {
        #[inline(always)]
        fn default() -> Chselr1 {
            Chselr1(0)
        }
    }
    #[doc = "ADC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADC enable"]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC enable"]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC disable"]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC disable"]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular conversion start"]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular conversion start"]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular conversion stop"]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular conversion stop"]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC calibration"]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC calibration"]
        #[inline(always)]
        pub fn set_adcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "ADC group regular conversion data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "ADC group regular conversion data"]
        #[inline(always)]
        pub const fn regular_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ADC group regular conversion data"]
        #[inline(always)]
        pub fn set_regular_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr0(pub u32);
    impl Hwcfgr0 {
        #[doc = "NUM_CHAN_24"]
        #[inline(always)]
        pub const fn num_chan_24(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NUM_CHAN_24"]
        #[inline(always)]
        pub fn set_num_chan_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Extra analog watchdog"]
        #[inline(always)]
        pub const fn extra_awds(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Extra analog watchdog"]
        #[inline(always)]
        pub fn set_extra_awds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Oversampling"]
        #[inline(always)]
        pub const fn ovs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Oversampling"]
        #[inline(always)]
        pub fn set_ovs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Hwcfgr0 {
        #[inline(always)]
        fn default() -> Hwcfgr0 {
            Hwcfgr0(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr1(pub u32);
    impl Hwcfgr1 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr1 {
        #[inline(always)]
        fn default() -> Hwcfgr1 {
            Hwcfgr1(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr2(pub u32);
    impl Hwcfgr2 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr2 {
        #[inline(always)]
        fn default() -> Hwcfgr2 {
            Hwcfgr2(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr3(pub u32);
    impl Hwcfgr3 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap11(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap9(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr3 {
        #[inline(always)]
        fn default() -> Hwcfgr3 {
            Hwcfgr3(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr4(pub u32);
    impl Hwcfgr4 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap15(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap13(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr4 {
        #[inline(always)]
        fn default() -> Hwcfgr4 {
            Hwcfgr4(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr5(pub u32);
    impl Hwcfgr5 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap19(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap18(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap17(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap16(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr5 {
        #[inline(always)]
        fn default() -> Hwcfgr5 {
            Hwcfgr5(0)
        }
    }
    #[doc = "Hardware Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr6(pub u32);
    impl Hwcfgr6 {
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap20(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap21(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap22(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub const fn chmap23(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel mapping"]
        #[inline(always)]
        pub fn set_chmap23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Hwcfgr6 {
        #[inline(always)]
        fn default() -> Hwcfgr6 {
            Hwcfgr6(0)
        }
    }
    #[doc = "ADC interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADC ready interrupt"]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready interrupt"]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC group regular end of sampling interrupt"]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sampling interrupt"]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of unitary conversion interrupt"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sequence conversions interrupt"]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC group regular overrun interrupt"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun interrupt"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC analog watchdog 1 interrupt"]
        #[inline(always)]
        pub const fn awd1ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 interrupt"]
        #[inline(always)]
        pub fn set_awd1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC analog watchdog 2 interrupt"]
        #[inline(always)]
        pub const fn awd2ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 2 interrupt"]
        #[inline(always)]
        pub fn set_awd2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC analog watchdog 3 interrupt"]
        #[inline(always)]
        pub const fn awd3ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 3 interrupt"]
        #[inline(always)]
        pub fn set_awd3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "End of calibration interrupt enable"]
        #[inline(always)]
        pub const fn eocalie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End of calibration interrupt enable"]
        #[inline(always)]
        pub fn set_eocalie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel Configuration Ready Interrupt enable"]
        #[inline(always)]
        pub const fn ccrdyie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Configuration Ready Interrupt enable"]
        #[inline(always)]
        pub fn set_ccrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "EXTI Identification register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipidr(pub u32);
    impl Ipidr {
        #[doc = "IP Identification"]
        #[inline(always)]
        pub const fn ipid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP Identification"]
        #[inline(always)]
        pub fn set_ipid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipidr {
        #[inline(always)]
        fn default() -> Ipidr {
            Ipidr(0)
        }
    }
    #[doc = "ADC interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADC ready flag"]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready flag"]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC group regular end of sampling flag"]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sampling flag"]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC group regular end of unitary conversion flag"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of unitary conversion flag"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC group regular end of sequence conversions flag"]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular end of sequence conversions flag"]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC group regular overrun flag"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC group regular overrun flag"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC analog watchdog 1 flag"]
        #[inline(always)]
        pub const fn awd1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 1 flag"]
        #[inline(always)]
        pub fn set_awd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC analog watchdog 2 flag"]
        #[inline(always)]
        pub const fn awd2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 2 flag"]
        #[inline(always)]
        pub fn set_awd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC analog watchdog 3 flag"]
        #[inline(always)]
        pub const fn awd3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC analog watchdog 3 flag"]
        #[inline(always)]
        pub fn set_awd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "End Of Calibration flag"]
        #[inline(always)]
        pub const fn eocal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "End Of Calibration flag"]
        #[inline(always)]
        pub fn set_eocal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel Configuration Ready flag"]
        #[inline(always)]
        pub const fn ccrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Configuration Ready flag"]
        #[inline(always)]
        pub fn set_ccrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "EXTI Size ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidr(pub u32);
    impl Sidr {
        #[doc = "Size Identification"]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Size Identification"]
        #[inline(always)]
        pub fn set_sid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sidr {
        #[inline(always)]
        fn default() -> Sidr {
            Sidr(0)
        }
    }
    #[doc = "ADC sampling time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub const fn smp1(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub fn set_smp1(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub const fn smp2(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Sampling time selection"]
        #[inline(always)]
        pub fn set_smp2(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub const fn smpsel(&self, n: usize) -> bool {
            assert!(n < 19usize);
            let offs = 8usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel sampling time selection"]
        #[inline(always)]
        pub fn set_smpsel(&mut self, n: usize, val: bool) {
            assert!(n < 19usize);
            let offs = 8usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Smpr {
        #[inline(always)]
        fn default() -> Smpr {
            Smpr(0)
        }
    }
    #[doc = "EXTI IP Version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Verr(pub u32);
    impl Verr {
        #[doc = "Minor Revision number"]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Minor Revision number"]
        #[inline(always)]
        pub fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Major Revision number"]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Major Revision number"]
        #[inline(always)]
        pub fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Verr {
        #[inline(always)]
        fn default() -> Verr {
            Verr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Res {
        #[doc = "12-bit resolution"]
        TWELVEBIT = 0,
        #[doc = "10-bit resolution"]
        TENBIT = 0x01,
        #[doc = "8-bit resolution"]
        EIGHTBIT = 0x02,
        #[doc = "6-bit resolution"]
        SIXBIT = 0x03,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Res {
        #[inline(always)]
        fn from(val: u8) -> Res {
            Res::from_bits(val)
        }
    }
    impl From<Res> for u8 {
        #[inline(always)]
        fn from(val: Res) -> u8 {
            Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SampleTime {
        #[doc = "1.5 ADC cycles"]
        CYCLES1_5 = 0,
        #[doc = "3.5 ADC cycles"]
        CYCLES3_5 = 0x01,
        #[doc = "7.5 ADC cycles"]
        CYCLES7_5 = 0x02,
        #[doc = "12.5 ADC cycles"]
        CYCLES12_5 = 0x03,
        #[doc = "19.5 ADC cycles"]
        CYCLES19_5 = 0x04,
        #[doc = "39.5 ADC cycles"]
        CYCLES39_5 = 0x05,
        #[doc = "79.5 ADC cycles"]
        CYCLES79_5 = 0x06,
        #[doc = "160.5 ADC cycles"]
        CYCLES160_5 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
}
