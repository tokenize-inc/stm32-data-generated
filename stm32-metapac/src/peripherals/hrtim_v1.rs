#[doc = "High Resolution Timer: Master Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrtim {
    ptr: *mut u8,
}
unsafe impl Send for Hrtim {}
unsafe impl Sync for Hrtim {}
impl Hrtim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Timer Control Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[inline(always)]
    pub const fn micr(self) -> crate::common::Reg<regs::Micr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mdier(self) -> crate::common::Reg<regs::Mdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Master Timer Counter Register"]
    #[inline(always)]
    pub const fn mcntr(self) -> crate::common::Reg<regs::Mcntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Master Timer Period Register"]
    #[inline(always)]
    pub const fn mper(self) -> crate::common::Reg<regs::Mper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Master Timer Repetition Register"]
    #[inline(always)]
    pub const fn mrep(self) -> crate::common::Reg<regs::Mrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Master Timer Compare X Register"]
    #[inline(always)]
    pub const fn mcmp(self, n: usize) -> crate::common::Reg<regs::Mcmpx, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.add(28usize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _)
        }
    }
    #[doc = "High Resolution Timer: Timing Unit"]
    #[inline(always)]
    pub const fn tim(self, n: usize) -> HrtimTimx {
        assert!(n < 5usize);
        unsafe { HrtimTimx::from_ptr(self.ptr.add(128usize + n * 128usize) as _) }
    }
}
#[doc = "High Resolution Timer: Timing Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrtimTimx {
    ptr: *mut u8,
}
unsafe impl Send for HrtimTimx {}
unsafe impl Sync for HrtimTimx {}
impl HrtimTimx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer X Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Timxcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Timer X Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Timxisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Timer X Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Timxicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Timer X DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Timxdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Timer X Counter Register"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Timxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Timer X Period Register"]
    #[inline(always)]
    pub const fn per(self) -> crate::common::Reg<regs::Timxper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Timer X Repetition Register"]
    #[inline(always)]
    pub const fn rep(self) -> crate::common::Reg<regs::Timxrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Timer X Compare X Register"]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Timxcmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.add(28usize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _)
        }
    }
    #[doc = "Timer X Compare X Compound Register"]
    #[inline(always)]
    pub const fn cmpc(self, n: usize) -> crate::common::Reg<regs::Timxcmpc, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + ([0usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Capture X Register"]
    #[inline(always)]
    pub const fn cpt(self, n: usize) -> crate::common::Reg<regs::Timxcpt, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize + n * 4usize) as _) }
    }
    #[doc = "Timer X Deadtime Register"]
    #[inline(always)]
    pub const fn dt(self) -> crate::common::Reg<regs::Timxdt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Timer X Output X Set Register"]
    #[inline(always)]
    pub const fn setr(self, n: usize) -> crate::common::Reg<regs::Timxsetr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output X Reset Register"]
    #[inline(always)]
    pub const fn rstr(self, n: usize) -> crate::common::Reg<regs::Timxrstr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eef(self, n: usize) -> crate::common::Reg<regs::Timxeef, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Reset Register"]
    #[inline(always)]
    pub const fn rst(self) -> crate::common::Reg<regs::Timxrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Timer X Chopper Register"]
    #[inline(always)]
    pub const fn chp(self) -> crate::common::Reg<regs::Timxchp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Timer X Capture X Control Register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Timxccr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output Register"]
    #[inline(always)]
    pub const fn outr(self) -> crate::common::Reg<regs::Timxoutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Timer X Fault Register"]
    #[inline(always)]
    pub const fn flt(self) -> crate::common::Reg<regs::Timxflt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
}
pub mod regs {
    #[doc = "Master Timer Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcmpx(pub u32);
    impl Mcmpx {
        #[doc = "Master Timer Compare X value"]
        #[inline(always)]
        pub const fn mcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master Timer Compare X value"]
        #[inline(always)]
        pub fn set_mcmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mcmpx {
        #[inline(always)]
        fn default() -> Mcmpx {
            Mcmpx(0)
        }
    }
    #[doc = "Master Timer Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcntr(pub u32);
    impl Mcntr {
        #[doc = "Counter value"]
        #[inline(always)]
        pub const fn mcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter value"]
        #[inline(always)]
        pub fn set_mcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mcntr {
        #[inline(always)]
        fn default() -> Mcntr {
            Mcntr(0)
        }
    }
    #[doc = "Master Timer Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "HRTIM Master Clock prescaler"]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HRTIM Master Clock prescaler"]
        #[inline(always)]
        pub fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Master Continuous mode"]
        #[inline(always)]
        pub const fn cont(&self) -> super::vals::Cont {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Cont::from_bits(val as u8)
        }
        #[doc = "Master Continuous mode"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: super::vals::Cont) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master Re-triggerable mode"]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master Re-triggerable mode"]
        #[inline(always)]
        pub fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Synchronization input"]
        #[inline(always)]
        pub const fn syncin(&self) -> super::vals::Syncin {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Syncin::from_bits(val as u8)
        }
        #[doc = "Synchronization input"]
        #[inline(always)]
        pub fn set_syncin(&mut self, val: super::vals::Syncin) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Synchronization Resets Master"]
        #[inline(always)]
        pub const fn syncrstm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Resets Master"]
        #[inline(always)]
        pub fn set_syncrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization Starts Master"]
        #[inline(always)]
        pub const fn syncstrtm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Starts Master"]
        #[inline(always)]
        pub fn set_syncstrtm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Synchronization output"]
        #[inline(always)]
        pub const fn syncout(&self) -> super::vals::Syncout {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Syncout::from_bits(val as u8)
        }
        #[doc = "Synchronization output"]
        #[inline(always)]
        pub fn set_syncout(&mut self, val: super::vals::Syncout) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Synchronization source"]
        #[inline(always)]
        pub const fn syncsrc(&self) -> super::vals::Syncsrc {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Syncsrc::from_bits(val as u8)
        }
        #[doc = "Synchronization source"]
        #[inline(always)]
        pub fn set_syncsrc(&mut self, val: super::vals::Syncsrc) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Master Counter enable"]
        #[inline(always)]
        pub const fn mcen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master Counter enable"]
        #[inline(always)]
        pub fn set_mcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer X counter enable"]
        #[inline(always)]
        pub const fn tcen(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X counter enable"]
        #[inline(always)]
        pub fn set_tcen(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Master Timer Repetition update"]
        #[inline(always)]
        pub const fn mrepu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer Repetition update"]
        #[inline(always)]
        pub fn set_mrepu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Burst DMA Update"]
        #[inline(always)]
        pub const fn brstdma(&self) -> super::vals::Brstdma {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Brstdma::from_bits(val as u8)
        }
        #[doc = "Burst DMA Update"]
        #[inline(always)]
        pub fn set_brstdma(&mut self, val: super::vals::Brstdma) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdier(pub u32);
    impl Mdier {
        #[doc = "Master Compare X Interrupt Enable"]
        #[inline(always)]
        pub const fn mcmpie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X Interrupt Enable"]
        #[inline(always)]
        pub fn set_mcmpie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition Interrupt Enable"]
        #[inline(always)]
        pub const fn mrepie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master Repetition Interrupt Enable"]
        #[inline(always)]
        pub fn set_mrepie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt Enable"]
        #[inline(always)]
        pub const fn syncie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input Interrupt Enable"]
        #[inline(always)]
        pub fn set_syncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master Update Interrupt Enable"]
        #[inline(always)]
        pub const fn mupdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update Interrupt Enable"]
        #[inline(always)]
        pub fn set_mupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master Compare X DMA request Enable"]
        #[inline(always)]
        pub const fn mcmpde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X DMA request Enable"]
        #[inline(always)]
        pub fn set_mcmpde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition DMA request Enable"]
        #[inline(always)]
        pub const fn mrepde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Master Repetition DMA request Enable"]
        #[inline(always)]
        pub fn set_mrepde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Sync Input DMA request Enable"]
        #[inline(always)]
        pub const fn syncde(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input DMA request Enable"]
        #[inline(always)]
        pub fn set_syncde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Master Update DMA request Enable"]
        #[inline(always)]
        pub const fn mupdde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update DMA request Enable"]
        #[inline(always)]
        pub fn set_mupdde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Mdier {
        #[inline(always)]
        fn default() -> Mdier {
            Mdier(0)
        }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Micr(pub u32);
    impl Micr {
        #[doc = "Master Compare X Interrupt flag clear"]
        #[inline(always)]
        pub const fn mcmpc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mcmpc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt flag clear"]
        #[inline(always)]
        pub const fn mrepc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mrepc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt flag clear"]
        #[inline(always)]
        pub const fn syncc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input Interrupt flag clear"]
        #[inline(always)]
        pub fn set_syncc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master update Interrupt flag clear"]
        #[inline(always)]
        pub const fn mupdc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master update Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mupdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Micr {
        #[inline(always)]
        fn default() -> Micr {
            Micr(0)
        }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Master Compare X Interrupt Flag"]
        #[inline(always)]
        pub const fn mcmp(&self, n: usize) -> super::vals::Event {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Master Compare X Interrupt Flag"]
        #[inline(always)]
        pub fn set_mcmp(&mut self, n: usize, val: super::vals::Event) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition Interrupt Flag"]
        #[inline(always)]
        pub const fn mrep(&self) -> super::vals::Event {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Master Repetition Interrupt Flag"]
        #[inline(always)]
        pub fn set_mrep(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt Flag"]
        #[inline(always)]
        pub const fn sync(&self) -> super::vals::Event {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Sync Input Interrupt Flag"]
        #[inline(always)]
        pub fn set_sync(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Master Update Interrupt Flag"]
        #[inline(always)]
        pub const fn mupd(&self) -> super::vals::Event {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Master Update Interrupt Flag"]
        #[inline(always)]
        pub fn set_mupd(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[doc = "Master Timer Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mper(pub u32);
    impl Mper {
        #[doc = "Master Timer Period value"]
        #[inline(always)]
        pub const fn mper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master Timer Period value"]
        #[inline(always)]
        pub fn set_mper(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mper {
        #[inline(always)]
        fn default() -> Mper {
            Mper(0)
        }
    }
    #[doc = "Master Timer Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrep(pub u32);
    impl Mrep {
        #[doc = "Master Timer Repetition counter value"]
        #[inline(always)]
        pub const fn mrep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Master Timer Repetition counter value"]
        #[inline(always)]
        pub fn set_mrep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Mrep {
        #[inline(always)]
        fn default() -> Mrep {
            Mrep(0)
        }
    }
    #[doc = "Timerx Capture 2 Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxccr(pub u32);
    impl Timxccr {
        #[doc = "Software Capture"]
        #[inline(always)]
        pub const fn swcpt(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Software Capture"]
        #[inline(always)]
        pub fn set_swcpt(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Update Capture"]
        #[inline(always)]
        pub const fn updcpt(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Update Capture"]
        #[inline(always)]
        pub fn set_updcpt(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "External Event X Capture"]
        #[inline(always)]
        pub const fn exevcpt(&self, n: usize) -> super::vals::Captureeffect {
            assert!(n < 10usize);
            let offs = 2usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "External Event X Capture"]
        #[inline(always)]
        pub fn set_exevcpt(&mut self, n: usize, val: super::vals::Captureeffect) {
            assert!(n < 10usize);
            let offs = 2usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer X output Set"]
        #[inline(always)]
        pub const fn txset(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer X output Set"]
        #[inline(always)]
        pub fn set_txset(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer X output Reset"]
        #[inline(always)]
        pub const fn txrst(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer X output Reset"]
        #[inline(always)]
        pub fn set_txrst(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer X Compare X"]
        #[inline(always)]
        pub const fn txcmp(&self, n: usize) -> super::vals::Captureeffect {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer X Compare X"]
        #[inline(always)]
        pub fn set_txcmp(&mut self, n: usize, val: super::vals::Captureeffect) {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Y output Set"]
        #[inline(always)]
        pub const fn tyset(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Y output Set"]
        #[inline(always)]
        pub fn set_tyset(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer Y output Reset"]
        #[inline(always)]
        pub const fn tyrst(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Y output Reset"]
        #[inline(always)]
        pub fn set_tyrst(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer Y Compare X"]
        #[inline(always)]
        pub const fn tycmp(&self, n: usize) -> super::vals::Captureeffect {
            assert!(n < 2usize);
            let offs = 22usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Y Compare X"]
        #[inline(always)]
        pub fn set_tycmp(&mut self, n: usize, val: super::vals::Captureeffect) {
            assert!(n < 2usize);
            let offs = 22usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Z output Set"]
        #[inline(always)]
        pub const fn tzset(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Z output Set"]
        #[inline(always)]
        pub fn set_tzset(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer Z output Reset"]
        #[inline(always)]
        pub const fn tzrst(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Z output Reset"]
        #[inline(always)]
        pub fn set_tzrst(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Timer Z Compare X"]
        #[inline(always)]
        pub const fn tzcmp(&self, n: usize) -> super::vals::Captureeffect {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer Z Compare X"]
        #[inline(always)]
        pub fn set_tzcmp(&mut self, n: usize, val: super::vals::Captureeffect) {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer T output Set"]
        #[inline(always)]
        pub const fn ttset(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer T output Set"]
        #[inline(always)]
        pub fn set_ttset(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Timer T output Reset"]
        #[inline(always)]
        pub const fn ttrst(&self) -> super::vals::Captureeffect {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer T output Reset"]
        #[inline(always)]
        pub fn set_ttrst(&mut self, val: super::vals::Captureeffect) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Timer T Compare X"]
        #[inline(always)]
        pub const fn ttcmp(&self, n: usize) -> super::vals::Captureeffect {
            assert!(n < 2usize);
            let offs = 30usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Captureeffect::from_bits(val as u8)
        }
        #[doc = "Timer T Compare X"]
        #[inline(always)]
        pub fn set_ttcmp(&mut self, n: usize, val: super::vals::Captureeffect) {
            assert!(n < 2usize);
            let offs = 30usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Timxccr {
        #[inline(always)]
        fn default() -> Timxccr {
            Timxccr(0)
        }
    }
    #[doc = "Timerx Chopper Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxchp(pub u32);
    impl Timxchp {
        #[doc = "Timerx carrier frequency value"]
        #[inline(always)]
        pub const fn carfrq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Timerx carrier frequency value"]
        #[inline(always)]
        pub fn set_carfrq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Timerx chopper duty cycle value"]
        #[inline(always)]
        pub const fn cardty(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Timerx chopper duty cycle value"]
        #[inline(always)]
        pub fn set_cardty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Timerx start pulsewidth"]
        #[inline(always)]
        pub const fn strtpw(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "Timerx start pulsewidth"]
        #[inline(always)]
        pub fn set_strtpw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for Timxchp {
        #[inline(always)]
        fn default() -> Timxchp {
            Timxchp(0)
        }
    }
    #[doc = "Timerx Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmp(pub u32);
    impl Timxcmp {
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxcmp {
        #[inline(always)]
        fn default() -> Timxcmp {
            Timxcmp(0)
        }
    }
    #[doc = "Timerx Compare X Compound Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmpc(pub u32);
    impl Timxcmpc {
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Timerx Repetition value (aliased from HRTIM_REPx register)"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Timerx Repetition value (aliased from HRTIM_REPx register)"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Timxcmpc {
        #[inline(always)]
        fn default() -> Timxcmpc {
            Timxcmpc(0)
        }
    }
    #[doc = "Timerx Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcnt(pub u32);
    impl Timxcnt {
        #[doc = "Timerx Counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxcnt {
        #[inline(always)]
        fn default() -> Timxcnt {
            Timxcnt(0)
        }
    }
    #[doc = "Timerx Capture X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcpt(pub u32);
    impl Timxcpt {
        #[doc = "Timerx Capture X value"]
        #[inline(always)]
        pub const fn cpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Capture X value"]
        #[inline(always)]
        pub fn set_cpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxcpt {
        #[inline(always)]
        fn default() -> Timxcpt {
            Timxcpt(0)
        }
    }
    #[doc = "Timerx Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcr(pub u32);
    impl Timxcr {
        #[doc = "HRTIM Timer x Clock prescaler"]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HRTIM Timer x Clock prescaler"]
        #[inline(always)]
        pub fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Continuous mode"]
        #[inline(always)]
        pub const fn cont(&self) -> super::vals::Cont {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Cont::from_bits(val as u8)
        }
        #[doc = "Continuous mode"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: super::vals::Cont) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Re-triggerable mode"]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Re-triggerable mode"]
        #[inline(always)]
        pub fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Push-Pull mode enable"]
        #[inline(always)]
        pub const fn pshpll(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Push-Pull mode enable"]
        #[inline(always)]
        pub fn set_pshpll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Synchronization Resets Timer X"]
        #[inline(always)]
        pub const fn syncrst(&self) -> super::vals::Syncrst {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Syncrst::from_bits(val as u8)
        }
        #[doc = "Synchronization Resets Timer X"]
        #[inline(always)]
        pub fn set_syncrst(&mut self, val: super::vals::Syncrst) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization Starts Timer X"]
        #[inline(always)]
        pub const fn syncstrt(&self) -> super::vals::Syncstrt {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Syncstrt::from_bits(val as u8)
        }
        #[doc = "Synchronization Starts Timer X"]
        #[inline(always)]
        pub fn set_syncstrt(&mut self, val: super::vals::Syncstrt) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Delayed CMP2 mode"]
        #[inline(always)]
        pub const fn delcmp2(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "Delayed CMP2 mode"]
        #[inline(always)]
        pub fn set_delcmp2(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Delayed CMP4 mode"]
        #[inline(always)]
        pub const fn delcmp4(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "Delayed CMP4 mode"]
        #[inline(always)]
        pub fn set_delcmp4(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Timer X Repetition update"]
        #[inline(always)]
        pub const fn repu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X Repetition update"]
        #[inline(always)]
        pub fn set_repu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer X reset update"]
        #[inline(always)]
        pub const fn rstu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X reset update"]
        #[inline(always)]
        pub fn set_rstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer A update"]
        #[inline(always)]
        pub const fn tau(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A update"]
        #[inline(always)]
        pub fn set_tau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer B update"]
        #[inline(always)]
        pub const fn tbu(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B update"]
        #[inline(always)]
        pub fn set_tbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer C update"]
        #[inline(always)]
        pub const fn tcu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C update"]
        #[inline(always)]
        pub fn set_tcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer D update"]
        #[inline(always)]
        pub const fn tdu(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D update"]
        #[inline(always)]
        pub fn set_tdu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Timer E update"]
        #[inline(always)]
        pub const fn teu(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E update"]
        #[inline(always)]
        pub fn set_teu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Master Timer update"]
        #[inline(always)]
        pub const fn mstu(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer update"]
        #[inline(always)]
        pub fn set_mstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Update Gating"]
        #[inline(always)]
        pub const fn updgat(&self) -> super::vals::Updgat {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Updgat::from_bits(val as u8)
        }
        #[doc = "Update Gating"]
        #[inline(always)]
        pub fn set_updgat(&mut self, val: super::vals::Updgat) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Timxcr {
        #[inline(always)]
        fn default() -> Timxcr {
            Timxcr(0)
        }
    }
    #[doc = "Timerx DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdier(pub u32);
    impl Timxdier {
        #[doc = "Compare X Interrupt Enable"]
        #[inline(always)]
        pub const fn cmpie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X Interrupt Enable"]
        #[inline(always)]
        pub fn set_cmpie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt Enable"]
        #[inline(always)]
        pub const fn repie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt Enable"]
        #[inline(always)]
        pub fn set_repie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt Enable"]
        #[inline(always)]
        pub const fn updie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update Interrupt Enable"]
        #[inline(always)]
        pub fn set_updie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture Interrupt Enable"]
        #[inline(always)]
        pub const fn cptie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture Interrupt Enable"]
        #[inline(always)]
        pub fn set_cptie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set Interrupt Enable"]
        #[inline(always)]
        pub const fn setrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set Interrupt Enable"]
        #[inline(always)]
        pub fn set_setrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset Interrupt Enable"]
        #[inline(always)]
        pub const fn rstrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset Interrupt Enable"]
        #[inline(always)]
        pub fn set_rstrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset/roll-over Interrupt Enable"]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over Interrupt Enable"]
        #[inline(always)]
        pub fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Interrupt Enable"]
        #[inline(always)]
        pub const fn dlyprtie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Interrupt Enable"]
        #[inline(always)]
        pub fn set_dlyprtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Compare X DMA request Enable"]
        #[inline(always)]
        pub const fn cmpde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X DMA request Enable"]
        #[inline(always)]
        pub fn set_cmpde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition DMA request Enable"]
        #[inline(always)]
        pub const fn repde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition DMA request Enable"]
        #[inline(always)]
        pub fn set_repde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Update DMA request Enable"]
        #[inline(always)]
        pub const fn updde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request Enable"]
        #[inline(always)]
        pub fn set_updde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Capture X DMA request Enable"]
        #[inline(always)]
        pub const fn cptde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 23usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture X DMA request Enable"]
        #[inline(always)]
        pub fn set_cptde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 23usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set DMA request Enable"]
        #[inline(always)]
        pub const fn setrde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 25usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set DMA request Enable"]
        #[inline(always)]
        pub fn set_setrde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 25usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset DMA request Enable"]
        #[inline(always)]
        pub const fn rstrde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 26usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset DMA request Enable"]
        #[inline(always)]
        pub fn set_rstrde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 26usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset/roll-over DMA request Enable"]
        #[inline(always)]
        pub const fn rstde(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over DMA request Enable"]
        #[inline(always)]
        pub fn set_rstde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Delayed Protection DMA request Enable"]
        #[inline(always)]
        pub const fn dlyprtde(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection DMA request Enable"]
        #[inline(always)]
        pub fn set_dlyprtde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Timxdier {
        #[inline(always)]
        fn default() -> Timxdier {
            Timxdier(0)
        }
    }
    #[doc = "Timerx Deadtime Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdt(pub u32);
    impl Timxdt {
        #[doc = "Deadtime Rising value"]
        #[inline(always)]
        pub const fn dtr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime Rising value"]
        #[inline(always)]
        pub fn set_dtr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Sign Deadtime Rising value"]
        #[inline(always)]
        pub const fn sdtr(&self) -> super::vals::Sdtr {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Sdtr::from_bits(val as u8)
        }
        #[doc = "Sign Deadtime Rising value"]
        #[inline(always)]
        pub fn set_sdtr(&mut self, val: super::vals::Sdtr) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Deadtime Prescaler"]
        #[inline(always)]
        pub const fn dtprsc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Deadtime Prescaler"]
        #[inline(always)]
        pub fn set_dtprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Deadtime Rising Sign Lock"]
        #[inline(always)]
        pub const fn dtrslk(&self) -> super::vals::Locked {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Locked::from_bits(val as u8)
        }
        #[doc = "Deadtime Rising Sign Lock"]
        #[inline(always)]
        pub fn set_dtrslk(&mut self, val: super::vals::Locked) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Deadtime Rising Lock"]
        #[inline(always)]
        pub const fn dtrlk(&self) -> super::vals::Locked {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Locked::from_bits(val as u8)
        }
        #[doc = "Deadtime Rising Lock"]
        #[inline(always)]
        pub fn set_dtrlk(&mut self, val: super::vals::Locked) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Deadtime Falling value"]
        #[inline(always)]
        pub const fn dtf(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime Falling value"]
        #[inline(always)]
        pub fn set_dtf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "Sign Deadtime Falling value"]
        #[inline(always)]
        pub const fn sdtf(&self) -> super::vals::Sdtf {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Sdtf::from_bits(val as u8)
        }
        #[doc = "Sign Deadtime Falling value"]
        #[inline(always)]
        pub fn set_sdtf(&mut self, val: super::vals::Sdtf) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Deadtime Falling Sign Lock"]
        #[inline(always)]
        pub const fn dtfslk(&self) -> super::vals::Locked {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Locked::from_bits(val as u8)
        }
        #[doc = "Deadtime Falling Sign Lock"]
        #[inline(always)]
        pub fn set_dtfslk(&mut self, val: super::vals::Locked) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Deadtime Falling Lock"]
        #[inline(always)]
        pub const fn dtflk(&self) -> super::vals::Locked {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Locked::from_bits(val as u8)
        }
        #[doc = "Deadtime Falling Lock"]
        #[inline(always)]
        pub fn set_dtflk(&mut self, val: super::vals::Locked) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxdt {
        #[inline(always)]
        fn default() -> Timxdt {
            Timxdt(0)
        }
    }
    #[doc = "Timer X External Event Filtering Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxeef(pub u32);
    impl Timxeef {
        #[doc = "External Event X latch"]
        #[inline(always)]
        pub const fn ltch(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X latch"]
        #[inline(always)]
        pub fn set_ltch(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub const fn fltr(&self, n: usize) -> super::vals::Eefltr {
            assert!(n < 5usize);
            let offs = 1usize + n * 6usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::Eefltr::from_bits(val as u8)
        }
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub fn set_fltr(&mut self, n: usize, val: super::vals::Eefltr) {
            assert!(n < 5usize);
            let offs = 1usize + n * 6usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for Timxeef {
        #[inline(always)]
        fn default() -> Timxeef {
            Timxeef(0)
        }
    }
    #[doc = "Timerx Fault Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxflt(pub u32);
    impl Timxflt {
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub const fn flten(&self, n: usize) -> super::vals::Flten {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Flten::from_bits(val as u8)
        }
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub fn set_flten(&mut self, n: usize, val: super::vals::Flten) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Fault sources Lock"]
        #[inline(always)]
        pub const fn fltlck(&self) -> super::vals::Locked {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Locked::from_bits(val as u8)
        }
        #[doc = "Fault sources Lock"]
        #[inline(always)]
        pub fn set_fltlck(&mut self, val: super::vals::Locked) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxflt {
        #[inline(always)]
        fn default() -> Timxflt {
            Timxflt(0)
        }
    }
    #[doc = "Timerx Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxicr(pub u32);
    impl Timxicr {
        #[doc = "Compare X Interrupt flag Clear"]
        #[inline(always)]
        pub const fn cmpc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_cmpc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt flag Clear"]
        #[inline(always)]
        pub const fn repc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_repc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt flag Clear"]
        #[inline(always)]
        pub const fn updc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_updc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture X Interrupt flag Clear"]
        #[inline(always)]
        pub const fn cptc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture X Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_cptc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set flag Clear"]
        #[inline(always)]
        pub const fn setrc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set flag Clear"]
        #[inline(always)]
        pub fn set_setrc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset flag Clear"]
        #[inline(always)]
        pub const fn rstrc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset flag Clear"]
        #[inline(always)]
        pub fn set_rstrc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset Interrupt flag Clear"]
        #[inline(always)]
        pub const fn rstc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_rstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Flag Clear"]
        #[inline(always)]
        pub const fn dlyprtc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Flag Clear"]
        #[inline(always)]
        pub fn set_dlyprtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Timxicr {
        #[inline(always)]
        fn default() -> Timxicr {
            Timxicr(0)
        }
    }
    #[doc = "Timerx Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxisr(pub u32);
    impl Timxisr {
        #[doc = "Compare X Interrupt Flag"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> super::vals::Event {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Compare X Interrupt Flag"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: super::vals::Event) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt Flag"]
        #[inline(always)]
        pub const fn rep(&self) -> super::vals::Event {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Repetition Interrupt Flag"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt Flag"]
        #[inline(always)]
        pub const fn upd(&self) -> super::vals::Event {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Update Interrupt Flag"]
        #[inline(always)]
        pub fn set_upd(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture X Interrupt Flag"]
        #[inline(always)]
        pub const fn cpt(&self, n: usize) -> super::vals::Event {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Capture X Interrupt Flag"]
        #[inline(always)]
        pub fn set_cpt(&mut self, n: usize, val: super::vals::Event) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set Interrupt Flag"]
        #[inline(always)]
        pub const fn setr(&self, n: usize) -> super::vals::Event {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Output X Set Interrupt Flag"]
        #[inline(always)]
        pub fn set_setr(&mut self, n: usize, val: super::vals::Event) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset Interrupt Flag"]
        #[inline(always)]
        pub const fn rstr(&self, n: usize) -> super::vals::Event {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Output X Reset Interrupt Flag"]
        #[inline(always)]
        pub fn set_rstr(&mut self, n: usize, val: super::vals::Event) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Reset Interrupt Flag"]
        #[inline(always)]
        pub const fn rst(&self) -> super::vals::Event {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Event::from_bits(val as u8)
        }
        #[doc = "Reset Interrupt Flag"]
        #[inline(always)]
        pub fn set_rst(&mut self, val: super::vals::Event) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Flag"]
        #[inline(always)]
        pub const fn dlyprt(&self) -> super::vals::TimaisrDlyprt {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::TimaisrDlyprt::from_bits(val as u8)
        }
        #[doc = "Delayed Protection Flag"]
        #[inline(always)]
        pub fn set_dlyprt(&mut self, val: super::vals::TimaisrDlyprt) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Current Push Pull Status"]
        #[inline(always)]
        pub const fn cppstat(&self) -> super::vals::Cppstat {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Cppstat::from_bits(val as u8)
        }
        #[doc = "Current Push Pull Status"]
        #[inline(always)]
        pub fn set_cppstat(&mut self, val: super::vals::Cppstat) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Idle Push Pull Status"]
        #[inline(always)]
        pub const fn ippstat(&self) -> super::vals::Ippstat {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ippstat::from_bits(val as u8)
        }
        #[doc = "Idle Push Pull Status"]
        #[inline(always)]
        pub fn set_ippstat(&mut self, val: super::vals::Ippstat) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Output X State"]
        #[inline(always)]
        pub const fn ostat(&self, n: usize) -> super::vals::Outputstate {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Outputstate::from_bits(val as u8)
        }
        #[doc = "Output X State"]
        #[inline(always)]
        pub fn set_ostat(&mut self, n: usize, val: super::vals::Outputstate) {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Copy"]
        #[inline(always)]
        pub const fn ocpy(&self, n: usize) -> super::vals::Outputstate {
            assert!(n < 2usize);
            let offs = 20usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Outputstate::from_bits(val as u8)
        }
        #[doc = "Output X Copy"]
        #[inline(always)]
        pub fn set_ocpy(&mut self, n: usize, val: super::vals::Outputstate) {
            assert!(n < 2usize);
            let offs = 20usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Timxisr {
        #[inline(always)]
        fn default() -> Timxisr {
            Timxisr(0)
        }
    }
    #[doc = "Timerx Output Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxoutr(pub u32);
    impl Timxoutr {
        #[doc = "Output 1 polarity"]
        #[inline(always)]
        pub const fn pol(&self, n: usize) -> super::vals::Pol {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output 1 polarity"]
        #[inline(always)]
        pub fn set_pol(&mut self, n: usize, val: super::vals::Pol) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Idle mode"]
        #[inline(always)]
        pub const fn idlem(&self, n: usize) -> super::vals::Idlem {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Idlem::from_bits(val as u8)
        }
        #[doc = "Output X Idle mode"]
        #[inline(always)]
        pub fn set_idlem(&mut self, n: usize, val: super::vals::Idlem) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output 1 Idle State"]
        #[inline(always)]
        pub const fn idles(&self, n: usize) -> super::vals::Idles {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Idles::from_bits(val as u8)
        }
        #[doc = "Output 1 Idle State"]
        #[inline(always)]
        pub fn set_idles(&mut self, n: usize, val: super::vals::Idles) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Fault state"]
        #[inline(always)]
        pub const fn faultx(&self, n: usize) -> super::vals::Fault {
            assert!(n < 2usize);
            let offs = 4usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            super::vals::Fault::from_bits(val as u8)
        }
        #[doc = "Output X Fault state"]
        #[inline(always)]
        pub fn set_faultx(&mut self, n: usize, val: super::vals::Fault) {
            assert!(n < 2usize);
            let offs = 4usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output X Chopper enable"]
        #[inline(always)]
        pub const fn chp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Chopper enable"]
        #[inline(always)]
        pub fn set_chp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Deadtime upon burst mode Idle entry"]
        #[inline(always)]
        pub const fn didl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Deadtime upon burst mode Idle entry"]
        #[inline(always)]
        pub fn set_didl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Deadtime enable"]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime enable"]
        #[inline(always)]
        pub fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Delayed Protection Enable"]
        #[inline(always)]
        pub const fn dlyprten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Enable"]
        #[inline(always)]
        pub fn set_dlyprten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Delayed Protection"]
        #[inline(always)]
        pub const fn dlyprt(&self) -> super::vals::Dlyprt {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Dlyprt::from_bits(val as u8)
        }
        #[doc = "Delayed Protection"]
        #[inline(always)]
        pub fn set_dlyprt(&mut self, val: super::vals::Dlyprt) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
    }
    impl Default for Timxoutr {
        #[inline(always)]
        fn default() -> Timxoutr {
            Timxoutr(0)
        }
    }
    #[doc = "Timerx Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxper(pub u32);
    impl Timxper {
        #[doc = "Timerx Period value"]
        #[inline(always)]
        pub const fn per(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Period value"]
        #[inline(always)]
        pub fn set_per(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxper {
        #[inline(always)]
        fn default() -> Timxper {
            Timxper(0)
        }
    }
    #[doc = "Timerx Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrep(pub u32);
    impl Timxrep {
        #[doc = "Timerx Repetition counter value"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timerx Repetition counter value"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Timxrep {
        #[inline(always)]
        fn default() -> Timxrep {
            Timxrep(0)
        }
    }
    #[doc = "Timerx Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrst(pub u32);
    impl Timxrst {
        #[doc = "Timer X Update reset"]
        #[inline(always)]
        pub const fn updt(&self) -> super::vals::Reseteffect {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer X Update reset"]
        #[inline(always)]
        pub fn set_updt(&mut self, val: super::vals::Reseteffect) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X compare X reset"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 2usize);
            let offs = 2usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer X compare X reset"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 2usize);
            let offs = 2usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Master timer Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> super::vals::Reseteffect {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Master timer Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: super::vals::Reseteffect) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Master compare X"]
        #[inline(always)]
        pub const fn mstcmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 4usize);
            let offs = 5usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Master compare X"]
        #[inline(always)]
        pub fn set_mstcmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 4usize);
            let offs = 5usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 10usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 10usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer X Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub const fn timxcmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 3usize);
            let offs = 19usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer X Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub fn set_timxcmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 3usize);
            let offs = 19usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Y Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub const fn timycmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 3usize);
            let offs = 22usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer Y Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub fn set_timycmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 3usize);
            let offs = 22usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub const fn timzcmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 3usize);
            let offs = 25usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub fn set_timzcmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 3usize);
            let offs = 25usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub const fn timtcmp(&self, n: usize) -> super::vals::Reseteffect {
            assert!(n < 3usize);
            let offs = 28usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Reseteffect::from_bits(val as u8)
        }
        #[doc = "Timer Compare \\[1, 2, 4\\]"]
        #[inline(always)]
        pub fn set_timtcmp(&mut self, n: usize, val: super::vals::Reseteffect) {
            assert!(n < 3usize);
            let offs = 28usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Timxrst {
        #[inline(always)]
        fn default() -> Timxrst {
            Timxrst(0)
        }
    }
    #[doc = "Timerx OutputX Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrstr(pub u32);
    impl Timxrstr {
        #[doc = "Software Reset trigger"]
        #[inline(always)]
        pub const fn srt(&self) -> super::vals::Inactiveeffect {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Software Reset trigger"]
        #[inline(always)]
        pub fn set_srt(&mut self, val: super::vals::Inactiveeffect) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub const fn resync(&self) -> super::vals::Inactiveeffect {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub fn set_resync(&mut self, val: super::vals::Inactiveeffect) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub const fn per(&self) -> super::vals::Inactiveeffect {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub fn set_per(&mut self, val: super::vals::Inactiveeffect) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> super::vals::Inactiveeffect {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: super::vals::Inactiveeffect) {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> super::vals::Inactiveeffect {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: super::vals::Inactiveeffect) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub const fn mstcmp(&self, n: usize) -> super::vals::Inactiveeffect {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub fn set_mstcmp(&mut self, n: usize, val: super::vals::Inactiveeffect) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub const fn timevnt(&self, n: usize) -> super::vals::Inactiveeffect {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub fn set_timevnt(&mut self, n: usize, val: super::vals::Inactiveeffect) {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> super::vals::Inactiveeffect {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: super::vals::Inactiveeffect) {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub const fn update(&self) -> super::vals::Inactiveeffect {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Inactiveeffect::from_bits(val as u8)
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub fn set_update(&mut self, val: super::vals::Inactiveeffect) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxrstr {
        #[inline(always)]
        fn default() -> Timxrstr {
            Timxrstr(0)
        }
    }
    #[doc = "Timerx OutputX Set Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxsetr(pub u32);
    impl Timxsetr {
        #[doc = "Software Set trigger"]
        #[inline(always)]
        pub const fn sst(&self) -> super::vals::Activeeffect {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Software Set trigger"]
        #[inline(always)]
        pub fn set_sst(&mut self, val: super::vals::Activeeffect) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub const fn resync(&self) -> super::vals::Activeeffect {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub fn set_resync(&mut self, val: super::vals::Activeeffect) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub const fn per(&self) -> super::vals::Activeeffect {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub fn set_per(&mut self, val: super::vals::Activeeffect) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> super::vals::Activeeffect {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: super::vals::Activeeffect) {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> super::vals::Activeeffect {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: super::vals::Activeeffect) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub const fn mstcmpx(&self, n: usize) -> super::vals::Activeeffect {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub fn set_mstcmpx(&mut self, n: usize, val: super::vals::Activeeffect) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub const fn timevnt(&self, n: usize) -> super::vals::Activeeffect {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub fn set_timevnt(&mut self, n: usize, val: super::vals::Activeeffect) {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> super::vals::Activeeffect {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: super::vals::Activeeffect) {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub const fn update(&self) -> super::vals::Activeeffect {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Activeeffect::from_bits(val as u8)
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub fn set_update(&mut self, val: super::vals::Activeeffect) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxsetr {
        #[inline(always)]
        fn default() -> Timxsetr {
            Timxsetr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Activeeffect {
        #[doc = "Timer event has no effect"]
        NOEFFECT = 0,
        #[doc = "Timer event forces the output to its active state"]
        SETACTIVE = 0x01,
    }
    impl Activeeffect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Activeeffect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Activeeffect {
        #[inline(always)]
        fn from(val: u8) -> Activeeffect {
            Activeeffect::from_bits(val)
        }
    }
    impl From<Activeeffect> for u8 {
        #[inline(always)]
        fn from(val: Activeeffect) -> u8 {
            Activeeffect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Brstdma {
        #[doc = "Update done independently from the DMA burst transfer completion"]
        INDEPENDENT = 0,
        #[doc = "Update done when the DMA burst transfer is completed"]
        COMPLETION = 0x01,
        #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
        ROLLOVER = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Brstdma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Brstdma {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Brstdma {
        #[inline(always)]
        fn from(val: u8) -> Brstdma {
            Brstdma::from_bits(val)
        }
    }
    impl From<Brstdma> for u8 {
        #[inline(always)]
        fn from(val: Brstdma) -> u8 {
            Brstdma::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Captureeffect {
        #[doc = "Timer event has no effect"]
        NOEFFECT = 0,
        #[doc = "Timer event triggers capture"]
        TRIGGERCAPTURE = 0x01,
    }
    impl Captureeffect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Captureeffect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Captureeffect {
        #[inline(always)]
        fn from(val: u8) -> Captureeffect {
            Captureeffect::from_bits(val)
        }
    }
    impl From<Captureeffect> for u8 {
        #[inline(always)]
        fn from(val: Captureeffect) -> u8 {
            Captureeffect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cont {
        #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
        SINGLESHOT = 0,
        #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
        CONTINUOUS = 0x01,
    }
    impl Cont {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cont {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cont {
        #[inline(always)]
        fn from(val: u8) -> Cont {
            Cont::from_bits(val)
        }
    }
    impl From<Cont> for u8 {
        #[inline(always)]
        fn from(val: Cont) -> u8 {
            Cont::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cppstat {
        #[doc = "Signal applied on output 1 and output 2 forced inactive"]
        OUTPUT1ACTIVE = 0,
        #[doc = "Signal applied on output 2 and output 1 forced inactive"]
        OUTPUT2ACTIVE = 0x01,
    }
    impl Cppstat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cppstat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cppstat {
        #[inline(always)]
        fn from(val: u8) -> Cppstat {
            Cppstat::from_bits(val)
        }
    }
    impl From<Cppstat> for u8 {
        #[inline(always)]
        fn from(val: Cppstat) -> u8 {
            Cppstat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dacsync {
        #[doc = "No DAC trigger generated"]
        DISABLED = 0,
        #[doc = "Trigger generated on DACSync1"]
        DACSYNC1 = 0x01,
        #[doc = "Trigger generated on DACSync2"]
        DACSYNC2 = 0x02,
        #[doc = "Trigger generated on DACSync3"]
        DACSYNC3 = 0x03,
    }
    impl Dacsync {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacsync {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacsync {
        #[inline(always)]
        fn from(val: u8) -> Dacsync {
            Dacsync::from_bits(val)
        }
    }
    impl From<Dacsync> for u8 {
        #[inline(always)]
        fn from(val: Dacsync) -> u8 {
            Dacsync::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Delcmp {
        #[doc = "CMP register is always active (standard compare mode)"]
        STANDARD = 0,
        #[doc = "CMP is recomputed and is active following a capture 1 event"]
        CAPTURE1 = 0x01,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 1 match"]
        CAPTUREX_COMPARE1 = 0x02,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 3 match"]
        CAPTUREX_COMPARE3 = 0x03,
    }
    impl Delcmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Delcmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Delcmp {
        #[inline(always)]
        fn from(val: u8) -> Delcmp {
            Delcmp::from_bits(val)
        }
    }
    impl From<Delcmp> for u8 {
        #[inline(always)]
        fn from(val: Delcmp) -> u8 {
            Delcmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dlyprt {
        #[doc = "Output 1 delayed idle on external event 6"]
        OUTPUT1_EE6 = 0,
        #[doc = "Output 2 delayed idle on external event 6"]
        OUTPUT2_EE6 = 0x01,
        #[doc = "Output 1 and 2 delayed idle on external event 6"]
        OUTPUT1_2_EE6 = 0x02,
        #[doc = "Balanced idle on external event 6"]
        BALANCED_EE6 = 0x03,
        #[doc = "Output 1 delayed idle on external event 7"]
        OUTPUT1_EE7 = 0x04,
        #[doc = "Output 2 delayed idle on external event 7"]
        OUTPUT2_EE7 = 0x05,
        #[doc = "Output 1 and 2 delayed idle on external event 7"]
        OUTPUT1_2_EE7 = 0x06,
        #[doc = "Balanced idle on external event 7"]
        BALANCED_EE7 = 0x07,
    }
    impl Dlyprt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dlyprt {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dlyprt {
        #[inline(always)]
        fn from(val: u8) -> Dlyprt {
            Dlyprt::from_bits(val)
        }
    }
    impl From<Dlyprt> for u8 {
        #[inline(always)]
        fn from(val: Dlyprt) -> u8 {
            Dlyprt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eefltr {
        #[doc = "No filtering"]
        DISABLED = 0,
        #[doc = "Blanking from counter reset/roll-over to Compare 1"]
        BLANKRESETTOCOMPARE1 = 0x01,
        #[doc = "Blanking from counter reset/roll-over to Compare 2"]
        BLANKRESETTOCOMPARE2 = 0x02,
        #[doc = "Blanking from counter reset/roll-over to Compare 3"]
        BLANKRESETTOCOMPARE3 = 0x03,
        #[doc = "Blanking from counter reset/roll-over to Compare 4"]
        BLANKRESETTOCOMPARE4 = 0x04,
        #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
        BLANKTIMFLTR1 = 0x05,
        #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
        BLANKTIMFLTR2 = 0x06,
        #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
        BLANKTIMFLTR3 = 0x07,
        #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
        BLANKTIMFLTR4 = 0x08,
        #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
        BLANKTIMFLTR5 = 0x09,
        #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
        BLANKTIMFLTR6 = 0x0a,
        #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
        BLANKTIMFLTR7 = 0x0b,
        #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
        BLANKTIMFLTR8 = 0x0c,
        #[doc = "Windowing from counter reset/roll-over to compare 2"]
        WINDOWRESETTOCOMPARE2 = 0x0d,
        #[doc = "Windowing from counter reset/roll-over to compare 3"]
        WINDOWRESETTOCOMPARE3 = 0x0e,
        #[doc = "Windowing from another timing unit: TIMWIN source"]
        WINDOWTIMWIN = 0x0f,
    }
    impl Eefltr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eefltr {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eefltr {
        #[inline(always)]
        fn from(val: u8) -> Eefltr {
            Eefltr::from_bits(val)
        }
    }
    impl From<Eefltr> for u8 {
        #[inline(always)]
        fn from(val: Eefltr) -> u8 {
            Eefltr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Event {
        #[doc = "No compare interrupt occurred"]
        NOEVENT = 0,
        #[doc = "Compare interrupt occurred"]
        EVENT = 0x01,
    }
    impl Event {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Event {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Event {
        #[inline(always)]
        fn from(val: u8) -> Event {
            Event::from_bits(val)
        }
    }
    impl From<Event> for u8 {
        #[inline(always)]
        fn from(val: Event) -> u8 {
            Event::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fault {
        #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
        DISABLED = 0,
        #[doc = "Output goes to active state after a fault event"]
        SETACTIVE = 0x01,
        #[doc = "Output goes to inactive state after a fault event"]
        SETINACTIVE = 0x02,
        #[doc = "Output goes to high-z state after a fault event"]
        SETHIGHZ = 0x03,
    }
    impl Fault {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fault {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fault {
        #[inline(always)]
        fn from(val: u8) -> Fault {
            Fault::from_bits(val)
        }
    }
    impl From<Fault> for u8 {
        #[inline(always)]
        fn from(val: Fault) -> u8 {
            Fault::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Flten {
        #[doc = "Fault input ignored"]
        IGNORED = 0,
        #[doc = "Fault input is active and can disable HRTIM outputs"]
        ACTIVE = 0x01,
    }
    impl Flten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flten {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flten {
        #[inline(always)]
        fn from(val: u8) -> Flten {
            Flten::from_bits(val)
        }
    }
    impl From<Flten> for u8 {
        #[inline(always)]
        fn from(val: Flten) -> u8 {
            Flten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Idlem {
        #[doc = "No action: the output is not affected by the burst mode operation"]
        NOEFFECT = 0,
        #[doc = "The output is in idle state when requested by the burst mode controller"]
        SETIDLE = 0x01,
    }
    impl Idlem {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idlem {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idlem {
        #[inline(always)]
        fn from(val: u8) -> Idlem {
            Idlem::from_bits(val)
        }
    }
    impl From<Idlem> for u8 {
        #[inline(always)]
        fn from(val: Idlem) -> u8 {
            Idlem::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Idles {
        #[doc = "Output idle state is inactive"]
        INACTIVE = 0,
        #[doc = "Output idle state is active"]
        ACTIVE = 0x01,
    }
    impl Idles {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idles {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idles {
        #[inline(always)]
        fn from(val: u8) -> Idles {
            Idles::from_bits(val)
        }
    }
    impl From<Idles> for u8 {
        #[inline(always)]
        fn from(val: Idles) -> u8 {
            Idles::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Inactiveeffect {
        #[doc = "Timer event has no effect"]
        NOEFFECT = 0,
        #[doc = "Timer event forces the output to its inactive state"]
        SETINACTIVE = 0x01,
    }
    impl Inactiveeffect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inactiveeffect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inactiveeffect {
        #[inline(always)]
        fn from(val: u8) -> Inactiveeffect {
            Inactiveeffect::from_bits(val)
        }
    }
    impl From<Inactiveeffect> for u8 {
        #[inline(always)]
        fn from(val: Inactiveeffect) -> u8 {
            Inactiveeffect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ippstat {
        #[doc = "Protection occurred when the output 1 was active and output 2 forced inactive"]
        OUTPUT1ACTIVE = 0,
        #[doc = "Protection occurred when the output 2 was active and output 1 forced inactive"]
        OUTPUT2ACTIVE = 0x01,
    }
    impl Ippstat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ippstat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ippstat {
        #[inline(always)]
        fn from(val: u8) -> Ippstat {
            Ippstat::from_bits(val)
        }
    }
    impl From<Ippstat> for u8 {
        #[inline(always)]
        fn from(val: Ippstat) -> u8 {
            Ippstat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Locked {
        #[doc = "Bits are writeable"]
        UNLOCKED = 0,
        #[doc = "Bits are read-only"]
        LOCKED = 0x01,
    }
    impl Locked {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Locked {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Locked {
        #[inline(always)]
        fn from(val: u8) -> Locked {
            Locked::from_bits(val)
        }
    }
    impl From<Locked> for u8 {
        #[inline(always)]
        fn from(val: Locked) -> u8 {
            Locked::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Outputstate {
        #[doc = "Output is or was inactive"]
        INACTIVE = 0,
        #[doc = "Output is or was active"]
        ACTIVE = 0x01,
    }
    impl Outputstate {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Outputstate {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Outputstate {
        #[inline(always)]
        fn from(val: u8) -> Outputstate {
            Outputstate::from_bits(val)
        }
    }
    impl From<Outputstate> for u8 {
        #[inline(always)]
        fn from(val: Outputstate) -> u8 {
            Outputstate::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pol {
        #[doc = "Positive polarity (output active high)"]
        ACTIVEHIGH = 0,
        #[doc = "Negative polarity (output active low)"]
        ACTIVELOW = 0x01,
    }
    impl Pol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pol {
        #[inline(always)]
        fn from(val: u8) -> Pol {
            Pol::from_bits(val)
        }
    }
    impl From<Pol> for u8 {
        #[inline(always)]
        fn from(val: Pol) -> u8 {
            Pol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Reseteffect {
        #[doc = "Timer Y compare Z event has no effect"]
        NOEFFECT = 0,
        #[doc = "Timer X counter is reset upon timer Y compare Z event"]
        RESETCOUNTER = 0x01,
    }
    impl Reseteffect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reseteffect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reseteffect {
        #[inline(always)]
        fn from(val: u8) -> Reseteffect {
            Reseteffect::from_bits(val)
        }
    }
    impl From<Reseteffect> for u8 {
        #[inline(always)]
        fn from(val: Reseteffect) -> u8 {
            Reseteffect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdtf {
        #[doc = "Positive deadtime on falling edge"]
        POSITIVE = 0,
        #[doc = "Negative deadtime on falling edge"]
        NEGATIVE = 0x01,
    }
    impl Sdtf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdtf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdtf {
        #[inline(always)]
        fn from(val: u8) -> Sdtf {
            Sdtf::from_bits(val)
        }
    }
    impl From<Sdtf> for u8 {
        #[inline(always)]
        fn from(val: Sdtf) -> u8 {
            Sdtf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdtr {
        #[doc = "Positive deadtime on rising edge"]
        POSITIVE = 0,
        #[doc = "Negative deadtime on rising edge"]
        NEGATIVE = 0x01,
    }
    impl Sdtr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdtr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdtr {
        #[inline(always)]
        fn from(val: u8) -> Sdtr {
            Sdtr::from_bits(val)
        }
    }
    impl From<Sdtr> for u8 {
        #[inline(always)]
        fn from(val: Sdtr) -> u8 {
            Sdtr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Syncin {
        #[doc = "Disabled. HRTIM is not synchronized and runs in standalone mode"]
        DISABLED = 0,
        _RESERVED_1 = 0x01,
        #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
        INTERNAL = 0x02,
        #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
        EXTERNAL = 0x03,
    }
    impl Syncin {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncin {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncin {
        #[inline(always)]
        fn from(val: u8) -> Syncin {
            Syncin::from_bits(val)
        }
    }
    impl From<Syncin> for u8 {
        #[inline(always)]
        fn from(val: Syncin) -> u8 {
            Syncin::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Syncout {
        #[doc = "Disabled"]
        DISABLED = 0,
        _RESERVED_1 = 0x01,
        #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        POSITIVEPULSE = 0x02,
        #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        NEGATIVEPULSE = 0x03,
    }
    impl Syncout {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncout {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncout {
        #[inline(always)]
        fn from(val: u8) -> Syncout {
            Syncout::from_bits(val)
        }
    }
    impl From<Syncout> for u8 {
        #[inline(always)]
        fn from(val: Syncout) -> u8 {
            Syncout::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Syncrst {
        #[doc = "Synchronization event has no effect on Timer x"]
        DISABLED = 0,
        #[doc = "Synchronization event resets Timer x"]
        RESET = 0x01,
    }
    impl Syncrst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncrst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncrst {
        #[inline(always)]
        fn from(val: u8) -> Syncrst {
            Syncrst::from_bits(val)
        }
    }
    impl From<Syncrst> for u8 {
        #[inline(always)]
        fn from(val: Syncrst) -> u8 {
            Syncrst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Syncsrc {
        #[doc = "Master timer Start"]
        MASTERSTART = 0,
        #[doc = "Master timer Compare 1 event"]
        MASTERCOMPARE1 = 0x01,
        #[doc = "Timer A start/reset"]
        TIMERASTART = 0x02,
        #[doc = "Timer A Compare 1 event"]
        TIMERACOMPARE1 = 0x03,
    }
    impl Syncsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncsrc {
        #[inline(always)]
        fn from(val: u8) -> Syncsrc {
            Syncsrc::from_bits(val)
        }
    }
    impl From<Syncsrc> for u8 {
        #[inline(always)]
        fn from(val: Syncsrc) -> u8 {
            Syncsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Syncstrt {
        #[doc = "Synchronization event has no effect on Timer x"]
        DISABLED = 0,
        #[doc = "Synchronization event starts Timer x"]
        START = 0x01,
    }
    impl Syncstrt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncstrt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncstrt {
        #[inline(always)]
        fn from(val: u8) -> Syncstrt {
            Syncstrt::from_bits(val)
        }
    }
    impl From<Syncstrt> for u8 {
        #[inline(always)]
        fn from(val: Syncstrt) -> u8 {
            Syncstrt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TimaisrDlyprt {
        #[doc = "Not in delayed idle or balanced idle mode"]
        INACTIVE = 0,
        #[doc = "Delayed idle or balanced idle mode entry"]
        ACTIVE = 0x01,
    }
    impl TimaisrDlyprt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TimaisrDlyprt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TimaisrDlyprt {
        #[inline(always)]
        fn from(val: u8) -> TimaisrDlyprt {
            TimaisrDlyprt::from_bits(val)
        }
    }
    impl From<TimaisrDlyprt> for u8 {
        #[inline(always)]
        fn from(val: TimaisrDlyprt) -> u8 {
            TimaisrDlyprt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Updgat {
        #[doc = "Update occurs independently from the DMA burst transfer"]
        INDEPENDENT = 0,
        #[doc = "Update occurs when the DMA burst transfer is completed"]
        DMABURST = 0x01,
        #[doc = "Update occurs on the update event following DMA burst transfer completion"]
        DMABURST_UPDATE = 0x02,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
        INPUT1 = 0x03,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
        INPUT2 = 0x04,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
        INPUT3 = 0x05,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
        INPUT1_UPDATE = 0x06,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
        INPUT2_UPDATE = 0x07,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
        INPUT3_UPDATE = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Updgat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Updgat {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Updgat {
        #[inline(always)]
        fn from(val: u8) -> Updgat {
            Updgat::from_bits(val)
        }
    }
    impl From<Updgat> for u8 {
        #[inline(always)]
        fn from(val: Updgat) -> u8 {
            Updgat::to_bits(val)
        }
    }
}
