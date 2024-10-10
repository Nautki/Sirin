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


            
                   
            0x1D MODEM_CONFIG_1 rw {
                0 implicit_header_mode_on,
                /// Error coding rate
                /// 001 -> 4/5
                /// 010 -> 4/6
                /// 011 -> 4/7
                /// 100 -> 4/8
                /// All other values -> reserved
                /// In implicit header mode should be set on receiver todetermineexpected coding rate. See 4.1.1.3
                1..3 coding_rate,
                /// Signal bandwidth:
                /// 0000 -> 7.8 kHz
                /// 0001 -> 10.4 kHz
                /// 0010 -> 15.6 kHz
                /// 0011 -> 20.8kHz
                /// 0100 -> 31.25 kHz
                /// 0101 -> 41.7 kHz
                /// 0110 -> 62.5 kHz
                /// 0111 -> 125 kHz
                /// 1000 -> 250 kHz
                /// 1001 -> 500 kHz
                /// other values -> reserved
                /// In the lower band (169MHz), signal bandwidths 8&9 are not supported)
                4..7 bw
            },
            0x1E MODEM_CONFIG_2 rw {
                /// RX Time-Out MSB
                0..1 symb_timeout[8..9],
                /// Enable CRC generation and check onpayload:
                /// 0 -> CRC disable
                /// 1 -> CRC enable
                /// If CRC is needed, RxPayloadCrcOn should beset: 
                /// - in Implicit header mode: on Tx and Rx side
                /// - in Explicit header mode: on the Tx side alone (recoveredfromtheheader in Rx side)
                2 rx_payload_crc_on,
                /// 0 -> normal mode, a single packet is sent
                /// 1 -> continuous mode, send multiple packets across the FIFO(used for spectral analysis)
                3 tx_continuous_mode,
                /// SF rate (expressed as a base-2logarithm)
                /// 6 -> 64 chips / symbol
                /// 7 -> 128 chips / symbol
                /// 8 -> 256 chips / symbol
                /// 9 -> 512 chips / symbol
                /// 10 -> 1024 chips / symbol
                /// 11 -> 2048 chips / symbol
                /// 12 -> 4096 chips / symbol
                /// other values reserved.
                4..7 spreading_factor
            },
            /// RX Time-Out LSB
            /// RX operation time-out value expressed as number of symbols:
            /// TimeOut = SymbTimeout * Ts
            0x1F SYMB_TIMEOUT_LSB rw symb_timeout[0..7],
            /// Preamble length MSB, = PreambleLength + 4.25Symbols
            /// See 4.1.1 for more details
            0x20 PREAMBLE_MSB rw preamble_length[8..15],
            /// Preamble Length LSB
            0x21 PREAMBLE_LSB rw preamble_length[0..7],
            /// Payload length in bytes. The register needs to be set in implicit header mode for the expected packet length. A 0 value is not permitted
            0x22 PAYLOAD_LENGTH rw payload_length[0..7],
            /// Maximum payload length; if header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size.
            0x23 MAX_PAYLOAD_LENGTH rw payload_max_length[0..7],
            /// Symbol periods between frequency hops. (0 = disabled). 1st hop always happen after the 1st header symbol
            0x24 HOP_PERIOD rw frew_hopping_period[0..7],
            /// Current value of RX databuffer pointer (address of last byte written by Lora receiver)
            0x25 FIFO_RX_BYTE_ADDR r fifo_rx_byte_addr_ptr,
            0x26 MODEM_CONFIG_3 rw {
                /// 0 -> LNA gain set by register LnaGain
                /// 1 -> LNA gain set by the internal AGC loo
                2 acg_auto_on,
                /// 0 -> Disabled
                /// 1 -> Enabled; mandated for when the symbol lengthexceeds16ms
                3 low_data_rate_optimize
            },
            /// Data rate offset value, used in conjunction with AFC
            0x27 PPM_CORRECTION rw ppm_correction,
            0x28 FEI_MSB r {
                /// Estimated frequency error from modem
                /// MSB of RF Frequency Error
                /// F_error = (FreqError * 2^24)/F_xtal * BW[kHz]/500
                0..3 freq_error[16..19]
            },
            /// Middle byte of RF FrequencyError
            0x29 FEI_MID r freq_error[8..15],
            /// LSB of RF Frequency Error
            0x2A FEI_LSB r freq_error[0..7],
            /// Wideband RSSI measurement used to locally generate a random number
            0x2C RSSI_WIDE_BAND r rssi_wide_band,
            /// See errata note
            0x2F IF_FREQ_2 rw if_freq_2,
        }
    }
}