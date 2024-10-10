use dev_csr::dev_csr;


dev_csr!{
    dev Rfm9x {
        regs {
            0x0C LNA rw {
                ///High Frequency (RFI_HF) LNA currentadjustment, 00 -> Default LNA current
                ///11 -> Boost on, 150% LNA current
                0..1 lna_boost_hf,
                ///Low Frequency (RFI_LF) LNA current adjustment, 00 -> Default LNA current
                ///Other -> Reserved
                3..4 lna_boost_lf,
                ///LNA gain setting: 
                ///000 -> not used
                ///001 -> G1 = maximum gain 010 G2
                ///011 -> G 3
                ///100 -> G4
                ///101 -> G5
                ///110 -> G6 = minimum gain 
                ///111  -> not used
                5..7 lna_gain
            },
            ///SPI interface address pointer in FIFO data buffer.
            0x0D FIFO_ADDR_PTR r fifo_addr_ptr[0..7],
            ///write base address in FIFO data buffer for TX modulator
            0x0E FIFO_TX_BASE_ADDR rw fifo_tx_base_addr[0..7],
            ///read base address in FIFO data buffer for RX demodulator
            0x0F FIFO_RX_BASE_ADDR rw fifo_rx_base_addr[0..7],
            ///Start address (in data buffer) of last packet received
            0x10 FIFO_RX_CURRENT_ADDR r fifo_rx_current_addr[0..7],
            0x11 IRQ_FLAGS_MASK rw {
                ///Cad Detected Interrupt Mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                0 cad_detected_mask,
                ///FHSS change channel interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                1 fhss_change_channel_mask,
                ///CAD complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                2 cad_done_mask,
                ///FIFO Payload transmission complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                3 tx_done_mask,
                ///Valid header received in Rx mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                4 valid_header_mask,
                ///Payload CRC error interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                5 payload_crc_error_mask,
                ///Packet reception complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                6 rx_done_mask,
                ///Timeout interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
                7 rx_timeout_mask
            }
            
        }
    }
}