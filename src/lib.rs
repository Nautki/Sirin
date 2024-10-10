use dev_csr::dev_csr;

dev_csr!{
    dev Rfm9x {
        regs {
            /// LoRa base-band FIFO data input/output. FIFO is cleared an not 
            /// accessible when device is in SLEEPmode
            0x00 FIFO rw fifo,
            0x01 OP_MODE rw{
                /// 0 is FSK Mode, 1 is LoRA mode
                /// Can only be modified in sleep mode
                /// Write opperation on other devices is ignored
                7 rw long_range_mode,
                /// This bit operates when device is in LoRa mode; 
                /// if set it allows access to FSK registers page 
                /// located in address space (0x0D:0x3F) while in LoRa mode
                /// 0: Access LoRa registers page 0x0D:0x3F
                /// 1: Access FSK registers page(in mode LoRa)0x0D:0x3F
                6 rw access_shared_reg,
                /// Access Low Frequency Mode registers 
                /// 0: High Frequency Mode (access to HF test registers) 
                /// 1: Low Frequency Mode(access to LF test registers)
                3 r low_freq_mode_on,
                /// Device modes 
                /// 000 : SLEEP 
                /// 001 : STDBY 
                /// 010 : Frequency synthesis TX(FSTX) 
                /// 011 : Transmit(TX) 
                /// 100 : Frequency synthesis RX(FSRX) 
                /// 101 : Receive continuous(RXCONTINUOUS) 
                /// 110 : receive single(RXSINGLE) 
                /// 111 : Channel activity detection(CAD)
                0..2 rw mode
                //?rwt?
            },
            /// MSB or FR carrier frequency
            0x06 FR_MSB rw frf_23_16[0..7],
            0x07 FR_MID rw frf_15_8[0..7],
            /// LSB or RF carrier frequency
            0x08 FR_LSB rw frf_7_0[0..7],
            0x09 PA_CONFIG rw {
                /// Selects PA output pin
                /// 0: RFO pin. Output power is limited to +14dBm
                /// 1: PA_BOOST pin. Output power is limited to +20 dBm
                7 pa_select,
                /// Select max output power.
                /// PMax = 10.8 + 0.6*MaxPower [dBm]
                4..6 max_power,
                /// POut = PMax - (15-OutputPower) if PaSelect = 0 (RFO pin)
                /// POut = 17 - (15-OutputPower) if PaSelect = 1 (PA_BOOST pin)
                0..3 output_power
            },
            /// Rise/Fall time of ramp up/down in FSK
            /// 0000 -> 3.4ms 
            /// 0001 -> 2ms 
            /// 0010 -> 1ms 
            /// 0011 -> 500us 
            /// 0100 -> 250us 
            /// 0101 -> 125us 
            /// 0110 -> 100us 
            /// 0111 -> 62us 
            /// 1000 -> 50us 
            /// 1001 -> 40us 
            /// 1010 -> 31us 
            /// 1011 -> 25us 
            /// 1100 -> 20us 
            /// 1101 -> 15us 
            /// 1110 -> 12us 
            /// 1111 -> 10us
            0x0A PA_RAMP rw pa_ramp, /// IDK

            0x0B OCP rw{
                /// Trimming of OCP current: 
                /// Imax = 45 + 5 * OcpTrim[mA] if OcpTrim <= 15 (120mA)/
                /// Imax = -30 + 10 * OcpTrim[mA] if 15 < OcpTrim <= 27 (130 to 240mA) 
                /// Imax = 240mA for higher settings 
                /// Default Imax=100mA
                5 ocp_on,
                0..4 ocp_trim,
            },
        }
    }
}