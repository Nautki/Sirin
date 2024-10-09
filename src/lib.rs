use dev_csr::dev_csr;

dev_csr!{
    dev Rfm9x {
        regs {
            /// LoRa base-band FIFO data input/output. FIFO is cleared an not 
            /// accessible when device is in SLEEPmode
            0x00 REG_FIFO rw fifo,

            // isaac section 108-109

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
                0..1 symb_timeout[8..9],
                2 rx_payload_crc_on,
                3 tx_continuous_mode,
                4..7 spreading_factor
            },
            0x1F SYMB_TIMEOUT_LSB rw symb_timeout[0..7],
            0x20 PREAMBLE_MSB rw preamble_length[8..15],
            0x21 PREAMBLE_LSB rw preamble_length[0..7],
            0x22 PAYLOAD_LENGTH rw payload_length[0..7],
            0x23 MAX_PAYLOAD_LENGTH rw payload_max_length[0..7],
            0x24 HOP_PERIOD rw frew_hopping_period[0..7],
            0x25 FIFO_RX_BYTE_ADDR r fifo_rx_byte_addr_ptr,
            0x26 MODEM_CONFIG_3 rw {
                2 acg_auto_on,
                3 low_data_rate_optimize
            },
            0x27 PPM_CORRECTION rw ppm_correction,
            0x28 FEI_MSB r {
                0..3 freq_error[16..19]
            },
            0x29 FEI_MID r freq_error[8..15],
            0x2A FEI_LSB r freq_error[0..7],
            0x2C RSSI_WIDE_BAND r rssi_wide_band,
            0x2F IF_FREQ_2 rw if_freq_2,



        }
    }
}