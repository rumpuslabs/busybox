use crate::gpio::*;
use crate::pac;
use core::marker::PhantomData;
use paste::paste;

pub trait AlternateFunction {}

pub struct GPIO<IO>(PhantomData<IO>);
impl<IO> AlternateFunction for GPIO<IO> where IO: PinDir {}

pub trait PinId {
    fn port(&self) -> usize;
    fn pin(&self) -> usize;
}

pub struct Pin<P, AF>
where
    P: PinId,
    AF: AlternateFunction,
{
    pub(crate) id: P,
    pub(crate) _marker: PhantomData<AF>,
}

pub struct PinIdStatic<const PORT: usize, const PIN: usize>;

impl<const PORT: usize, const PIN: usize> PinId for PinIdStatic<PORT, PIN> {
    fn port(&self) -> usize {
        PORT
    }

    fn pin(&self) -> usize {
        PIN
    }
}

pub struct PinIdDynamic {
    port: u8,
    pin: u8,
}

impl PinId for PinIdDynamic {
    fn port(&self) -> usize {
        self.port as usize
    }

    fn pin(&self) -> usize {
        self.pin as usize
    }
}

impl<const PORT: usize, const PIN: usize, AF> Pin<PinIdStatic<PORT, PIN>, AF>
where
    AF: AlternateFunction,
{
    fn new() -> Self {
        Self {
            id: PinIdStatic {},
            _marker: PhantomData,
        }
    }

    pub fn downgrade(self) -> Pin<PinIdDynamic, AF> {
        Pin {
            id: PinIdDynamic {
                port: PORT as u8,
                pin: PIN as u8,
            },
            _marker: PhantomData,
        }
    }
}

pub enum PinMode {
    PullUp,
    Repeater,
    Neither,
    PullDown,
}

macro_rules! functions {
    ($( $af:ident ),+ $(,)?) => {
        $(
            pub struct $af {}
            impl AlternateFunction for $af {}
        )+
    };
}

functions! {
    Ad0, Aout,
    Cap0, Cap1, Cap2, Cap3, Clkout, Cts1,
    Dcd1, Dsr1, Dtr1,
    Eint0, Eint1, Eint2, Eint3, EnetCrs, EnetMdc, EnetMdio, EnetRefClk, EnetRxd0, EnetRxd1, EnetRxEr, EnetTxd0, EnetTxd1, EnetTxEn,
    I2srxClk, I2srxSda, I2srxWs, I2stxClk, I2stxSda, I2stxWs,
    Mat0, Mat1, Mat2, Mat3, Mcabort, Mcoa0, Mcoa1, Mcoa2, Mcob0, Mcob1, Mcob2, Mci0, Mci1, Mci2, Miso, Miso0, Miso1, Mosi, Mosi0, Mosi1,
    Nmi,
    Pcap1, Pwm1,
    Rd1, Rd2, Ri1, Rts1, Rxd0, Rxd1, Rxd2, Rxd3, RxMclk,
    Sck, Sck0, Sck1, Scl1, Scl2, Sda1, Sda2, Ssel, Ssel0, Ssel1, Stclk,
    Td1, Td2, Txd0, Txd1, Txd2, Txd3, TxMclk,
    UsbConnect, UsbOvrcr, UsbPpwr, UsbPwrd, UsbUpLed,
    Vbus,
}

macro_rules! pins {
    ($( ( $port:literal, $pin:literal, $reg:literal $( , $( $af1:ident )? $( , $( $af2:ident )? $( , $af3:ident )?)?)? ) ),+ $(,)?) => (
        paste! {
            pub struct Pins {
                $(
                    pub [<p $port _ $pin >]: Pin<PinIdStatic<$port, $pin>, GPIO<Input>>,
                )+
            }

            impl Pins {
                pub(crate) fn new(_pinconnect: pac::PINCONNECT, _gpio: pac::GPIO) -> Self {
                    Pins {
                        $(
                            [<p $port _ $pin >]: Pin::new(),
                        )+
                    }
                }
            }

            $(
                impl<AF> Pin<PinIdStatic<$port, $pin>, AF>
                where
                    AF: AlternateFunction,
                {
                    pub fn set_mode(self, mode: PinMode) -> Self {
                        let pinconnect = pac::PINCONNECT::ptr();
                        unsafe {
                            (*pinconnect).[< pinmode $reg >].modify(|_,w| {
                                let field = w.[< p $port _ $pin mode >]();
                                match mode {
                                    PinMode::PullUp => field.pull_up(),
                                    PinMode::Repeater => field.repeater(),
                                    PinMode::Neither => field.disabled(),
                                    PinMode::PullDown => field.pull_down(),
                                }
                            })
                        };
                        self
                    }

                    pub fn into_gpio(self) -> Pin<PinIdStatic<$port, $pin>, GPIO<Input>> {
                        let pinconnect = pac::PINCONNECT::ptr();
                        unsafe {
                            (*pinconnect).[< pinsel $reg >].modify(|_,w| {
                                w.[< p $port _ $pin >]().[< gpio_p $port >]()
                            })
                        }
                        Pin::<PinIdStatic<$port, $pin>, GPIO<Output>>::new().into_input()
                    }


                    $(
                        $(
                            pub fn [< into_ $af1 >](self) -> Pin<PinIdStatic<$port, $pin>, [< $af1:camel >]> {
                                let pinconnect = pac::PINCONNECT::ptr();
                                unsafe {
                                    (*pinconnect).[< pinsel $reg >].modify(|_,w| {
                                        w.[< p $port _ $pin >]().[< $af1 >]()
                                    })
                                }
                                Pin::new()
                            }
                        )?

                        $(
                            $(
                                pub fn [< into_ $af2 >](self) -> Pin<PinIdStatic<$port, $pin>, [< $af2:camel >]> {
                                    let pinconnect = pac::PINCONNECT::ptr();
                                    unsafe {
                                        (*pinconnect).[< pinsel $reg >].modify(|_,w| {
                                            w.[< p $port _ $pin >]().[< $af2 >]()
                                        })
                                    }
                                    Pin::new()
                                }
                            )?

                            $(
                                pub fn [< into_ $af3 >](self) -> Pin<PinIdStatic<$port, $pin>, [< $af3:camel >]> {
                                    let pinconnect = pac::PINCONNECT::ptr();
                                    unsafe {
                                        (*pinconnect).[< pinsel $reg >].modify(|_,w| {
                                            w.[< p $port _ $pin >]().[< $af3 >]()
                                        })
                                    }
                                    Pin::new()
                                }

                            )?

                        )?

                    )?
                }
            )+
        }
    )
}

pins! {
    (0, 00, 0, rd1, txd3, sda1),
    (0, 01, 0, td1, rxd3, scl1),
    (0, 02, 0, txd0, ad0),
    (0, 03, 0, rxd0, ad0),
    (0, 04, 0, i2srx_clk, rd2, cap2),
    (0, 05, 0, i2srx_ws, td2, cap2),
    (0, 06, 0, i2srx_sda, ssel1, mat2),
    (0, 07, 0, i2stx_clk, sck1, mat2),
    (0, 08, 0, i2stx_ws, miso1, mat2),
    (0, 09, 0, i2stx_sda, mosi1, mat2),
    (0, 10, 0, txd2, sda2, mat3),
    (0, 11, 0, rxd2, scl2, mat3),
    (0, 15, 0, txd1, sck0, sck),
    (0, 16, 1, rxd1, ssel0, ssel),
    (0, 17, 1, cts1, miso0, miso),
    (0, 18, 1, dcd1, mosi0, mosi),
    (0, 19, 1, dsr1,, sda1),
    (0, 20, 1, dtr1,, scl1),
    (0, 21, 1, ri1,, rd1),
    (0, 22, 1, rts1,, td1),
    (0, 23, 1, ad0, i2srx_clk, cap3),
    (0, 24, 1, ad0, i2srx_ws, cap3),
    (0, 25, 1, ad0, i2srx_sda, txd3),
    (0, 26, 1, ad0, aout, rxd3),

    // These pins don't have PINMODE1 entries
    //(0, 27, 1, sda0, usb_sda),
    //(0, 28, 1, scl0, usb_scl),
    //(0, 29, 1, usb_dp),
    //(0, 30, 1, usb_dm),

    (1, 00, 2, enet_txd0),
    (1, 01, 2, enet_txd1),
    (1, 04, 2, enet_tx_en),
    (1, 08, 2, enet_crs),
    (1, 09, 2, enet_rxd0),
    (1, 10, 2, enet_rxd1),
    (1, 14, 2, enet_rx_er),
    (1, 15, 2, enet_ref_clk),
    (1, 16, 3, enet_mdc),
    (1, 17, 3, enet_mdio),
    (1, 18, 3, usb_up_led, pwm1, cap1),
    (1, 19, 3, mcoa0, usb_ppwr, cap1),
    (1, 20, 3, mci0, pwm1, sck0),
    (1, 21, 3, mcabort, pwm1, ssel0),
    (1, 22, 3, mcob0, usb_pwrd, mat1),
    (1, 23, 3, mci1, pwm1, miso0),
    (1, 24, 3, mci2, pwm1, mosi0),
    (1, 25, 3, mcoa1,, mat1),
    (1, 26, 3, mcob1, pwm1, cap0),
    (1, 27, 3, clkout, usb_ovrcr, cap0),
    (1, 28, 3, mcoa2, pcap1, mat0),
    (1, 29, 3, mcob2, pcap1, mat0),
    (1, 30, 3,, vbus, ad0),
    (1, 31, 3,, sck1, ad0),

    (2, 00, 4, pwm1, txd1),
    (2, 01, 4, pwm1, rxd1),
    (2, 02, 4, pwm1, cts1),
    (2, 03, 4, pwm1, dcd1),
    (2, 04, 4, pwm1, dsr1),
    (2, 05, 4, pwm1, dtr1),
    (2, 06, 4, pcap1, ri1),
    (2, 07, 4, rd2, rts1),
    (2, 08, 4, td2, txd2, enet_mdc),
    (2, 09, 4, usb_connect, rxd2, enet_mdio),
    (2, 10, 4, eint0, nmi),
    (2, 11, 4, eint1,, i2stx_clk),
    (2, 12, 4, eint2,, i2stx_ws),
    (2, 13, 4, eint3,, i2stx_sda),

    (3, 25, 7,, mat0, pwm1),
    (3, 26, 7, stclk, mat0, pwm1),

    (4, 28, 9, rx_mclk, mat2, txd3),
    (4, 29, 9, tx_mclk, mat2, rxd3),
}
