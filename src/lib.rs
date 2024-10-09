use dev_csr::dev_csr;

dev_csr!{
    dev Rfm9x {
        regs {
            /// LoRa base-band FIFO data input/output. FIFO is cleared an not 
            /// accessible when device is in SLEEPmode
            0x00 REG_FIFO rw fifo,
        }
    }
}


dev_csr! {
    dev Rfm9x{
        regs{
            0x12 IRQ_FLAG rw {
                7 rx_time_out,
                6 rx_done,
                5 payload_crc_err,
                4 valid_header,
                3 tx_done,
                2 cad_done,
                1 fhss_change_channel,
                0 cad_detected,
            },
            0x13 RX_NB_BYTES r fifo_rx_bytes[0..7],
            0x14 RX_PACKET_CNT_VALUE_MSB r valid_header_cnt[8..15],
            0x15 RX_PACKET_CNT_VALUE_LSB r valid_header_cnt[0..7],
            0x16 RX_PACKET_CNT_VALUE_MSB rw valid_packet_cnt[8..15],
            0x17 RX_PACKET_CNT_VALUE_LSB r valid_packet_cnt[0..7],
            0x18 MODEM_STAT r {
                0..4 modem_status,
                5..7 rx_coding_rate,
            },
            0x19 PKT_SNR_VALUE r packet_snr[0..7],
            0x1A PKT_RSSI_VALUE r packet_rssi[0..7],
            0x1B RSSI_VALUE r rssi[0..7],
            0x1C HOP_CHANNEL r {
                0..5 fhss_present_channel,
                6 crc_on_payload,
                7 pll_time_out,
            },

        }
    }

}