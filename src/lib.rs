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
           },
           0x02 PIN_CTRL rw //Pull-up control register for SDO, OCS-Aux, and SDO_Aux pins
           {
                7 ois_pullup_L, //1 disables pull-up on OCS_Aux and SDO_Aux pins, 0 enables, default 0
                6 sdo_pullup, //Enables pull-up on SDO pin, default 0
           },
           0x07 FIFO_CTRL1 rw fifo_wtm[1..7], //FIFO watermark threshold, 1 LSB = TAG (1 byte) + 1 sensor (6 bytes) in FIFO, flag rises when #bytes in FIFO > threshold
           0x08 FIFO_CTRL2 rw
           {
                7 stop_fifo_on_wtm, //Limit FIFO depth to threshold level
                6 fifo_compr, //Enable compression
                4 fifo_batch_odrchg, //Batch ODR nCHANGE sensor in FIFO
                2..1 fifo_uncompr_rate, //Configures the rate of written uncompressed data (default 0). 0: do not force uncompressed data writing, 1: uncompressed data every 8 batch, 2: every 16 batch, 3: every 32 batch
                0 fifo_wtm[0], //Enable FSM-triggered batching of channel 2 when available, 0 disabled, 1 enabled, default 0
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
                7    pulsed_drdy, //Enables pulsed data-ready mode.
                    //0: data-ready latched mode (returns to 0 only after an interface reading) (default)
                    //1: data-ready pulsed mode (the data ready pulses are 75 �s long)
                6..5 trig_ctr_bdr, //Select the trigger for the internal counter of batch events for the accel, gyro, and EIS gyro.
                    //00: accel batch event.
                    //01: gyro batch event.
                    //10-11: gyro EIS batch event.
                2..0 batch_counter_thresh[0..2] //Sets the threshold for the internal
                    //counter of batch events. When this counter reaches the
                    //threshold, the counter is reset and counter_bdr_reached is
                    //set to 1.
           },
           0x0C COUNTER_BDR_REG2 rw batch_counter_thresh[3..10],
           0x0D INT1_CTRL rw //INT1 pin control register.
               //Output is the OR combination of all selected here and in MD1_CFG.
               //All bits default 0.
           {
                7 den_drdy_flag, //Sends DEN_DRDY (DEN stamped on the sensor data flag) to the INT1 pin
                6 int1_cnt_bdr, //Enables COUNTER_BDR_IA interrupt on INT1 pin.
                5 int1_fifo_full, //Enables FIFO full flag interrupt on INT1 pin.
                4 int1_fifo_over, //Enables FIFO overrun interrupt on INT1 pin.
                3 int1_fifo_thresh, //Enables FIFO threshold interrupt on INT1 pin.
                2 int1_boot, //Enables boot status on the INT1 pin
                1 int1_gyro_rdy, //Enables gyro data-ready interrupt on INT1 pin.
                0 int1_accel_rdy //Enables accel data-ready interrupt on INT1 pin.
           },
           0x0E INT2_CTRL rw //INT2 pin control register.
               //Output is the OR combination of all selected here and in MD2_CFG.
               //All defaults 0.
           {
                6 int2_cnt_bdr, //Enables COUNTER_BDR_IA interrupt on INT2 pin.
                5 int2_fifo_full, //Enables FIFO full interrupt on INT2 pin.
                4 int2_fifo_over, //Enables FIFO overrun interrupt on INT2 pin.
                3 int2_fifo_thresh, //Enables FIFO threshold interrupt on INT2 pin.
                2 int2_temp_rdy, //Enables temperature data-ready interrupt on INT2 pin.
                1 int2_gyro_rdy, //Enables gyro data-ready interrupt on INT2 pin.
                0 int2_accel_rdy //Enables accel data-ready interrupt on INT2 pin.
           },
           0x0F WHO_AM_I r whoami, //whoami value. Read-only, fixed at 0x6C.
           0x10 CTRL1_XL rw
           {
                7..4 accel_odr_mode, //Accel ODR selection.
                    //When XL_HM_MODE=1 in CTRL6_C:
                        //0000: Power-down
                        //1011: 1.6Hz (low power only)
                        //0001: 12.5Hz (low power)
                        //0010: 26Hz (low power)
                        //0011: 52Hz (low power)
                        //0100: 104Hz (normal mode)
                        //0101: 208Hz (normal mode)
                        //0110: 416Hz (high performance)
                        //0111: 833Hz (high performance)
                        //1000: 1.66kHz (high performance)
                        //1001: 3.33kHz (high performance)
                        //1010: 6.66kHz (high performance)
                        //11xx: reserved
                    //Wnen XL_HM_MODE=0 in CTRL6_C:
                        //0000: Power-down
                        //1011: 12.5Hz (high performance)
                        //0001: 12.5Hz (high performance)
                        //0010: 26Hz (high performance)
                        //0011: 52Hz (high performance)
                        //0100: 104Hz (high performance)
                        //0101: 208Hz (high performance)
                        //0110: 416Hz (high performance)
                        //0111: 833Hz (high performance)
                        //1000: 1.66kHz (high performance)
                        //1001: 3.33kHz (high performance)
                        //1010: 6.66kHz (high performance)
                        //11xx: Reserved
                3..2 accel_fs, //Accelerometer full-scale selection
                    //When XL_FS_MODE=0 in CTRL8_XL
                        //00: ±2g default)
                        //01: ±16
                        //10: ±4
                        //11: ±8
                    //When XL_FS_MODE=1 in CTRL8_XL
                        //00: ±2g default)
                        //01: �2g
                        //10: ±4
                        //11: ±8
                1 accel_lpf2, //Accelerometer high-resolution selection
                    //0: output from first stage digital filtering selected (default)
                    //1: output from LPF2 second filtering stage selected
           },
           0x11 CTRL2_G rw
           {
                7..4 gyro_odr, //Accel output data rate selection.
                    //When gyro_hm_mode=1:
                        //0000: power down (default)
                        //0001: 12.5Hz (low power)
                        //0010: 26Hz (low power)
                        //0011: 52Hz (low power)
                        //0100: 104Hz (normal)
                        //0101: 208Hz (normal)
                        //0110: 416Hz (high performance)
                        //0111: 833Hz (high performance)
                        //1000: 1.66kHz (high performance)
                        //1001: 3.33kHz (high performance)
                        //1010: 6.66kHz (high performance)
                        //others: reserved
                    //When gyro_hm_mode=0:
                        //0000: power down (high performance)
                        //0001: 12.5Hz (high performance)
                        //0010: 26Hz (high performance)
                        //0011: 52Hz (high performance)
                        //0100: 104Hz (high performance)
                        //0101: 208Hz (high performance)
                        //0110: 416Hz (high performance)
                        //0111: 833Hz (high performance)
                        //1000: 1.66kHz (high performance)
                        //1001: 3.33kHz (high performance)
                        //1010: 6.66kHz (high performance)
                        //others: reserved
                3..2 gyro_fs_select, //Gyroscope UI chain full-scale selection
                    //00: ±250 dp
                    //01: ±500 dp
                    //10: ±1000 dp
                    //11: ±2000 dp
                1 gyro_125dps, //Selects gyro UI chain full-scale ±125 dp
                    //0: FS selected through gyro_fs_select
                    //1: FS set to ±125 dp
           },
           0x12 CTRL3_C rw
           {
                7 reboot_mem, //Reboots memory content. Default 0, auto-cleared.
                6 block_update, //Block data update. Default value: 0
                    //0: continuous update
                    //1: output registers are not updated until MSB and LSB have been read
                5 interrupts_active_low, //Whether or not interrupts are active low, default 0
                4 pp_od, //Interrupt activation level. Default value: 0
                    //0: interrupt output pins active high
                    //1: interrupt output pins active low
                3 spi_mode_select, //Push-pull/open-drain selection on the INT1 and INT2 pins. Default value: 0
                    //0: push-pull mode
                    //1: open-drain mode
                2 inc_addrs, //Register address automatically incremented during a multiple byte access with a serial interface (I²C or SPI). Default value: 1
                    //0: disabled
                    //1: enabled
                0 sw_reset, //Set 1 to reset software, default 0, auto-cleared.
           },
           0x13 CTRL4 rw
           {
                6 gyro_sleep, //Enables gyroscope sleep mode. Default value: 0
                5 int2_on_int1, //Enables all interrupt signals available on the INT1 pin. Default value: 0
                    //0: interrupt signals divided between the INT1 and INT2 pins
                    //1: all interrupt signals in logic or on the INT1 pin
                3 drdy_mask, //Enables data available
                    //0: disabled
                    //1: mask DRDY on pin (both accelerometer and gyroscope) until filter settling ends (accelerometer and gyroscope independently masked)
                2 disable_i2c, //Disables I²C interface. Default value: 
                    //0: SPI, I²C, and MIPI I3CSM interfaces enabled(default)
                    //1: I²C interface disable
                1 gyro_lpf1, //Enables gyroscope digital LPF1 if the auxiliary SPI is disabled; the bandwidth can be selected through FTYPE[2:0] in CTRL6_C (15h).
           },
           0x14 CTRL5 rw
           {
                7 accel_ulp, //Enables accelerometer ultralow-power mode. Default value: 0
                6..5 read_wraparound, //Circular burst mode (wraparound) read from the output registers. Default value: 00
                    //00: no wraparound
                    //01: accelerometer only
                    //10: gyroscope only
                    //11: gyroscope + accelerometer
                3..2 gyro_selftest, //Enables angular rate sensor self-test. Default value: 00
                    //00: self-test disabled
                    //01: Positive sign self-test
                    //10: Reserved
                    //11: Negative sign self-test
                1..0 accel_selftest, //Enables linear acceleration sensor self-test. Default value: 00
                    //00: self-test disabled
                    //01: Positive sign self-test
                    //10: Negative sign self-test
                    //11: Reserved
           },
           0x15 CTRL6_C rw
           {
                7..5 den_trigger_mode, //Trigger mode selection for DEN
                    //100: Edge-sensitive trigger mode is selected.
                    //010: Level-sensitive trigger mode is selected.
                    //011: Level-sensitive latched mode is selected.
                    //110: Level-sensitive FIFO enable mode is selected.
                4 accel_hp_dis, //Disables high-performance operating mode for the accelerometer. Default value: 0
                3 accel_offset_weight, //Weight of the accelerometer user offset bits of registers X_OFS_USR (73h), Y_OFS_USR (74h), Z_OFS_USR (75h)
                    //0: 2^-10 g/LSB
                    //1: 2^-6 g/LSB
                2..0 gyro_lpf_bw, //Gyroscope low-pass filter (LPF1) bandwidth selection.
                    //Table didn't fit, see datasheet
           },
           0x16 CTRL7_G rw
           {
                7 gyro_hp_dis, //Disables high-performance operating mode for gyroscope. Default value: 0
                6 gyro_hpf_enable, //Enables gyroscope digital high-pass filter. The filter is enabled only if the gyroscope is in HP mode. Default value: 0
                5..4 gyro_hpf_cutoff, //Gyroscope digital HP filter cutoff selection. Default: 00
                    //00: 16 mHz
                    //01: 65 mHz
                    //10: 260 mHz
                    //11: 1.04 Hz
                2 ois_on_primary, //Selects how to enable and disable the OIS chain, after first configuration and enabling through SPI2.
                    //0: OIS chain is enabled/disabled with SPI2 interface
                    //1: OIS chain is enabled/disabled with primary interface
                1 accel_offset_enable, //Enables accelerometer user offset correction block; it is valid for the low-pass path - see Figure 17. Accelerometer composite filter. Default value: 0
                    //0: accelerometer user offset correction block bypassed
                    //1: accelerometer user offset correction block enabled
                0 ois_enable, //Enables/disables the OIS chain from the primary interface when ois_on_primary is 1.
           },
           0x17 CTRL8_XL rw
           {
                7..5 accel_hpf_cutoff, //Accelerometer LPF2 and HP filter configuration and cutoff setting.
                    //Low Pass, accel_lpf2=0: ODR/2
                    //Low Pass, accel_lpf2=1:
                        //000: ODR/4
                        //001: ODR/10
                        //010: ODR/20
                        //011: ODR/45
                        //100: ODR/100
                        //101: ODR/200
                        //110: ODR/400
                        //111: ODR/800
                    //High Pass:
                        //000: ODR/4
                        //001: ODR/10
                        //010: ODR/20
                        //011: ODR/45
                        //100: ODR/100
                        //101: ODR/200
                        //110: ODR/400
                        //111: ODR/800
                4 accel_hp_refmode, //Enables accelerometer high-pass filter reference mode (valid for high-pass path - HP_SLOPE_XL_EN bit must be 1). Default value: 0
                3 accel_fastsettle, //Enables accelerometer LPF2 and HPF fast-settling mode. The filter sets the second samples after writing this bit. Active only during device exit from power- down mode. Default value: 0
                2 accel_slope_hp, //Accelerometer slope filter / high-pass filter selection
                1 accel_new_fs, //Accelerometer full-scale management between UI chain and OIS chain
                    //0: old full-scale mode. When XL UI is on, the full scale is the same between UI/OIS and is chosen by the UI CTRL registers; when XL UI is in PD, the OIS can choose the FS.
                    //1: new full-scale mode. Full scales are independent between the UI/OIS chain but both bound to ±8g
                0 lpf2_on_6d, //LPF2 on 6D function selection.
                    //0: ODR/2 low-pass filtered data sent to 6D interrupt function
                    //1: LPF2 output data sent to 6D interrupt function
           },
           0x18 CTRL9_XL rw
           {
                7 den_x, //DEN value stored in LSB of X-axis. Default value: 1
                6 den_y, //DEN value stored in LSB of Y-axis. Default value: 1
                5 den_z, //DEN value stored in LSB of Z-axis. Default value: 1
                4 den_stamp, //DEN stamping sensor selection. Default value: 0
                    //0: DEN pin info stamped in the gyroscope axis selected by bits [7:5]
                    //1: DEN pin info stamped in the accelerometer axis selected by bits [7:5]
                3 den_accel_enable, //Extends DEN functionality to accelerometer sensor. Default value: 0
                2 den_high, //DEN active level configuration. Default value: 0
                    //0: active low
                    //1: active high
                1 i3c_disable, //Disables MIPI I3CSM communication protocol, default 0
           },
           0x19 CTRL10_C rw
           {
               5 timestamp_enable, //Enables timestamp counter. Default value: 0
                    //The counter is readable in TIMESTAMP0 (40h), TIMESTAMP1 (41h), TIMESTAMP2 (42h), and TIMESTAMP3 (43h).
           },
           0x1A ALL_INT_SRC r //Source register for all interrupts
           {
                7 timestamp_endct, //Alerts timestamp overflow within 6.4 ms
                5 sleep_change_ia, //Detects change event in activity/inactivity status. Default value: 0
                4 orientation_ia, //Interrupt active for change in position of portrait, landscape, face-up, face-down. Default value: 0
                3 double_tap, //Double-tap event status. Default value: 0
                2 single_tap, //Single-tap event status. Default value: 0
                1 wake_ia, //Wake-up event status. Default value: 0
                0 freefall_ia, //Free-fall event status. Default value: 0
           },
           0x1B WAKE_UP_SRC r
           {
                4 sleep_state, //Sleep status bit. Default value: 0
                2 wake_x, //Wake-up event detection status on X-axis. Default value: 0
                1 wake_y, //Wake-up event detection status on Y-axis. Default value: 0
                0 wake_z, //Wake-up event detection status on Z-axis. Default value: 0
           },
           0x1C TAP_SRC r
           {
                6 tap_ia, //Tap event detection status. Default: 0
                3 tap_sign, //Sign of acceleration detected by tap event. Default: 0
                2 tap_x, //Tap event detection status on X-axis. Default value: 0
                1 tap_y, //Tap event detection status on Y-axis. Default value: 0
                0 tap_z, //Tap event detection status on Z-axis. Default value: 0
           },
           0x1D D6D_SRC r 
           {
                7 den_drdy, //DEN data-ready signal. It is set high when the data output is related to the data coming from a DEN active condition.
                5 high_z, //Z-axis high event (over threshold). Default value: 0
                4 low_z, //Z-axis low event (under threshold). Default value: 0
                3 high_y, //Y-axis high event (over threshold). Default value: 0
                2 low_y, //Y-axis low event (under threshold). Default value: 0
                1 high_x, //X-axis high event (over threshold). Default value: 0
                0 low_x, //X-axis low event (under threshold). Default value: 0
           },
           0x1E STATUS_REG r
           {
                2 temp_da, //temp data available
                1 gyro_da, //gyro data available
                0 accel_da //accel data available
           },
           ///Temp data output register
           0x20 OUT_TEMP_L r temp_data[0..7], 
           0x21 OUT_TEMP_H r temp_data[8..15],
           0x22 OUTX_L_G r gyro_pitch_rate[0..7], //Gyro pitch axis angular rate
           0x23 OUTX_H_G r gyro_pitch_rate[8..15],
           0x24 OUTY_L_G r gyro_roll_rate[0..7], //Gyro roll axis angular rate
           0x25 OUTY_H_G r gyro_roll_rate[8..15],
           0x26 OUTZ_L_G r gyro_yaw_rate[0..7], //Gyro yaw axis angular rate
           0x27 OUTZ_H_G r gyro_yaw_rate[8..15],
           0x28 OUTX_L_A r accel_x[0..7], //Accel x output
           0x29 OUTX_H_A r accel_x[8..15],
           0x2A OUTY_L_A r accel_y[0..7], //Accel y output
           0x2B OUTY_H_A r accel_y[8..15],
           0x2C OUTZ_L_A r accel_z[0..7], //Accel z output
           0x2D OUTZ_H_A r accel_z[8..15],
           0x2E UI_OUTX_L_G_OIS_EIS r x_ois_eis[0..7], //x axis OIS/EIS
           0x2F UI_OUTX_H_G_OIS_EIS r x_ois_eis[8..15],
           0x30 UI_OUTY_L_G_OIS_EIS r y_ois_eis[0..7], //y axis OIS/EIS
           0x31 UI_OUTY_H_G_OIS_EIS r y_ois_eis[8..15],
           0x32 UI_OUTZ_L_G_OIS_EIS r z_ois_eis[0..7], //z axis OIS/EIS
           0x33 UI_OUTZ_H_G_OIS_EIS r z_ois_eis[8..15],
           0x34 UI_OUTX_L_A_OIS_DualC r x_ois_dc[0..7], //x axis OIS/DualC
           0x35 UI_OUTX_H_A_OIS_DualC r x_ois_dc[8..15],
           0x36 UI_OUTY_L_A_OIS_DualC r y_ois_dc[0..7], //y axis OIS/DualC
           0x37 UI_OUTY_H_A_OIS_DualC r y_ois_dc[8..15],
           0x38 UI_OUTZ_L_A_OIS_DualC r z_ois_dc[0..7], //z axis OIS/DualC
           0x39 UI_OUTZ_H_A_OIS_DualC r z_ois_dc[8..15],
           0x40 TIMESTAMP0 r timestamp[0..7], //Timestamp output. 1LSB = 21.75u
           0x41 TIMESTAMP1 r timestamp[8..15],
           0x42 TIMESTAMP2 r timestamp[16..23],
           0x43 TIMESTAMP3 r timestamp[24..31],
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
           0x62 I3C_BUS_AVB rw
           {
                4..3 i3c_bus_avb_sel,
                    //These bits are used to select the bus available time when MIPI I3CSM IBI is used.
                    //Default value: 00
                    //(00: bus available time equal to 50 µsec (default);
                    //01: bus available time equal to 2 µsec;
                    //10: bus available time equal to 1 msec;
                    //11: bus available time equal to 25 msec
                0 pd_dis_int1
                    //This bit allows disabling the INT1 pull-down.
                    //PD_DIS_INT1
                    //INT1
                    //(0: Pull-down on INT1 enabled (pull-down is effectively connected only when no interrupts are routed 
                    //to the INT1 pin or when the MIPI I3CSM dynamic address is assigned);
                    //1: Pull-down on INT1 disabled (pull-down not connected)
           },
           0x63 INTERNAL_FREQ_FINE rw
           {
                7..0 freq_fine
                //Difference in percentage of the effective ODR (and timestamp rate) with respect to the typical. Step: 0.15%. 
                //8-bit format, two's complement.
           },
           
           /*0x6F INT_OIS rw
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
                6 lvl2_ois, 
                    //Enables level-sensitive latched mode on the OIS chain. Default value: 0
                5 den_lh_ois,
                    //Indicates polarity of DEN signal on OIS chain
                    //  (0: DEN pin is active-low;
                    //  1: DEN pin is active-high)
                1..0 st_accel_ois
                    // Selects accelerometer self-test – active only if the accelerometer OIS chain is enabled. Default value: 00
                    // (00: normal mode;
                    // 01: positive sign self-test;
                    // 10: negative sign self-test;
                    // 11: not allowed
           },*/
           0x70 CTRL1_OIS rw
            {   
                6 lvl1_ois,
                    //Enables OIS data level-sensitive trigger
                5 sim_ois,
                    //SPI2 3- or 4-wire interface. Default value: 0
                    //(0: 4-wire SPI2;
                    //1: 3-wire SPI2
                4 mode4_en,
                    // Enables accelerometer OIS chain. OIS outputs are available through SPI2 in registers 28h-2Dh.
                    // Note: OIS_EN_SPI2 must be enabled (that is, set to 1) to enable also the accelerometer OIS chain.
                3..2 fs_g_ois,
                    //Selects gyroscope OIS chain full-scale
                    // (00: ±250 dps;
                    // 01: ±500 dps;
                    // 10: ±1000 dps;
                    // 11: ±2000 dps)
                1 fs_125_ois,
                    //Selects gyroscope OIS chain full-scale ±125 dps
                    //(0: FS selected through bits FS[1:0]_OIS_G;
                    //1: ±125 dps)
                0 ois_en_spi2
                    // Enables OIS chain data processing for gyroscope in mode 3 and mode 4 (Mode4_EN = 1) and 
                    // accelerometer data in mode 4 (Mode4_EN = 1).
                    //  When the OIS chain is enabled, the OIS outputs are available through the SPI2 in registers OUTX_L_G 
                    // (22h) and OUTX_H_G (23h) through OUTZ_L_A (2Ch) and OUTZ_H_A (2Dh) and STATUS_REG (1Eh) / 
                    // STATUS_SPIAux (1Eh), and LPF1 is dedicated to this chain.  
           },
           0x71 CTRL2_OIS r
           {
            5..4 hpm_ois,
                //Selects gyroscope OIS chain digital high-pass filter cutoff. Default value: 00
                //  (00: 16 mHz;
                //  HPM[1:0]_OIS
                //  01: 65 mHz;
                //  10: 260 mHz;
                //  11: 1.04 Hz) 
            2..1 ftype_ois,
                //Selects gyroscope digital LPF1 filter bandwidth. Table 151 shows cutoff and phase values obtained with all 
                //configurations.
            0 hp_en_ois
                // Enables gyroscope OIS chain digital high-pass filter
                // Gyroscope OIS chain digital LPF1 filter bandwidth selection
                // FTYPE_[1:0]_OIS
                // 00
                // 01
                // 10
                // 11
                // Cutoff [Hz]  Phase @ 20 Hz [°]
                // 335.5        -6.69
                // 232.0        -8.78
                // 171.1        -11.18
                // 609.0        -4.91
           },
           0x72 CTRL3_OIS r
           {
                7..6 fs_xl_ois,
                    //Accelerometer OIS channel full-scale selection
                    // FS[1:0]_XL_OIS   XL_FS_MODE = 0                                              XL_FS_MODE = 1
                    //                  XL UI ON                                   XL UI PD         -
                    // 00 (default)     Full-scale selected from user interface    ±2 g             ±2 g
                    // 01                                                          ±16 g            ±2 g
                    // 10                                                          ±4 g             ±4 g
                    // 11                                                          ±8 g             ±8 g
                5..3 filter_xl_conf_ois,
                    //Accelerometer OIS channel bandwidth and phase
                    // FILTER_XL_CONF_OIS[2:0] Typ. overall bandwidth [Hz]  Typ. overall phase [°]
                    // 000                     289                          -5.72 @ 20 Hz
                    // 001                     258                          -6.80 @ 20 Hz
                    // 010                     120                          -13.2 @ 20 Hz
                    // 011                     65.1                         -21.5 @ 20 Hz
                    // 100                     33.2                         -19.1 @ 10 Hz
                    // 101                     16.6                         -33.5 @ 10 Hz
                    // 110                     8.30                         -26.7 @ 4 Hz
                    // 111                     4.14                         -26.2 @ 2 Hz
                2..1 st1_ois,
                    // Selects gyroscope OIS chain self-test. Default value: 00
                    // Table 156 lists the output variation when the self-test is enabled and ST_OIS_CLAMPDIS = 1.
                    // (00: Normal mode;
                    // 01: Positive sign self-test;
                    // 10: Normal mode;
                    // 11: Negative sign self-test)
                0 st_ois_clampdis
                    // Disables OIS chain clamp
                    //  (0: All OIS chain outputs = 8000h during self-test;
                    //  1: OIS chain self-test outputs as shown in Table 156.
                    //  Table 156. Self-test nominal output variation
                    //  Full scale       Output variation [dps]
                    //  ±2000            ±400
                    //  ±1000            ±200
                    //  ±500             ±100
                    //  ±250             ±50
                    //  ±125             ±25
           },
           0x73 X_OFS_USR rw usr_offset_x,
           0x74 Y_OFS_USR rw usr_offset_y,
           0x75 Z_OFS_USR rw usr_offset_z,
           0x78 FIFO_DATA_OUT_TAG r
           {
                7..3 tag_sensor, //FIFO tag. Identifies sensor used for FIFO data.
                    //Value     Sensor
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
                    //0x19      Sensor hub nack
                2..1 tag_counter, 
                    //2-bit counter which identifies sensor time slot
                0 tag_parity 
                    //Parity check of TAG content
           },
           0x79 FIFO_DATA_OUT_X_L r,
                    //x axis ouptut
           0x7A FIFO_DATA_OUT_X_H r,
                    //x axis ouptut
           0x7B FIFO_DATA_OUT_Y_L r,
                    //y axis output
           0x7C FIFO_DATA_OUT_Y_H r,
                    //y axis output
           0x7D FIFO_DATA_OUT_Z_L r,
                    //z axis output
           0x7E FIFO_DATA_OUT_Z_H r
                    //z axis output
        }
    }
}


pub struct Lsm6dso<S: SpiHandle> {
    spi: S
}

impl <S: SpiHandle> Lsm6dso<S> {
    /*
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

    }*/
    
    pub fn new(spi: S) -> Self {
        Self {
            spi
        }
    }

    pub async fn read(&mut self) -> Result<u16, <S::Bus as ErrorType>::Error> {
        let accelx: u16 = self.accel_x().await?;
        Ok(accelx)
    }

    pub async fn raw_accel(&mut self) -> Result<(i16, i16, i16), <S::Bus as ErrorType>::Error> {
        Ok(unsafe {
            let accel_x: i16 = mem::transmute(self.accel_x().await?);
            let accel_y: i16 = mem::transmute(self.accel_y().await?);
            let accel_z: i16 = mem::transmute(self.accel_z().await?);

            (accel_x, accel_y, accel_z)
        })
    }

    pub async fn accel(&mut self) -> Result<(i32, i32, i32), <S::Bus as ErrorType>::Error> {
        let (raw_x, raw_y, raw_z) = self.raw_accel().await?;
        let fs = 32; //sensitivity mode TODO: read from chip
        let scalar: i32 = 122 * fs/4;
        let accel_x: i32 = scalar * (raw_x as i32);
        let accel_y: i32 = scalar * (raw_y as i32);
        let accel_z: i32 = scalar * (raw_z as i32);

        Ok((accel_x, accel_y, accel_z))
    }
    
}


impl <S: SpiHandle> ReadLsm6dso for Lsm6dso<S> {
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

impl <S: SpiHandle> WriteLsm6dso for Lsm6dso<S> {
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
