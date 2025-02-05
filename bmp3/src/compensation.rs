//! Should follow the reference implementation given
//! in `bmp3.c` from Bosch Sensortec.
//! https://github.com/boschsensortec/BMP3_SensorAPI

use uunit::{Celsius, Pascals, WithUnits};

macro_rules! trim_data {
    ($raw_trim_data:ident, $trim_data:ident, $constr:ident; $( $ident:ident : $ty: ident $(- 2**$sub:literal)? >> $bits:expr),* $(,)?) => {
        #[repr(C, packed)]
        pub struct $raw_trim_data {
            $(pub $ident: $ty),*
        }

        impl $raw_trim_data {
            pub fn swap_bytes(&mut self) {
                $(
                    self.$ident = self.$ident.swap_bytes();
                )*
            }
        }

        /// AKA "Quantized"
        #[derive(Default)]
        pub struct $trim_data {
            $(pub $ident: f64),*
        }

        // Private constructor function to be called from public impl
        fn $constr(raw: &$raw_trim_data) -> $trim_data {
            $trim_data {
                $($ident: {
                    let raw = raw.$ident as f64;
                    // TODO: make constant
                    let sub: f64 = 0.0 $(+ powi(2.0, $sub))?;
                    let div: f64 = powi(2.0, $bits);
                    (raw - sub) / div
                }),*
            }
        }
    };
}

// Reference BST-BMP388-DS001-07 (Rev 1.7) Page 55, Section 9.1. Calibration Coefficient.
// This defines two structs, [Compensator] (`bmp3_reg_calib_data`) and
// [TrimData] (`bmp3_quantized_calib_data`)
trim_data! {
    RawTrimData, Compensator, trim_data_from_raw;

    // <var name>: <type> [- subtrahend] >> <power of 2 to divide by>
    par_t1 : u16         >> -8,
    par_t2 : u16         >> 30,
    par_t3 :  i8         >> 48,
    par_p1 : i16 - 2**14 >> 20,
    par_p2 : i16 - 2**14 >> 29,
    par_p3 :  i8         >> 32,
    par_p4 :  i8         >> 37,
    par_p5 : u16         >> -3,
    par_p6 : u16         >>  6,
    par_p7 :  i8         >>  8,
    par_p8 :  i8         >> 15,
    par_p9 : i16         >> 48,
    par_p10:  i8         >> 48,
    par_p11:  i8         >> 65,
}

impl Compensator {
    pub fn from_raw(raw: &RawTrimData) -> Self {
        trim_data_from_raw(raw)
    }

    pub fn temperature(&self, raw_temperature: i64) -> Celsius<f64> {
        let raw_temp = raw_temperature as f64;
        let d1 = raw_temp - self.par_t1;
        let d2 = d1 * self.par_t2;
        let res = d2 + (d1 * d1) * self.par_t3;
        res.with_units()
    }

    pub fn pressure(&self, raw_pressure: u64, temperature: Celsius<f64>) -> Pascals<f64> {
        let raw_pressure = raw_pressure as f64;

        let d1 = self.par_p6 * temperature.value;
        let d2 = self.par_p7 * powi(temperature.value, 2);
        let d3 = self.par_p8 * powi(temperature.value, 3);
        let out1 = self.par_p5 + d1 + d2 + d3;

        let d1 = self.par_p2 * temperature.value;
        let d2 = self.par_p3 * powi(temperature.value, 2);
        let d3 = self.par_p4 * powi(temperature.value, 3);
        let out2 = raw_pressure * (self.par_p1 + d1 + d2 + d3);

        let d1 = powi(raw_pressure, 2);
        let d2 = self.par_p9 + self.par_p10 * temperature.value;
        let d3 = d1 * d2;
        let out3 = d3 + powi(raw_pressure, 3) * self.par_p11;

        (out1 + out2 + out3).with_units()
    }
}

// why is this not in core??
fn powi(base: f64, exp: i32) -> f64 {
    let mut acc = 1.0;
    let mut i = 0;
    if exp < 0 {
        // negative exponents
        while i < -exp {
            acc /= base;
            i += 1;
        }
    } else {
        while i < exp {
            acc *= base;
            i += 1;
        }
    }
    
    acc
}