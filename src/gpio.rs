#![allow(dead_code)]

use embedded_hal::digital::v2::*;
use core::{marker::PhantomData, convert::Infallible};

const GP_FSEL0: *mut u32 = 0x7e200000 as *mut u32;
const GP_FSEL1: *mut u32 = 0x7e200004 as *mut u32;
const GP_FSEL2: *mut u32 = 0x7e200008 as *mut u32;
const GP_FSEL3: *mut u32 = 0x7e20000c as *mut u32;
const GP_FSEL4: *mut u32 = 0x7e200010 as *mut u32;
const GP_FSEL5: *mut u32 = 0x7e200014 as *mut u32;
const GP_SET0: *mut u32 = 0x7e20001c as *mut u32;
const GP_SET1: *mut u32 = 0x7e200020 as *mut u32;
const GP_CLR0: *mut u32 = 0x7e200028 as *mut u32;
const GP_CLR1: *mut u32 = 0x7e20002c as *mut u32;
const GP_LEV0: *mut u32 = 0x7e200034 as *mut u32;
const GP_LEV1: *mut u32 = 0x7e200038 as *mut u32;
const GP_EDS0: *mut u32 = 0x7e200040 as *mut u32;
const GP_EDS1: *mut u32 = 0x7e200044 as *mut u32;
const GP_REN0: *mut u32 = 0x7e20004c as *mut u32;
const GP_REN1: *mut u32 = 0x7e200050 as *mut u32;
const GP_FEN0: *mut u32 = 0x7e200058 as *mut u32;
const GP_FEN1: *mut u32 = 0x7e20005c as *mut u32;
const GP_HEN0: *mut u32 = 0x7e200064 as *mut u32;
const GP_HEN1: *mut u32 = 0x7e200068 as *mut u32;
const GP_HEN2: *mut u32 = 0x7e20006c as *mut u32;
const GP_LEN0: *mut u32 = 0x7e200070 as *mut u32;
const GP_LEN1: *mut u32 = 0x7e200074 as *mut u32;
const GP_AREN0: *mut u32 = 0x7e20007c as *mut u32;
const GP_AREN1: *mut u32 = 0x7e200080 as *mut u32;
const GP_AFEN0: *mut u32 = 0x7e200088 as *mut u32;
const GP_AFEN1: *mut u32 = 0x7e20008c as *mut u32;
const GP_PUD: *mut u32 = 0x7e200094 as *mut u32;
const GP_PUDCLK0: *mut u32 = 0x7e200098 as *mut u32;
const GP_PUDCLK1: *mut u32 = 0x7e20009c as *mut u32;
const GP_GPTEST: *mut u32 = 0x7e2000b0 as *mut u32;

const FUNCTION_INPUT: u32 = 0;
const FUNCTION_OUTPUT: u32 = 1;

#[inline]
fn set_pin_function(pin_num: u32, function: u32) {
    let function_select_reg = match pin_num / 10 {
        0 => GP_FSEL0,
        1 => GP_FSEL1,
        2 => GP_FSEL2,
        3 => GP_FSEL3,
        4 => GP_FSEL4,
        5 => GP_FSEL5,
        _ => unreachable!(),
    };

    unsafe {
        let shift_amount = (pin_num % 10) * 3;
        let mut reg = *function_select_reg;

        reg &= !0x07 << shift_amount;
        reg |= (function & 0x07) << shift_amount;

        *function_select_reg = reg;
    }
}

#[inline]
fn clear_pin(pin_num: u32) {
    let clear_reg = match pin_num / 31 {
        0 => GP_CLR0,
        1 => GP_CLR1,
        _ => unreachable!(),
    };

    unsafe {
        *clear_reg |= 1 << (pin_num % 32);
    }
}

#[inline]
fn set_pin(pin_num: u32) {
    let set_reg = match pin_num / 31 {
        0 => GP_SET0,
        1 => GP_SET1,
        _ => unreachable!(),
    };

    unsafe {
        *set_reg |= 1 << (pin_num % 32);
    }
}

/// Returns nonzero if level is high, 0 if level is low.
#[inline]
fn pin_level(pin_num: u32) -> u32 {
    let lev_reg = match pin_num / 31 {
        0 => GP_LEV0,
        1 => GP_LEV1,
        _ => unreachable!(),
    };

    unsafe {
        *lev_reg & (1 << (pin_num % 32))
    }
}

pub struct Input;
pub struct Output;

macro_rules! impl_gpio {
    ($pin_number:expr, $t:ident) => {
        pub struct $t<M>(PhantomData<M>);

        impl $t<Input> {
            pub fn new() -> Self {
                set_pin_function($pin_number, FUNCTION_INPUT);
                Self(PhantomData)
            }
        }

        impl InputPin for $t<Input> {
            type Error = Infallible;

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(pin_level($pin_number) != 0)
            }

            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(pin_level($pin_number) == 0)
            }
        }

        impl $t<Output> {
            pub fn new() -> Self {
                set_pin_function($pin_number, FUNCTION_OUTPUT);
                Self(PhantomData)
            }
        }

        impl OutputPin for $t<Output> {
            type Error = Infallible;

            fn set_high(&mut self) -> Result<(), Self::Error> {
                set_pin($pin_number);
                Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                clear_pin($pin_number);
                Ok(())
            }
        }
    };
}

pub mod pins {
    use super::*;

    impl_gpio!(0, Gp0);
    impl_gpio!(1, Gp1);
    impl_gpio!(2, Gp2);
    impl_gpio!(3, Gp3);
    impl_gpio!(4, Gp4);
    impl_gpio!(5, Gp5);
    impl_gpio!(6, Gp6);
    impl_gpio!(7, Gp7);
    impl_gpio!(8, Gp8);
    impl_gpio!(9, Gp9);
    impl_gpio!(10, Gp10);
    impl_gpio!(11, Gp11);
    impl_gpio!(12, Gp12);
    impl_gpio!(13, Gp13);
    impl_gpio!(14, Gp14);
    impl_gpio!(15, Gp15);
    impl_gpio!(16, Gp16);
    impl_gpio!(17, Gp17);
    impl_gpio!(18, Gp18);
    impl_gpio!(19, Gp19);
    impl_gpio!(20, Gp20);
    impl_gpio!(21, Gp21);
    impl_gpio!(22, Gp22);
    impl_gpio!(23, Gp23);
    impl_gpio!(24, Gp24);
    impl_gpio!(25, Gp25);
    impl_gpio!(26, Gp26);
    impl_gpio!(27, Gp27);
    impl_gpio!(28, Gp28);
    impl_gpio!(29, Gp29);
    impl_gpio!(30, Gp30);
    impl_gpio!(31, Gp31);
    impl_gpio!(32, Gp32);
    impl_gpio!(33, Gp33);
    impl_gpio!(34, Gp34);
    impl_gpio!(35, Gp35);
    impl_gpio!(36, Gp36);
    impl_gpio!(37, Gp37);
    impl_gpio!(38, Gp38);
    impl_gpio!(39, Gp39);
    impl_gpio!(40, Gp40);
    impl_gpio!(41, Gp41);
    impl_gpio!(42, Gp42);
    impl_gpio!(43, Gp43);
    impl_gpio!(44, Gp44);
    impl_gpio!(45, Gp45);
    impl_gpio!(46, Gp46);
    impl_gpio!(47, Gp47);
    impl_gpio!(48, Gp48);
    impl_gpio!(49, Gp49);
    impl_gpio!(50, Gp50);
    impl_gpio!(51, Gp51);
    impl_gpio!(52, Gp52);
    impl_gpio!(53, Gp53);
}
