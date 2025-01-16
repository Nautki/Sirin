#![no_std]

use core::{fmt::Debug, mem};

use dev_csr::dev_csr;
use embassy_futures::yield_now;
use embedded_hal::spi::{ ErrorKind as SpiError, ErrorType};
use embedded_hal_async::spi::SpiBus;
use spi_handle::SpiHandle;


dev_csr! {
    dev Lsm6dso{
        regs{
           0x01 FUNC_CFG_ACCESS rw //enables access to some embedded functions registers
           {
                7 emb_func, //embedded functions config register access, default 0
                6 shub_reg, //sensor hub config reg access, default 0
                3 fsm_wr_ctrl, //let FSM control the CTRL registers, default 0
                2 global_reset, //global reset of the device, default 0
                1 spi2_reset, //pulse 1 to reset the control registers of SPI2, default 0
                0 ois_from_ui, //Enables control of OIS config from main UI, default 0
           },
           0x02 PIN_CTRL rw //Pull-up control register for SDO, OCS-Aux, and SDO_Aux pins
           {
                7 ois_pullup_L, //1 disables pull-up on OCS_Aux and SDO_Aux pins, 0 enables, default 0
                6 sdo_pullup, //Enables pull-up on SDO pin, default 0
                5 post_reset_action, //0: config reset (SW reset + dyn addr reset), 1: global reset (POR reset)
           },
           0x03 IF_CFG rw
           {
                7 sda_pullup, //Enable pull-up on SDA pin, default 0
                6 i2c_aux_pullup, //1: enable internal pull-up on aux I2C line
                5 anti_spike_filter, //1: anti-spike on SCL and SDA always on, 0: filter managed by protocol
                4 interrupt_active, //0: active high interrupt, 1: active low interrupt, default 0
                3 pp_od, //Push-pull/open-drain selection on INT1 and INT2 pins. Default value: 0 (0: push-pull mode; 1: open-drain mode)
                2 spi_mode_select, //0: 4-wire interface, 1: 3-wire interface, default 0
                0 i2c_disable, //0: i2c and i3c enabled, 1: disabled, default 0
           },
           0x06 ODR_TRIG_CFG rw odr_data_amount, //ODR-triggered mode configuration register (R/W), determines amount of data generated during ref period in ODR-triggered mode. Allowed vals: 0, 4-255
           0x07 FIFO_CTRL1 rw fifo_wtm, //FIFO watermark threshold, 1 LSB = TAG (1 byte) + 1 sensor (6 bytes) in FIFO, flag rises when #bytes in FIFO > threshold
           0x08 FIFO_CTRL2 rw
           {
                7 stop_fifo_on_wtm, //Limit FIFO depth to threshold level
                6 fifo_compr, //Enable compression
                4 fifo_batch_odrchg, //Batch ODR nCHANGE sensor in FIFO
                2..1 fifo_uncompr_rate, //Configures the rate of written uncompressed data (default 0). 0: do not force uncompressed data writing, 1: uncompressed data every 8 batch, 2: every 16 batch, 3: every 32 batch
                0 fifo_dualc_fsm //Enable FSM-triggered batching of channel 2 when available, 0 disabled, 1 enabled, default 0
           },
           0x09 FIFO_CTRL3 rw
           {
                7..4 fifo_gyro_bdr, //Select batch data rate for gyro data. 0000: gyro not batched, 0001: 1.875Hz, otherwise rate = 1.875*2^{input}, max input 1100
                3..0 fifo_accel_bdr //Select batch data rate for accelerometer. Same as gyro.
           },
           0x0A FIFO_CTRL4 rw
           {
                7..6 fifo_ts_decim, //Select decimation for timestamp batching. 00: timestamp not batched. 01: write rate = max(fifo_accel_bdr, fifo_gyro_bdr). 10: write rate = max(fifo_accel_bdr, fifo_gyro_bdr)/8.  11: write rate = max(fifo_accel_bdr, fifo_gyro_bdr)/32.
                5..4 fifo_temp_bdr, //Select batch data rate for temperature. 00: not batched. 01: 1.875Hz. 10: 15Hz. 11: 60Hz.
                3 fifo_gyro_eis, //Enable gyroscope EIS value batching, 0 disabled, 1 enabled, default 0.
                2..0 fifo_mode //Mode selection.
                    //000: FIFO disabled.
                    //001: FIFO mode: stop collection when FIFO is full.
                    //010: continuousWTM-to-full mode: cont. mode with wtm until
                    //trigger is deasserted, then FIFO mode,
                    //011: continuous-to-FIFO mode: continuous until trigger is
                    //deasserted, then FIFO mode.
                    //100: bypass-to-continuous mode: FIFO disabled until trigger
                    //is deasserted, then continuous mode.
                    //101: reserved.
                    //110: continuous mode: if FIFO is full, new samples overwrite
                    //old ones.
                    //111: bypass-to-FIFO mode: FIFO disabled until trigger is
                    //deasserted, then FIFO mode.
           },
           0x0B COUNTER_BDR_REG1 rw
           {
                6..5 trig_ctr_bdr, //Select the trigger for the internal counter of batch events for the accel, gyro, and EIS gyro.
                    //00: accel batch event.
                    //01: gyro batch event.
                    //10-11: gyro EIS batch event.
                1..0 batch_counter_thresh[0..1] //Sets the threshold for the internal
                    //counter of batch events. When this counter reaches the
                    //threshold, the counter is reset and counter_bdr_reached is
                    //set to 1.
           },
           0x0C COUNTER_BDR_REG2 rw batch_counter_thresh[2..9],
           0x0D INT1_CTRL rw //INT1 pin control register.
               //Output is the OR combination of all selected here and in MD1_CFG.
               //All bits default 0.
           {
                6 int1_cnt_bdr, //Enables COUNTER_BDR_IA interrupt on INT1 pin.
                5 int1_fifo_full, //Enables FIFO full flag interrupt on INT1 pin.
                4 int1_fifo_over, //Enables FIFO overrun interrupt on INT1 pin.
                3 int1_fifo_thresh, //Enables FIFO threshold interrupt on INT1 pin.
                1 int1_gyro_rdy, //Enables gyro data-ready interrupt on INT1 pin.
                0 int1_accel_rdy //Enables accel data-ready interrupt on INT1 pin.
           },
           0x0E INT2_CTRL rw //INT2 pin control register.
               //Output is the OR combination of all selected here and in MD2_CFG.
               //All defaults 0.
           {
                7 int2_endop, //Enables routing the embedded-func ENDOP signal to
                    //the INT2 pin.
                6 int2_cnt_bdr, //Enables COUNTER_BDR_IA interrupt on INT2 pin.
                5 int2_fifo_full, //Enables FIFO full interrupt on INT2 pin.
                4 int2_fifo_over, //Enables FIFO overrun interrupt on INT2 pin.
                3 int2_fifo_thresh, //Enables FIFO threshold interrupt on INT2 pin.
                2 int2_eis_rdy, //Enables gyro EIS data-ready interrupt on INT2 pin.
                1 int2_gyro_rdy, //Enables gyro data-ready interrupt on INT2 pin.
                0 int2_accel_rdy //Enables accel data-ready interrupt on INT2 pin.
           },
           0x0F WHO_AM_I r whoami, //whoami value. Read-only, fixed at 0x70.
           0x10 CTRL1 rw
           {
                6..4 accel_mode, //Accel op mode selection.
                    //000: high-performance mode (default).
                    //001: high-accuracy ODR mode.
                    //010: reserved.
                    //011: ODR-triggered mode.
                    //100: low-power mode 1 (2 mean).
                    //101: low-power mode 2 (4 mean).
                    //110: low-power mode 3 (8 mean).
                    //111: normal mode.
                3..0 accel_odr //Accel ODR selection.
                    //0000: power down (default)
                    //0001: 1.875Hz (low power)
                    //0010: 7.5Hz (high-performance, normal)
                    //0011: 15Hz (LP, HP, normal)
                    //0100: 30Hz (LP, HP, normal)
                    //0101: 60Hz (LP, HP, normal)
                    //0110: 120Hz (LP, HP, normal)
                    //0111: 240Hz (LP, HP, normal)
                    //1000: 480Hz (HP, normal)
                    //1001: 960Hz (HP, normal)
                    //1010: 1.92kHz (HP, normal)
                    //1011: 3.84kHz (HP)
                    //1100: 7.68kHz (HP)
                    //others: reserved
           },
           0x11 CTRL2 rw
           {
                6..4 gyro_mode, //Gyro op mode select.
                    //000: high-performance (default)
                    //001: high-accuracy ODR
                    //010: reserved
                    //011: ODR-triggered
                    //100: sleep
                    //101: low-power
                    //110-111: reserved
                3..0 gyro_odr //Accel ODR selection.
                    //0000: power down (default)
                    //0010: 7.5Hz (high-performance, normal)
                    //0011: 15Hz (LP, HP, normal)
                    //0100: 30Hz (LP, HP, normal)
                    //0101: 60Hz (LP, HP, normal)
                    //0110: 120Hz (LP, HP, normal)
                    //0111: 240Hz (LP, HP, normal)
                    //1000: 480Hz (HP, normal)
                    //1001: 960Hz (HP, normal)
                    //1010: 1.92kHz (HP, normal)
                    //1011: 3.84kHz (HP)
                    //1100: 7.68kHz (HP)
                    //others: reserved
           },
           0x12 CTRL3 rw
           {
                7 mem_reset, //Resets memory content. Automatically clears.
                6 block_upd, //Block data update.
                    //0: continuous update
                    //1: output registers are not updated until LSB or MSB have been
                    //read.
                    //default: 1
                2 auto_inc, //Auto-increment addresses during multiple-byte serial
                    //access. Default 1.
                    //0: disabled, 1: enabled.
                0 sw_reset //Software reset, resets all control registers.
                    //Automatically cleared. Default 0.
           },
           0x13 CTRL4 rw
           {
                4 int2_on_int1, //OR INT2 output onto INT1 output
                3 mask_rdy, //Mask data-ready signal, default 0 (disabled),
                    //until filter setting ends.
                2 int2_temp_dry, //Enable temperature sensor data-ready interrupt
                    //on INT2 pin.
                1 pulsed_rdy, //Enable pulsed data-ready mode.
                0 int2_input_ah //Is INT2 input trigger active high
           },
           0x14 CTRL5 rw
           {
                2..1 bus_act_sel, //Bus available time select for IBI.
                    //00: 2u.
                    //01: 50u (default).
                    //10: 1ms.
                    //11: 25ms.
                0 int_i3c //Enables INT pin when I3C is enabled. Default 0.
           },
           0x15 CTRL6 rw
           {
                6..4 gyro_lpbw, //Gyro low-pass bandwidth select. See datasheet.
                3..0 gyro_fs //Gyro UI chain full-scale select.
                    //0000: �125dps (default)
                    //0001: �250dps
                    //0010: �500dps
                    //0011: �1000dps
                    //0100: �2000dps
                    //1100: �4000dps
                    //Others: reserved
           },
           0x16 CTRL7 rw
           {
                0 gyro_lp_enable //enable gyro low-pass filter
           },
           0x17 CTRL8 rw
           {
                7..5 accel_hplp_bw, //Accel high-pass and low-pass bandwidth.
                    //See datasheet because big table.
                3 accel_dc, //Enables dual-channel mode. Default 0 (disabled).
                1..0 accel_fs //Accel full-scale select.
                    //00: �2g
                    //01: �4g
                    //10: �8g
                    //11: �16g
           },
           0x18 CTRL9 rw
           {
                6 accel_hpref, //Enabled accel high-pass reference mode. Default 0.
                5 accel_fast_settle, //Enables filter fast settling mode. Def 0.
                4 accel_hp_slope, //Accel slope filter/HP filter selection.
                    //0: LP filter path
                    //1: HP filter path
                3 accel_lpf2, //enables second stage filtering for accel
                1 accel_ofs_w, //Accel user register offset weight
                    //0: 2^-10g/LSB
                    //1: 2^-6g/LSB
                0 accel_ofs_en //Enables accel user offset correction block.
           },
           0x19 CTRL10 rw
           {
                6 emb_debug, //Enables embedded function debug mode
                3..2 gyro_st, //Gyro self-test select
                    //00: normal (default)
                    //01: positive sign
                    //10: negative sign
                    //11: reserved
                1..0 accel_st //Accel self-test select
                    //00: normal (default)
                    //01: positive sign
                    //10: negative sign
                    //11: reserved
           },
           0x1A CTRL_STATUS r
           {
                2 ctrl_status //Flag that indicates the current controller of
                    //the device's config registers. Default 0.
                    //0: All registers and configs are available from the standard
                    //interface.
                    //1: Some registers and configs are under FSM control and are 
                    //in read-only mode from the standard interface.
           },
           0x1B FIFO_STATUS1 r fifo_diff[0..7], //Number of unread sensor data
                //(TAG + 6 bytes) stored in FIFO.
           0x1C FIFO_STATUS2 r
           {
                7 fifo_wtm_ia, //FIFO watermark status
                6 fifo_ovr_ia, //FIFO overrun status
                5 fifo_full_ia, //FIFO full status, 1: will be full at next ODR
                4 ctr_bdr_ia, //COUNTER_BDR_IA status, shows if threshold reached
                3 fifo_ovr_latched, //FIFO latched overrun status
                0 fifo_diff[8]
           },
           0x1D ALL_INT_SRC r //Source register for all interrupts
           {
                7 emb_ia, //embedded func interrupt status
                6 shub_ia, //sensor hub interrupt status
                5 sleep_ia, //Detects change in activity/inactivity status
                4 d6d_ia, //Orientation change status
                2 tap_ia, //single or double tap event detection status
                1 wu_ia, //wake event status
                0 ff_ia //free-fall event status
           },
           0x1E STATUS_REG r
           {
                7 ts_endcnt, //Timestamp overflow alert
                5 ois_rdy, //Accel or gyro OIS data ready
                4 gda_eis, //EIS gyro data ready
                2 temp_da, //temp data available
                1 gyro_da, //gyro data available
                0 accel_da //accel data available
           },
           0x20 OUT_TEMP_L r temp_data[8..15], //Temp data output register
           0x21 OUT_TEMP_H r temp_data[0..7],
           0x22 OUTX_L_G r gyro_pitch_rate[8..15], //Gyro pitch axis angular rate
           0x23 OUTX_H_G r gyro_pitch_rate[0..7],
           0x24 OUTY_L_G r gyro_roll_rate[8..15], //Gyro roll axis angular rate
           0x25 OUTY_H_G r gyro_roll_rate[0..7],
           0x26 OUTZ_L_G r gyro_yaw_rate[8..15], //Gyro yaw axis angular rate
           0x27 OUTZ_H_G r gyro_yaw_rate[0..7],
           0x28 OUTX_L_A r accel_x[8..15], //Accel x output
           0x29 OUTX_H_A r accel_x[0..7],
           0x2A OUTY_L_A r accel_y[8..15], //Accel y output
           0x2B OUTY_H_A r accel_y[0..7],
           0x2C OUTZ_L_A r accel_z[8..15], //Accel z output
           0x2D OUTZ_H_A r accel_z[0..7],
           0x2E UI_OUTX_L_G_OIS_EIS r x_ois_eis[8..15], //x axis OIS/EIS
           0x2F UI_OUTX_H_G_OIS_EIS r x_ois_eis[0..7],
           0x30 UI_OUTY_L_G_OIS_EIS r y_ois_eis[8..15], //y axis OIS/EIS
           0x31 UI_OUTY_H_G_OIS_EIS r y_ois_eis[0..7],
           0x32 UI_OUTZ_L_G_OIS_EIS r z_ois_eis[8..15], //z axis OIS/EIS
           0x33 UI_OUTZ_H_G_OIS_EIS r z_ois_eis[0..7],
           0x34 UI_OUTX_L_A_OIS_DualC r x_ois_dc[8..15], //x axis OIS/DualC
           0x35 UI_OUTX_H_A_OIS_DualC r x_ois_dc[0..7],
           0x36 UI_OUTY_L_A_OIS_DualC r y_ois_dc[8..15], //y axis OIS/DualC
           0x37 UI_OUTY_H_A_OIS_DualC r y_ois_dc[0..7],
           0x38 UI_OUTZ_L_A_OIS_DualC r z_ois_dc[8..15], //z axis OIS/DualC
           0x39 UI_OUTZ_H_A_OIS_DualC r z_ois_dc[0..7],
           //0x40 - 0x43: Timestamp output registers: 1LSB = 25 μs
           0x40 TIMESTAMP0 r timestamp[31..24],
           0x41 TIMESTAMP1 r timestamp[23..16],
           0x42 TIMESTAMP2 r timestamp[15..8],
           0x43 TIMESTAMP3 r timestamp[7..0],
           0x56 TAP_CFG0 rw
           {
                6 int_clr_on_read, //This bit allows immediately clearing the latched interrupts of an event detection
                //upon the read of the corresponding status register. It must be set to 1 together
                //with LIR. Default value: 0
                //(0: latched interrupt signal cleared at the end of the ODR period;
                //1: latched interrupt signal immediately cleared)
                5 sleep_status_on_int, //Activity/inactivity interrupt mode configuration.
                //If INT1_SLEEP_CHANGE or INT2_SLEEP_CHANGE bits are enabled, drives
                //the sleep status or sleep change on the INT pins. Default value: 0
                //(0: sleep change notification on INT pins; 1: sleep status reported on INT pins)
                4 slope_fds, //HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions.
                //Default value: 0 (0: SLOPE filter applied; 1: HPF applied)
                3 tap_x_en, //Enable X direction in tap recognition. Default value: 0
                //(0: X direction disabled; 1: X direction enabled)
                2 tap_y_en, //Enable Y direction in tap recognition. Default value: 0
                //(0: Y direction disabled; 1: Y direction enabled)
                1 tap_z_en, //Enable Z direction in tap recognition. Default value: 0
                //(0: Z direction disabled; 1: Z direction enabled)
                0 lir //Latched Interrupt. Default value: 0
                //(0: interrupt request not latched; 1: interrupt request latched)
           },
           0x57 TAP_CFG1 rw
           {
                7..5 tap_priority, //Selection of axis priority for TAP detection (see Table 119)
                //TAP_PRIORITY_[2:0] | Max. priority | Mid. priority | Min. priority
                //000 | X | Y | Z
                //001 | Y | X | Z
                //010 | X | Z | Y
                //011 | Z | Y | X
                //100 | X | Y | Z
                //101 | Y | Z | X
                //110 | Z | X | Y
                //111 | Z | Y | X
                4..0 tap_ths_x //X-axis tap recognition threshold. Default value: 0
                //1 LSB = FS_XL / (2^5)
           },
           0x58 TAP_CFG2 rw 
           {
                7 interrupts_enable, //Enable basic interrupts (6D/4D, free-fall, wake-up, tap, inactivity). Default value: 0
                //(0: interrupt disabled; 1: interrupt enabled)
                6..5 inact_en, //Enable activity/inactivity (sleep) function. Default value: 00
                //(00: stationary/motion-only interrupts generated, XL and gyro do not change;
                //01: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro does not change;
                //10: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to sleep mode;
                //11: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to power-down mode)
                4..0 tap_ths_y //Y-axis tap recognition threshold. Default value: 0
                //1 LSB = FS_XL / (2^5)
           },
           0x59 TAP_THS_6D rw 
           {
                7 d4d_en, //4D orientation detection enable. Z-axis position detection is disabled.
                //Default value: 0
                //(0: disabled; 1: enabled)
                6..5 sixd_ths, //Threshold for 4D/6D function. Default value: 00
                //SIXD_THS | Threshold value
                //00 | 68 degrees
                //01 | 47 degrees
                //10 | Reserved
                //11 | Reserved
                4..0 tap_ths_z //Z-axis recognition threshold. Default value: 0
                //1 LSB = FS_XL / (2^5)
           },
           0x5A INT_DUR2 rw 
           {
                7..4 dur, //Duration of maximum time gap for double tap recognition. Default: 0000
                //When double tap recognition is enabled, this register expresses the maximum time
                //between two consecutive detected taps to determine a double tap event. The default
                //value of these bits is 0000b which corresponds to 16*ODR_XL time. If the DUR[3:0]
                //bits are set to a different value, 1LSB corresponds to 32*ODR_XL time.
                3..2 quiet, //Expected quiet time after a tap detection. Default value: 00
                //Quiet time is the time after the first detected tap in which there must not be any
                //overthreshold event. The default value of these bits is 00b which corresponds to
                //2*ODR_XL time. If the QUIET[1:0] bits are set to a different value, 1LSB corresponds
                //to 4*ODR_XL time.
                1..0 shock //Maximum duration of overthreshold event. Default value: 00
                //Maximum duration is the maximum time of an overthreshold signal detection to be
                //recognized as a tap event. The default value of these bits is 00b which corresponds
                //to 4*ODR_XL time. If the SHOCK[1:0] bits are set to a different value, 1LSB
                //corresponds to 8*ODR_XL time.
           },
           0x5B WAKE_UP_THS rw
           {
                7 single_double_tap, //Single/double-tap event enable. Default: 0
                //(0: only single-tap event enabled;
                //1: both single and double-tap events enabled)
                6 usr_off_on_wu, //Drives the low-pass filtered data with user offset correction 
                //(instead of high-pass filtered data) to the wakeup function.
                5..0 wk_ths //Threshold for wakeup: 1 LSB weight depends on WAKE_THS_W in
                //WAKE_UP_DUR (5Ch). Default value: 000000
           },
           0x5C WAKE_UP_DUR rw
           {
                7 ff_dur5, //Free fall duration event. Default: 0
                //For the complete configuration of the free-fall duration, refer to FF_DUR[4:0] in
                //FREE_FALL (5Dh) configuration.
                //1 LSB = 1 ODR_time
                6..5 wake_dur, //Wake up duration event. Default: 00
                //1LSB = 1 ODR_time
                4 wake_ths_w, //Weight of 1 LSB of wakeup threshold. Default: 0
                //(0: 1 LSB = FS_XL / (26);
                //1: 1 LSB = FS_XL / (28) )
                3..0 sleep_dur //Duration to go in sleep mode. Default value: 0000 (this corresponds to 16 ODR)
                //1 LSB = 512 ODR
           },
           0x5D FREE_FALL rw
           {
                7..3 ff_dur, //Free-fall duration event. Default: 0
                //For the complete configuration of the free fall duration, refer to FF_DUR5 in
                //WAKE_UP_DUR (5Ch) configuration
                2..0 ff_ths //Free fall threshold setting. Default: 000
                //FF_THS[2:0] | Threshold value
                //000 | 312 mg
                //001 | 438 mg
                //010 | 500 mg
                //011 | Reserved
                //100 | Reserved
                //101 | Reserved
                //110 | Reserved
                //111 | Reserved
           },
           0x5E MD1_CFG rw 
           {
                7 int1_sleep_change, //Routing of activity/inactivity recognition event on INT1. Default: 0
                //(0: routing of activity/inactivity event on INT1 disabled;
                //1: routing of activity/inactivity event on INT1 enabled)
                6 int1_single_tap, //Routing of single-tap recognition event on INT1. Default: 0
                //(0: routing of single-tap event on INT1 disabled;
                //1: routing of single-tap event on INT1 enabled)
                5 int1_wu, //Routing of wakeup event on INT1. Default value: 0
                //(0: routing of wakeup event on INT1 disabled;
                //1: routing of wakeup event on INT1 enabled)
                4 int1_ff, //Routing of free-fall event on INT1. Default value: 0
                //(0: routing of free-fall event on INT1 disabled;
                //1: routing of free-fall event on INT1 enabled)
                3 int1_double_tap, //Routing of tap event on INT1. Default value: 0
                //(0: routing of double-tap event on INT1 disabled;
                //1: routing of double-tap event on INT1 enabled)
                2 int1_6d, //Routing of 6D event on INT1. Default value: 0
                //(0: routing of 6D event on INT1 disabled;
                //1: routing of 6D event on INT1 enabled)
                1 int1_emb_func, //Routing of embedded functions event on INT1. Default value: 0
                //(0: routing of embedded functions event on INT1 disabled;
                //1: routing embedded functions event on INT1 enabled)
                0 int1_shub //Routing of sensor hub communication concluded event on INT1.
                //Default value: 0
                //(0: routing of sensor hub communication concluded event on INT1 disabled;
                //1: routing of sensor hub communication concluded event on INT1 enabled)
           },
           0x5F MD2_CFG rw 
           {
                7 int2_sleep_change, //Routing of activity/inactivity recognition event on INT2. Default: 0
                //(0: routing of activity/inactivity event on INT2 disabled;
                //1: routing of activity/inactivity event on INT2 enabled)
                6 int2_single_tap, //Single-tap recognition routing on INT2. Default: 0
                //(0: routing of single-tap event on INT2 disabled;
                //1: routing of single-tap event on INT2 enabled)
                5 int2_wu, //Routing of wakeup event on INT2. Default value: 0
                //(0: routing of wakeup event on INT2 disabled;
                //1: routing of wake-up event on INT2 enabled)
                4 int2_ff, //Routing of free-fall event on INT2. Default value: 0
                //(0: routing of free-fall event on INT2 disabled;
                //1: routing of free-fall event on INT2 enabled)
                3 int2_double_tap, //Routing of tap event on INT2. Default value: 0
                //(0: routing of double-tap event on INT2 disabled;
                //1: routing of double-tap event on INT2 enabled)
                2 int2_6d, //Routing of 6D event on INT2. Default value: 0
                //(0: routing of 6D event on INT2 disabled;
                //1: routing of 6D event on INT2 enabled)
                1 int2_emb_func, //Routing of embedded functions event on INT2. Default value: 0
                //(0: routing of embedded functions event on INT2 disabled;
                //1: routing embedded functions event on INT2 enabled)
                0 int2_timestamp //Enables routing on INT2 pin of the alert for timestamp overflow within 6.4 ms
           },
           0x62 HAODR_CFG rw
           {
                1..0 haodr_sel //
                    //Selects the ODR set supported when high-accuracy ODR (HAODR)
                    //mode is enabled (see Table 19. Accelerometer and gyroscope ODR
                    //selection in high-accuracy ODR mode). Defauly: 00
           },
           0x63 EMB_FUNC_CFG rw
           {
                7 accel_dualc_batch_from_if, //
                    //When dual-channel mode is enabled, this bit enables batching
                    //the accelerometer channel 2 in FIFO. Default value: 0
                5 emb_func_irq_mask_g_settl, //
                    //Enables / masks execution trigger of the embedded functions when
                    //gyroscope data are settling. Default value: 0
                    //(0: disabled;
                    //1: masks execution trigger of the embedded functions until
                    //gyroscope filter settling ends)
                4 emb_func_irq_mask_accel_settl, //
                    //Enables / masks execution trigger of the embedded functions
                    //when accelerometer data are settling. Default value: 0
                    //(0: disabled;
                    //1: masks execution trigger of the embedded functions until
                    //accelerometer filter settling ends)
                3 emb_func_disable //
                    //Disables execution of the embedded functions. Default value: 0
                    //(0: disabled;
                    //1: embedded functions execution trigger is not generated anymore
                    //and all initialization procedures are forced when this bit is
                    //set back to 0).
           },
           0x64 UI_HANDSHAKE_CTRL rw
           {
                1 ui_shared_ack, //
                    //Primary interface side. This bit acknowledges the handshake.
                    //If the secondary interface is not accessing the shared
                    //registers, this bit is set to 1 by the device and the R/W
                    //operation on the UI_SPI2_SHARED_0 (65h) through
                    //UI_SPI2_SHARED_5 (6Ah) registers is allowed on the primary
                    //interface.
                0 ui_shared_req //
                    //This bit is used by the primary interface master to request
                    //access to the UI_SPI2_SHARED_0 (65h) through UI_SPI2_SHARED_5
                    //(6Ah) registers. When the R/W operation is finished, the master
                    //must reset this bit.
           },
           0x65 UI_SPI2_SHARED_0 rw ui_spi2_shared_0, //UI/SPI2 shared registers
           0x66 UI_SPI2_SHARED_1 rw ui_spi2_shared_1,
           0x67 UI_SPI2_SHARED_2 rw ui_spi2_shared_2,
           0x68 UI_SPI2_SHARED_3 rw ui_spi2_shared_3,
           0x69 UI_SPI2_SHARED_4 rw ui_spi2_shared_4,
           0x6A UI_SPI2_SHARED_5 rw ui_spi2_shared_5,
                //Volatile byte is used as a contact point between the primary and
                //secondary interface host. These shared registers are accessible only
                //by one interface at a time and access is managed through the
                //UI_SHARED_ACK and UI_SHARED_REQ bits of register UI_HANDSHAKE_CTRL
                //(64h) and the SPI2_SHARED_ACK and SPI2_SHARED_REQ bits of register
                //SPI2_HANDSHAKE_CTRL (6Eh).
           0x6B CTRL_EIS rw
           {
                7..6 odr_gyro_eis, //
                    //Enables and selects the ODR of the gyroscope EIS channel.
                    //(00: EIS channel is off (default);
                    //01: 1.92 kHz;
                    //10: 960 Hz;
                    //11: reserved)
                4 lpg_gyro_eis_bw, //
                    //Gyroscope digital LPF_EIS filter bandwidth selection.
                    //Refer to Table 191 (datasheet).
                3 gyro_eis_on_gyro_ois_out, //
                    //Enables routing gyroscope EIS output to OIS from UI output
                    //addresses (2Eh – 33h). When this bit is set to 1, the
                    //gyroscope OIS data cannot be read from primary interface.
                    //Default value: 0
                2..0 fs_gyro_eis //
                    //Gyroscope full-scale selection for EIS channel. If the
                    //FS_G_[3:0] bits in CTRL6 (15h) are equal to 1100 (±4000 dps)
                    //FS_G_EIS_[2:0] must be set to "100" in order to have
                    //±4000 dpsfull scale on both UI and EIS channels. If the
                    //FS_G_3 bit in register CTRL6 (15h) is equal to 0, the EIS
                    //channel full scale can be selected as follows:
                    //(000: ±125 dps (default);
                    //001: ±250 dps;
                    //010: ±500 dps;
                    //011: ±1000 dps;
                    //100: ±2000 dps;
                    //others: reserved)
           },
           0x6F UI_INT_OIS rw
               //OIS interrupt configuration register
               //The primary interface can write to this register when the
               //OIS_CTRL_FROM_UI bit in the FUNC_CFG_ACCESS (01h) register is equal
               //to 1 (primary IF full-control mode); this register is read-only
               //when the OIS_CTRL_FROM_UI bit is equal to 0 (SPI2 full-control mode)
               //and shows the content of the SPI2_INT_OIS (6Fh) register.
           {
                7 int2_drdy_ois, //
                    //Enables OIS chain DRDY on INT2 pin from the UI interface.
                    //This setting has priority over all other INT2 settings.
                6 drdy_mask_ois, //
                    //Enables / masks OIS data available. Default value: 0
                    //(0: disabled;
                    //1: masks OIS DRDY signals (both accelerometer and gyroscope)
                    //until filter settling ends (accelerometer and gyroscope
                    //independently masked))
                4 st_ois_clampdis //
                    //Disables OIS chain clamp during self-test. Default value: 0
                    //(0: All OIS chain outputs = 8000h during self-test;
                    //1: OIS chain self-test outputs)
           },
           0x70 UI_CTRL1_OIS r
               //OIS configuration register
               //The primary interface can write this register when the
               //OIS_CTRL_FROM_UI bit in the FUNC_CFG_ACCESS (01h) register is equal
               //to 1 (primary IF full-control mode); this register is read-only
               //when the OIS_CTRL_FROM_UI bit is equal to 0 (SPI2 full-control mode)
               //and shows the content of the SPI2_CTRL1_OIS (70h) register.
           {
                5 sim_ois, //SPI2 3- or 4-wire interface. Default value: 0
                2 ois_accel_en, //Enables accelerometer OIS chain. Default value: 0
                1 ois_gyro_en, //Enables gyroscope OIS chain. Default value: 0
                0 spi2_read_en //In primary IF full-control mode, enables auxiliary
                    //SPI for reading OIS data in registers SPI2_OUTX_L_G_OIS (22h)
                    //and SPI2_OUTX_H_G_OIS (23h) through Section 11.9
                    //SPI2_OUTZ_L_A_OIS (2Ch) and SPI2_OUTZ_H_A_OIS (2Dh).
                    //Default value: 0
           },
           0x71 UI_CTRL2_OIS r //read-only in SPI2 mode b/c redundant, but I'm not
               //sure which mode we're running this in.
           {
                4..3 lpf1_gyro_ois_bw, //Gyroscope OIS bandwidth selection.
                    //Value     Cutoff(Hz)      Phase @20Hz(�)
                    //00        293             -7.1
                    //01        217             -9.1
                    //10        158             -11.9
                    //11        476             -5.1
                2..0 fs_gyro_ois //Gyro OIS full-scale selection.
                    //000: ±125 dps
                    //001: ±250 dps
                    //010: ±500 dps
                    //011: ±1000 dps
                    //100: ±2000 dps
                    //Others reserved
           },
           0x72 UI_CTRL3_OIS r
           {
                5..3 lpf_accel_ois_bw, //Selects accel OIS channel bandwidth,
                    //Default value 0.
                    //Value     Typ. Overall BW (Hz)    Typ. Overall Phase @20Hz(�)
                    //000       749                     -3.41    
                    //001       539                     -4.04
                    //010       342                     -5.31
                    //011       162                     -9.08
                    //100       78.5                    -16.4
                    //101       38.6                    -29.6
                    //110       19.3                    -28.8
                    //111       9.8                     -29.1
                    //Default 0.
                1..0 fs_accel_ois //Selects accel OIS channel full-scale.
                    //00: ±2 g default)
                    //01: ±4g
                    //10: ±8 
                    //11: ±16 
           },
           0x73 X_OFS_USR rw usr_offset_x,
           0x74 Y_OFS_USR rw usr_offset_y,
           0x75 Z_OFS_USR rw usr_offset_z,
           0x78 FIFO_DATA_OUT_TAG r
           {
                7..3 tag_sensor, //FIFO tag. Identifies sensor used for FIFO data.
                    //Value     Sensor
                    //0x00      FIFO empty
                    //0x01      Gyroscope NC
                    //0x02      Accelerometer NC
                    //0x03      Temperature
                    //0x04      Timestamp
                    //0x05      CFG_Change
                    //0x06      Accelerometer NC_T_2
                    //0x07      Accelerometer NC_T_1
                    //0x08      Accelerometer 2xC
                    //0x09      Accelerometer 3xC
                    //0x0A      Gyroscope NC_T_2
                    //0x0B      Gyroscope NC_T_1
                    //0x0C      Gyroscope 2xC
                    //0x0D      Gyroscope 3xC
                    //0x0E      Sensor hub slave 0
                    //0x0F      Sensor hub slave 1
                    //0x10      Sensor hub slave 2
                    //0x11      Sensor hub slave 3
                    //0x12      Step counter
                    //0x13      SFLP game rotation vector
                    //0x16      SFLP gyroscope bias
                    //0x17      SFLP gravity vector
                    //0x19      Sensor hub nack
                    //0x1D      Accelerometer dualC
                    //0x1E      Enhanced EIS gyroscope
                    //Others reserved
                2..1 tag_counter //2-bit counter which identifies sensor time slot
           },

        }
    }
}


pub struct Lsm6dsv<S: SpiHandle> {
    spi: S
}

impl <S: SpiHandle> Lsm6dsv<S> {
    type Error = <S::Bus as ErrorType>::Error;
    pub fn new(spi: S) -> Self {
        Self {
            spi
        }
    }
    pub fn setup(
        &mut self
    ) -> Result<(),Self::Error> {
        Ok(()) //TODO: Use this function to perform initial setup of the IMU. Example: opening register access,
               //configuring SPI bus, setting verbosity/accuracy mode.
               //KNOWN NEED TO SET: 
    }

    
    
}


impl <S: SpiHandle> ReadLsm6dsv for Lsm6dsv<S> {
    type Error = <S::Bus as ErrorType>::Error;

    async fn read_contiguous_regs(
        &mut self,
        addr: impl ReadableAddr,
        out: &mut [u8]
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;

        // set rw bit
        // write = 1, read = 0
        let addr: u8 = addr.as_addr() | 0b1000_0000;
        
        bus.write(&[addr]).await?;
        bus.transfer_in_place(out).await?;
        Ok(())
    }
}

impl <S: SpiHandle> WriteLsm6dsv for Lsm6dsv<S> {
    type Error = <S::Bus as ErrorType>::Error;

    async fn write_contiguous_regs(
        &mut self,
        addr: impl WritableAddr,
        values: &[u8]
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;

        let addr: u8 = addr.as_addr() & 0b0111_1111;

        bus.write(&[addr.as_addr()]).await?;
        bus.write(values).await?;

        Ok(())
    }

}
