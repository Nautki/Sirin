use dev_csr::dev_csr;

dev_csr! {
    dev Rfm9x{
        regs{
            0x12 IRQ_FLAG rw {
                /// Timeout interrupt: writing a 1 clears the IRQ
                7 rx_time_out,
                /// Packet reception complete interrupt: writing a 1 clears the IRQ
                6 rx_done,
                /// Payload CRC error interrupt: writing a 1 clears the IR
                5 payload_crc_err,
                /// Valid header received in Rx: writing a 1 clears theIRQ
                4 valid_header,
                /// FIFO Payload transmission complete interrupt: writing a 1 clears the IRQ
                3 tx_done,
                /// CAD complete: write to clear: writing a 1 clears the IRQ
                2 cad_done,
                /// FHSS change channel interrupt: writing a 1 clears the IR
                1 fhss_change_channel,
                /// Valid Lora signal detected during CAD operation: writing a 1clears the IR
                0 cad_detected,
            },
            /// Number of payload bytes of latest packetreceived
            0x13 RX_NB_BYTES r fifo_rx_bytes[0..7],
            /// Number of valid headers received since last transition intoRx mode, MSB(15:8). Header and packet counters are reseted in Sleep mode
            0x14 RX_HEADER_CNT_VALUE_MSB r valid_header_cnt[8..15],
            /// Number of valid headers received since last transition intoRx mode, LSB(7:0). Header and packet counters are reseted in Sleep mode.
            0x15 RX_HEADER_CNT_VALUE_LSB r valid_header_cnt[0..7],
            /// Number of valid packets received since last transition into Rx mode, MSB(15:8). Header and packet counters are reseted in Sleep mode.
            0x16 RX_PACKET_CNT_VALUE_MSB rw valid_packet_cnt[8..15],
            /// Number of valid packets received since last transition intoRx mode, LSB(7:0). Header and packet counters are reseted in Sleep mode
            0x17 RX_PACKET_CNT_VALUE_LSB r valid_packet_cnt[0..7],
            0x18 MODEM_STAT r {
                ///Signal detected 
                0 signal_detected,
                /// 1 Signal synchronized
                1 signal_synchronized,
                /// 2 RX on-going
                2 rx_on_going,
                /// 3 Header info valid
                3 header_info_valid,
                /// 4 Modem clear
                4 modem_clear
                /// Coding rate of last headerreceived
                5..7 rx_coding_rate,
            },
            /// Estimation of SNR on last packet received.In two’s compliment format multiplied by 4. SNR[dB] = PacketSnr[twos complement]/4
            0x19 PKT_SNR_VALUE r packet_snr[0..7],
            /// RSSI of the latest packet received (dBm): 
            ///RSSI[dBm] = -157 + Rssi (using HF output port, SNR >=0)
            ///or RSSI[dBm] = -164 + Rssi (using LF output port, SNR >= 0)
            /// (see section 5.5.5 for details)
            0x1A PKT_RSSI_VALUE r packet_rssi[0..7],
            /// Current RSSI value (dBm)
            /// RSSI[dBm] = -157 + Rssi (using HF outputport)
            /// or RSSI[dBm] = -164 + Rssi (using LF outputport)
            /// (see section 5.5.5 for details*/
            0x1B RSSI_VALUE r rssi[0..7],
            0x1C HOP_CHANNEL r {
                /// Current value of frequency hopping channel inuse.
                0..5 fhss_present_channel,
                /// CRC Information extracted from the received packetheader
                /// (Explicit header mode only)
                /// 0 -> Header indicates CRC off
                /// 1 -> Header indicates CRC on
                6 crc_on_payload,
                /// PLL failed to lock while attempting a TX/RX/CAD operation
                /// 1 -> PLL did not lock
                /// 0 -> PLL did lock
                7 pll_time_out,
            },

        }
    }

}