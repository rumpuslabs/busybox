#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Location Register"]
    pub ilr: ILR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Clock Control Register"]
    pub ccr: CCR,
    #[doc = "0x0c - Counter Increment Interrupt Register"]
    pub ciir: CIIR,
    #[doc = "0x10 - Alarm Mask Register"]
    pub amr: AMR,
    #[doc = "0x14 - Consolidated Time Register 0"]
    pub ctime0: CTIME0,
    #[doc = "0x18 - Consolidated Time Register 1"]
    pub ctime1: CTIME1,
    #[doc = "0x1c - Consolidated Time Register 2"]
    pub ctime2: CTIME2,
    #[doc = "0x20 - Seconds Counter"]
    pub sec: SEC,
    #[doc = "0x24 - Minutes Register"]
    pub min: MIN,
    #[doc = "0x28 - Hours Register"]
    pub hrs: HRS,
    #[doc = "0x2c - Day of Month Register"]
    pub dom: DOM,
    #[doc = "0x30 - Day of Week Register"]
    pub dow: DOW,
    #[doc = "0x34 - Day of Year Register"]
    pub doy: DOY,
    #[doc = "0x38 - Months Register"]
    pub month: MONTH,
    #[doc = "0x3c - Years Register"]
    pub year: YEAR,
    #[doc = "0x40 - Calibration Value Register"]
    pub calibration: CALIBRATION,
    #[doc = "0x44..0x58 - General Purpose Register 0"]
    pub gpreg: [GPREG; 5],
    #[doc = "0x58 - RTC Auxiliary Enable register"]
    pub rtc_auxen: RTC_AUXEN,
    #[doc = "0x5c - RTC Auxiliary control register"]
    pub rtc_aux: RTC_AUX,
    #[doc = "0x60 - Alarm value for Seconds"]
    pub asec: ASEC,
    #[doc = "0x64 - Alarm value for Minutes"]
    pub amin: AMIN,
    #[doc = "0x68 - Alarm value for Hours"]
    pub ahrs: AHRS,
    #[doc = "0x6c - Alarm value for Day of Month"]
    pub adom: ADOM,
    #[doc = "0x70 - Alarm value for Day of Week"]
    pub adow: ADOW,
    #[doc = "0x74 - Alarm value for Day of Year"]
    pub adoy: ADOY,
    #[doc = "0x78 - Alarm value for Months"]
    pub amon: AMON,
    #[doc = "0x7c - Alarm value for Year"]
    pub ayrs: AYRS,
}
#[doc = "ILR (rw) register accessor: Interrupt Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ilr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ilr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilr`]
module"]
pub type ILR = crate::Reg<ilr::ILR_SPEC>;
#[doc = "Interrupt Location Register"]
pub mod ilr;
#[doc = "CCR (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Control Register"]
pub mod ccr;
#[doc = "CIIR (rw) register accessor: Counter Increment Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciir`]
module"]
pub type CIIR = crate::Reg<ciir::CIIR_SPEC>;
#[doc = "Counter Increment Interrupt Register"]
pub mod ciir;
#[doc = "AMR (rw) register accessor: Alarm Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amr`]
module"]
pub type AMR = crate::Reg<amr::AMR_SPEC>;
#[doc = "Alarm Mask Register"]
pub mod amr;
#[doc = "CTIME0 (r) register accessor: Consolidated Time Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime0`]
module"]
pub type CTIME0 = crate::Reg<ctime0::CTIME0_SPEC>;
#[doc = "Consolidated Time Register 0"]
pub mod ctime0;
#[doc = "CTIME1 (r) register accessor: Consolidated Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime1`]
module"]
pub type CTIME1 = crate::Reg<ctime1::CTIME1_SPEC>;
#[doc = "Consolidated Time Register 1"]
pub mod ctime1;
#[doc = "CTIME2 (r) register accessor: Consolidated Time Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctime2`]
module"]
pub type CTIME2 = crate::Reg<ctime2::CTIME2_SPEC>;
#[doc = "Consolidated Time Register 2"]
pub mod ctime2;
#[doc = "SEC (rw) register accessor: Seconds Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "Seconds Counter"]
pub mod sec;
#[doc = "MIN (rw) register accessor: Minutes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`]
module"]
pub type MIN = crate::Reg<min::MIN_SPEC>;
#[doc = "Minutes Register"]
pub mod min;
#[doc = "HRS (rw) register accessor: Hours Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrs`]
module"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hours Register"]
pub mod hrs;
#[doc = "DOM (rw) register accessor: Day of Month Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dom`]
module"]
pub type DOM = crate::Reg<dom::DOM_SPEC>;
#[doc = "Day of Month Register"]
pub mod dom;
#[doc = "DOW (rw) register accessor: Day of Week Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dow`]
module"]
pub type DOW = crate::Reg<dow::DOW_SPEC>;
#[doc = "Day of Week Register"]
pub mod dow;
#[doc = "DOY (rw) register accessor: Day of Year Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doy`]
module"]
pub type DOY = crate::Reg<doy::DOY_SPEC>;
#[doc = "Day of Year Register"]
pub mod doy;
#[doc = "MONTH (rw) register accessor: Months Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`month::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`month::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@month`]
module"]
pub type MONTH = crate::Reg<month::MONTH_SPEC>;
#[doc = "Months Register"]
pub mod month;
#[doc = "YEAR (rw) register accessor: Years Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`year::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`year::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@year`]
module"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "Years Register"]
pub mod year;
#[doc = "CALIBRATION (rw) register accessor: Calibration Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calibration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calibration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calibration`]
module"]
pub type CALIBRATION = crate::Reg<calibration::CALIBRATION_SPEC>;
#[doc = "Calibration Value Register"]
pub mod calibration;
#[doc = "GPREG (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg`]
module"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpreg;
#[doc = "RTC_AUX (rw) register accessor: RTC Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_aux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_aux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_aux`]
module"]
pub type RTC_AUX = crate::Reg<rtc_aux::RTC_AUX_SPEC>;
#[doc = "RTC Auxiliary control register"]
pub mod rtc_aux;
#[doc = "RTC_AUXEN (rw) register accessor: RTC Auxiliary Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_auxen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_auxen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_auxen`]
module"]
pub type RTC_AUXEN = crate::Reg<rtc_auxen::RTC_AUXEN_SPEC>;
#[doc = "RTC Auxiliary Enable register"]
pub mod rtc_auxen;
#[doc = "ASEC (rw) register accessor: Alarm value for Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asec`]
module"]
pub type ASEC = crate::Reg<asec::ASEC_SPEC>;
#[doc = "Alarm value for Seconds"]
pub mod asec;
#[doc = "AMIN (rw) register accessor: Alarm value for Minutes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amin`]
module"]
pub type AMIN = crate::Reg<amin::AMIN_SPEC>;
#[doc = "Alarm value for Minutes"]
pub mod amin;
#[doc = "AHRS (rw) register accessor: Alarm value for Hours\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahrs`]
module"]
pub type AHRS = crate::Reg<ahrs::AHRS_SPEC>;
#[doc = "Alarm value for Hours"]
pub mod ahrs;
#[doc = "ADOM (rw) register accessor: Alarm value for Day of Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adom`]
module"]
pub type ADOM = crate::Reg<adom::ADOM_SPEC>;
#[doc = "Alarm value for Day of Month"]
pub mod adom;
#[doc = "ADOW (rw) register accessor: Alarm value for Day of Week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adow`]
module"]
pub type ADOW = crate::Reg<adow::ADOW_SPEC>;
#[doc = "Alarm value for Day of Week"]
pub mod adow;
#[doc = "ADOY (rw) register accessor: Alarm value for Day of Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adoy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adoy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adoy`]
module"]
pub type ADOY = crate::Reg<adoy::ADOY_SPEC>;
#[doc = "Alarm value for Day of Year"]
pub mod adoy;
#[doc = "AMON (rw) register accessor: Alarm value for Months\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amon`]
module"]
pub type AMON = crate::Reg<amon::AMON_SPEC>;
#[doc = "Alarm value for Months"]
pub mod amon;
#[doc = "AYRS (rw) register accessor: Alarm value for Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ayrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ayrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ayrs`]
module"]
pub type AYRS = crate::Reg<ayrs::AYRS_SPEC>;
#[doc = "Alarm value for Year"]
pub mod ayrs;
