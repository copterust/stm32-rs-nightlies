#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_RTC_STAMP_LSECSS_SSRU();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1();
    fn DAC();
    fn COMP();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM16();
    fn TIM17();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn LPUART1();
    fn LPTIM1();
    fn LPTIM2();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn LPTIM3();
    fn SPI3();
    fn HSEM();
    fn I2C3_EV();
    fn I2C3_ER();
    fn RADIO_BUSY();
    fn AES();
    fn TRUE_RNG();
    fn PKA();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMAMUX_OVR();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 62] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_RTC_STAMP_LSECSS_SSRU,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC1 },
    Vector { _handler: DAC },
    Vector { _reserved: 0 },
    Vector { _handler: COMP },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: LPTIM3 },
    Vector { _handler: SPI3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HSEM },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: RADIO_BUSY,
    },
    Vector { _handler: AES },
    Vector { _handler: TRUE_RNG },
    Vector { _handler: PKA },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector {
        _handler: DMAMUX_OVR,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG = 0,
    #[doc = "1 - PVD through EXTI"]
    PVD = 1,
    #[doc = "2 - Tamper, TimeStamp, LSECSS interrupt through"]
    TAMP_RTC_STAMP_LSECSS_SSRU = 2,
    #[doc = "3 - RTC wakeup interrupt through EXTI\\[19\\]"]
    RTC_WKUP = 3,
    #[doc = "4 - Flash memory global interrupt and Flash memory ECC"]
    FLASH = 4,
    #[doc = "5 - RCC global interrupt"]
    RCC = 5,
    #[doc = "6 - EXTI line 0 interrupt through EXTI"]
    EXTI0 = 6,
    #[doc = "7 - EXTI line 1 interrupt through EXTI"]
    EXTI1 = 7,
    #[doc = "8 - EXTI line 2 interrupt through EXTI"]
    EXTI2 = 8,
    #[doc = "9 - EXTI line 3 interrupt through EXTI"]
    EXTI3 = 9,
    #[doc = "10 - EXTI line 4 interrupt through EXTI"]
    EXTI4 = 10,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1 Channel3 interrupt"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1 Channel4 interrupt"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1 Channel5 interrupt"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1 Channel6 interrupt"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1 Channel 7 interrupt"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC1 global interrupt"]
    ADC1 = 18,
    #[doc = "19 - DAC global interrupt"]
    DAC = 19,
    #[doc = "21 - COMP2 and COMP1 interrupt through"]
    COMP = 21,
    #[doc = "22 - EXTI line 9_5 interrupt through EXTI"]
    EXTI9_5 = 22,
    #[doc = "23 - Timer 1 break interrupt"]
    TIM1_BRK = 23,
    #[doc = "24 - Timer 1 Update"]
    TIM1_UP = 24,
    #[doc = "25 - Timer 1 trigger and communication"]
    TIM1_TRG_COM = 25,
    #[doc = "26 - Timer 1 capture compare interrupt"]
    TIM1_CC = 26,
    #[doc = "27 - TIM2 global interrupt"]
    TIM2 = 27,
    #[doc = "28 - Timer 16 global interrupt"]
    TIM16 = 28,
    #[doc = "29 - Timer 17 global interrupt"]
    TIM17 = 29,
    #[doc = "30 - I2C1 event interrupt"]
    I2C1_EV = 30,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_ER = 31,
    #[doc = "32 - I2C2 error interrupt"]
    I2C2_EV = 32,
    #[doc = "33 - I2C2 error interrupt"]
    I2C2_ER = 33,
    #[doc = "34 - SPI 1 global interrupt"]
    SPI1 = 34,
    #[doc = "35 - SPI2 global interrupt"]
    SPI2 = 35,
    #[doc = "36 - USART1 global interrupt"]
    USART1 = 36,
    #[doc = "37 - USART2 global interrupt"]
    USART2 = 37,
    #[doc = "38 - LPUART1 global interrupt"]
    LPUART1 = 38,
    #[doc = "39 - LPtimer 1 global interrupt"]
    LPTIM1 = 39,
    #[doc = "40 - LPtimer 2 global interrupt"]
    LPTIM2 = 40,
    #[doc = "41 - EXTI line 15_10\\]
interrupt through EXTI"]
    EXTI15_10 = 41,
    #[doc = "42 - RTC Alarms A & B interrupt"]
    RTC_ALARM = 42,
    #[doc = "43 - LPtimer 3 global interrupt"]
    LPTIM3 = 43,
    #[doc = "44 - SPI3 global interrupt"]
    SPI3 = 44,
    #[doc = "47 - Semaphore interrupt 0 to CPU1"]
    HSEM = 47,
    #[doc = "48 - I2C3 event interrupt"]
    I2C3_EV = 48,
    #[doc = "49 - I2C3 error interrupt"]
    I2C3_ER = 49,
    #[doc = "50 - Radio IRQs RFBUSY interrupt through EXTI"]
    RADIO_BUSY = 50,
    #[doc = "51 - AES global interrupt"]
    AES = 51,
    #[doc = "52 - True random number generator interrupt"]
    TRUE_RNG = 52,
    #[doc = "53 - Private key accelerator interrupt"]
    PKA = 53,
    #[doc = "54 - DMA2 channel 1 interrupt"]
    DMA2_CH1 = 54,
    #[doc = "55 - DMA2 channel 2 interrupt"]
    DMA2_CH2 = 55,
    #[doc = "56 - DMA2 channel 3 interrupt"]
    DMA2_CH3 = 56,
    #[doc = "57 - DMA2 channel 4 interrupt"]
    DMA2_CH4 = 57,
    #[doc = "58 - DMA2 channel 5 interrupt"]
    DMA2_CH5 = 58,
    #[doc = "59 - DMA2 channel 6 interrupt"]
    DMA2_CH6 = 59,
    #[doc = "60 - DMA2 channel 7 interrupt"]
    DMA2_CH7 = 60,
    #[doc = "61 - DMAMUX overrun interrupt"]
    DMAMUX_OVR = 61,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Direct memory access controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma1;
#[doc = "Direct memory access controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMA request multiplexer"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA request multiplexer"]
pub mod dmamux;
#[doc = "Public key accelerator"]
pub struct PKA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA {}
impl PKA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka::RegisterBlock {
        0x5800_2000 as *const _
    }
}
impl Deref for PKA {
    type Target = pka::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA::ptr() }
    }
}
#[doc = "Public key accelerator"]
pub mod pka;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x5800_0400 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x5800_0000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "True random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x5800_1000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "True random number generator"]
pub mod rng;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Low-power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low-power timer"]
pub mod lptim1;
#[doc = "Low-power timer"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim2::RegisterBlock {
        0x4000_9400 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "Low-power timer"]
pub mod lptim2;
#[doc = "Low-power timer"]
pub struct LPTIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM3 {}
impl LPTIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim3::RegisterBlock {
        0x4000_9800 as *const _
    }
}
impl Deref for LPTIM3 {
    type Target = lptim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM3::ptr() }
    }
}
#[doc = "Low-power timer"]
pub mod lptim3;
#[doc = "General-purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General-purpose timers"]
pub mod tim16;
#[doc = "General-purpose timers"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim17::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim17::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "General-purpose timers"]
pub mod tim17;
#[doc = "Advanced-control timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-control timers"]
pub mod tim1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x4001_0030 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub mod vrefbuf;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x5800_1800 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub mod aes;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Inter-integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5801_0000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Hardware semaphore"]
pub struct HSEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSEM {}
impl HSEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsem::RegisterBlock {
        0x5800_1400 as *const _
    }
}
impl Deref for HSEM {
    type Target = hsem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSEM::ptr() }
    }
}
#[doc = "Hardware semaphore"]
pub mod hsem;
#[doc = "Analog to digital convertor"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4001_2400 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog to digital convertor"]
pub mod adc;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4800_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x4800_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x5800_0800 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct LPUART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART {}
impl LPUART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for LPUART {
    type Target = lpuart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod lpuart;
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "Tamper and backup registers"]
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAMP::ptr() }
    }
}
#[doc = "Tamper and backup registers"]
pub mod tamp;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvic_stir::RegisterBlock {
        0xe000_ef00 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[doc = "Microcontroller Debug Unit"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Microcontroller Debug Unit"]
pub mod dbgmcu;
#[doc = "System configuration controller"]
pub struct SYSCFG_CONTINUE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG_CONTINUE {}
impl SYSCFG_CONTINUE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg_continue::RegisterBlock {
        0x4001_0100 as *const _
    }
}
impl Deref for SYSCFG_CONTINUE {
    type Target = syscfg_continue::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG_CONTINUE::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg_continue;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "PKA"]
    pub PKA: PKA,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "LPUART"]
    pub LPUART: LPUART,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TAMP"]
    pub TAMP: TAMP,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "SYSCFG_CONTINUE"]
    pub SYSCFG_CONTINUE: SYSCFG_CONTINUE,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            COMP: COMP {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            PKA: PKA {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            LPTIM3: LPTIM3 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            HSEM: HSEM {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            LPUART: LPUART {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            SYSCFG_CONTINUE: SYSCFG_CONTINUE {
                _marker: PhantomData,
            },
        }
    }
}
