use dev_csr::dev_csr;

dev_csr!{
    dev H3Lis {
        regs {
            /// Should be 32h
            0x0F WHO_AM_I r who_am_i,
            0x20 CTRL_REG1 rw {
                0 x_enable,
                1 y_enable,
                2 z_enable,
                3..4 data_rate,
                5..7 power_mode
            },
            0x21 CTRL_REG2 rw {
                0..1 high_pass_filter_cutoff_freq,
                2 high_pass_filter_enabled_interrupt_1,
                3 high_pass_filter_enabled_interrupt_2,
                /// Filtered data selection. Default value: 0
                /// (0: internal filter bypassed; 1: data from internal filter sent to output register)
                4 filtered_data_selection,
                /// 0 = normal mode
                5..6 high_pass_filter_mode,
                /// Reboots memory content when true
                7 boot
            },
            0x22 CTRL_REG3 rw,
            0x23 CTRL_REG4 rw,
            0x24 CTRL_REG5 rw,
            0x25 HP_FILTER_RESET r hp_filter_reset,
            0x26 REFERENCE rw reference,
            0x27 STATUS_REG r, 

        }
    }
}