use crate::hal;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;

static SYSTEM_TICKS: AtomicUsize = AtomicUsize::new(0);

pub fn init(p: &mut hal::Peripherals) {
    p.syscon.scs.write(|w| w.oscrange().low().oscen().enabled());
    while p.syscon.scs.read().oscstat().is_not_ready() {}
    p.syscon.clksrcsel.write(|w| w.clksrc().main_osc());

    unsafe {
        p.syscon
            .pll0cfg
            .write(|w| w.msel0().bits(14).nsel0().bits(0));
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0xAA));
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0x55));

        p.syscon.pll0con.write(|w| w.plle0().set_bit());
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0xAA));
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0x55));
    }

    while p.syscon.pll0stat.read().plock0().bit_is_clear() {}

    unsafe {
        p.syscon.cclkcfg.write(|w| w.cclksel().bits(2));
        p.syscon
            .pll0con
            .write(|w| w.plle0().set_bit().pllc0().set_bit());
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0xAA));
        p.syscon.pll0feed.write(|w| w.pll0feed().bits(0x55));
    }

    // configure the system timer to wrap around every millisecond
    p.systick.set_clock_source(SystClkSource::Core);
    p.systick.set_reload(120_000 - 1); // 1ms
    p.systick.clear_current();
    p.systick.enable_counter();
    p.systick.enable_interrupt();
}

pub fn now() -> usize {
    SYSTEM_TICKS.load(Ordering::Relaxed)
}

#[exception]
fn SysTick() {
    SYSTEM_TICKS.fetch_add(1, Ordering::Relaxed);
}
