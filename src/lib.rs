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
                2 int1_boot //Enables boot status on the INT1 pin
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
           }
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
           0x40 TIMESTAMP0 r timestamp[0..7], //Timestamp output. 1LSB = 21.75u
           0x41 TIMESTAMP1 r timestamp[8..15],
           0x42 TIMESTAMP2 r timestamp[16..23],
           0x43 TIMESTAMP3 r timestamp[24..31],
           0x44 UI_STATUS_REG_OIS r
           {
                2 gyro_settling, //High when gyro is settling
                1 gda_ois, //Is gyro OIS data available
                0 accel_da_ois //Is accel OIS data available
           },
           0x45 WAKE_UP_SRC r
           {
                6 sleep_change_ia, //Detects change event in activity/inactivity
                5 wakeup_ff_ia, //Detects free-fall event
                4 sleep_state, //Sleep status bit
                3 wu_detect, //wake-up detection status
                2 x_wu, //wake-up on x-axis
                1 y_wu, //wake-up on y-axis
                0 z_wu //wake-up on z-axis
           },
           0x46 TAP_SRC r
           {
                6 tap_src_ia, //tap event detection status
                5 single_tap, //single-tap event status
                4 double_tap, //double-tap event status
                3 tap_sign, //sign of acceleration detected by tap event
                2 x_tap, //tap event on x-axis
                1 y_tap, //tap event on y-axis
                0 z_tap, //tap event on z-axis
           },
           0x47 D6D_SRC r
           {
                6 d6d_src_ia, //orientation change interrupt status
                5 z_high, //z-axis high event
                4 z_low, //z-axis low event
                3 y_high, //y-axis high event
                2 y_low, //y-axis low event
                1 x_high, //x-axis high event
                0 x_low //x-axis low event
           },
           0x49 EMB_FUNC_STATUS_MAINPAGE r
           {
                7 is_fsm_lc, //Interrupt status bit for FSM long counter timeout
                5 is_sigmot, //Interrupt status bit for significant motion detection
                4 is_tilt, //Interrupt status bit for tilt detection
                3 is_step_det //Interrupt status bit for step detection
           },
           0x4A FSM_STATUS_MAINPAGE r
           {
                7 is_fsm8, //FSM8 interrupt status
                6 is_fsm7, //FSM7 interrupt status
                5 is_fsm6, //FSM6 interrupt status
                4 is_fsm5, //FSM5 interrupt status
                3 is_fsm4, //FSM4 interrupt status
                2 is_fsm3, //FSM3 interrupt status
                1 is_fsm2, //FSM2 interrupt status
                0 is_fsm1 //FSM1 interrupt status
           },
           0x4F INTERNAL_FREQ_FINE r freq_fine, //Difference in percentage of
                //the effective ODR (and timestamp rate) with respect to the typical.
                //Step: 0.13%. 8-bit format, two's complement.
                //See datasheet for more info.
           0x50 FUNCTIONS_ENABLE rw
           {
                7 enable_interrupts, //Enable basic interrupts
                6 enable_timestamp, //Enable timestamp counter
                3 dis_res_on_read_all_int, //Disable reset latched on reading
                    //ALL_INT_SRC
                1..0 enable_inact //Enables sleep function. Default value: 00
                    //00: stationary/motion-only interrupts generated,
                    //accel/gyro configuration do not change.
                    //01: sets accel to low-power mode 1 with accel ODR selected
                    //through the XL_INACT_ODR_[1:0] bits of the INACTIVITY_DUR (54h)
                    //register, gyroscope configuration does not change.
                    //10: sets accelerometer to low-power mode 1 with accelerometer
                    //ODR selected through the XL_INACT_ODR_[1:0] bits of the
                    //INACTIVITY_DUR (54h) register, gyroscope in sleep mode;
                    //11: sets accelerometer to low-power mode 1 with accelerometer
                    //ODR selected through the XL_INACT_ODR_[1:0] bits of the
                    //INACTIVITY_DUR (54h) register, gyroscope in power-down mode
           },
           0x51 DEN rw
           {
                6 den_lvl, //Enables DEN data level-sensitive trigger mode
                5 den_lvl_latched, //Enables DEN data level-sensitive latched mode
                4 den_accel, //Extends DEN to accel
                3 den_x, //DEN value stored on LSB of x-axis, default yes
                2 den_y, //DEN value stored on LSB of y-axis, default yes
                1 den_z, //DEN value stored on LSB of z-axis, default yes
                0 den_accel_gyro //DEN stamping sensor selection. Default 0.
                    //0: DEN pin info stamped in the gyroscope axis selected by bits
                    //DEN_X, DEN_Y, DEN_Z
                    //1: DEN pin info stamped in the accelerometer axis selected by
                    //bits DEN_X, DEN_Y, DEN_Z)
           },
           0x54 INACTIVITY_DUR rw
           {
                7 sleep_status_on_int, //Sleep interrupt mode configuration.
                    //If the INT1_SLEEP_CHANGE or INT2_SLEEP_CHANGE bit is enabled,
                    //drives the sleep status or sleep change on the INT pin.
                    //Default value: 0
                    //0: sleep change notification on INT pin
                    //1: sleep status reported on INT pin
                6..4 wu_inact_ths_w,
                    //Weight of 1 LSB of wake-up (WU_THS) and activity/inactivity
                    //(INACT_THS) threshold.
                    //000: 7.8125 mg/LSB (default)
                    //001: 15.625 mg/LSB;
                    //010: 31.25 mg/LSB;
                    //011: 62.5 mg/LSB;
                    //100: 125 mg/LSB;
                    //101 - 110 - 111: 250 mg/LSB
                3..2 accel_inact_odr, //Selects the ODR_XL target during inactivity.
                    //00: 1.875 Hz;
                    //01: 15 Hz (default);
                    //10: 30 Hz;
                    //11: 60 Hz
                1..0 inact_dur
                    //Duration in the transition from stationary to motion
                    //(from inactivity to activity).
                    //00: transition to motion (activity) immediately at first
                    //overthreshold event (default);
                    //01: transition to motion (activity) after two consecutive
                    //overthreshold events;
                    //10: transition to motion (activity) after three consecutive
                    //overthreshold events;
                    //11: transition to motion (activity) after four consecutive
                    //overthreshold events
           },
           0x55 INACTIVITY_THS rw
           {
                5..0 inact_ths //Activity/inactivity threshold. The resolution of the
                    //threshold depends on the value of WU_INACT_THS_W_[2:0] in the
                    //INACTIVITY_DUR (54h) register. Default value: 000000
           },
           0x56 TAP_CFG0 rw
           {
                6 low_pass_on_6d,
                    //LPF2 filter on 6D function selection. Refer to Figure 30.
                    //Default value: 0
                    //0: ODR/2 low-pass filtered data sent to 6D interrupt function;
                    //1: LPF2 output data sent to 6D interrupt function
                5 hw_func_mask_accel_settle,
                    //Enables masking the execution trigger of the basic interrupt
                    //functions (6D/4D, free-fall, wake-up, tap, activity/inactivity)
                    //when accelerometer data are settling. Default value: 0
                    //Note: Refer to the product application note for the details
                    //regarding operating/power mode configurations, settings,
                    //turn-on/off time and on-the-fly changes.
                4 slope_fds,
                    //HPF or slope filter selection on wake-up and activity/inactivit
                    //functions. Refer to Figure 30 (datasheet).
                3 tap_x_en, //enable x tap recognition
                2 tap_y_en, //enable y tap recognition
                1 tap_z_en, //enable z tap recognition
                0 lir //Latched interrupt
           },
           0x57 TAP_CFG1 rw
           {
                7..5 tap_priority, //Selection of axis priority for tap detection
                    //input     max     mid     min
                    //000       x       y       z
                    //001       y       x       z
                    //010       x       z       y
                    //011       z       y       x
                    //100       x       y       z
                    //101       y       z       x
                    //110       z       x       y
                    //111       z       y       x
                4..0 tap_ths_x //X-axis tap recognition threshold. Default value: 0
                    //1 LSB = FS_XL / (2^5)
           },
           0x58 TAP_CFG2 rw
           {
                4..0 tap_ths_y //like tap_ths_x but y
           },
           0x59 TAP_THS_6D rw
           {
                7 d4d_en, //Enables 4D orientation detection.
                    //Z-axis position detection is disabled. Default value: 0
                6..5 sixd_ths, //Threshold for 4D/6D function. Default value: 00
                    //00: 80�
                    //01: 70�
                    //10: 60�
                    //11: 50�
                4..0 tap_ths_z //like tap_ths_x but z
           },
           0x5A TAP_DUR rw
           {
                7..4 tap_dur, //
                    //Duration of maximum time gap for double-tap recognition.
                    //Default: 0000
                    //When double-tap recognition is enabled, this register expresses
                    //the maximum time between two consecutive detected taps to
                    //determine a double-tap event. The default value of these bits is
                    //0000b which corresponds to 16/ODR_XL time. If the DUR_[3:0] bits
                    //are set to a different value, 1LSB corresponds to 32/ODR_XL
                    //time.
                3..2 tap_quiet, //
                    //Expected quiet time after a tap detection. Default value: 00
                    //Quiet time is the time after the first detected tap in which
                    //there must not be any overthreshold event. The default value of
                    //these bits is 00b which corresponds to 2/ODR_XL time. If the
                    //QUIET_[1:0] bits are set to a different value, 1LSB corresponds
                    //to 4/ODR_XL time.
                1..0 tap_shock //
                    //Maximum duration of overthreshold event. Default value: 00
                    //Maximum duration is the maximum time of an overthreshold
                    //signal detection to be recognized as a tap event. The default
                    //value of these bits is 00b which corresponds to 4/ODR_XL time.
                    //If the SHOCK_[1:0] bits are set to a different value, 1LSB
                    //corresponds to 8/ODR_XL time.
           },
           0x5B WAKE_UP_THS rw
           {
                7 single_double_tap, //Enables single/double-tap event. Default 0
                6 usr_off_on_wu, //
                    //Drives the low-pass filtered data with user offset correction
                    //(instead of high-pass filtered data) to the wake-up and the
                    //activity/inactivity functions. Refer to Figure 30. Default
                    //value: 0
                5..0 wk_ths //
                    //Wake-up threshold. The resolution of the threshold depends on
                    //the value of WU_INACT_THS_W_[2:0] in the INACTIVITY_DUR (54h)
                    //register. Default value: 000000
           },
           0x5C WAKE_UP_DUR rw
           {
                7 ff_dur[0], //Free-fall duration event. Default: 0
                    //For the complete configuration of the free-fall duration, refer
                    //to FF_DUR_[4:0] in the FREE_FALL (5Dh) configuration.
                    //1 LSB = 1/ODR_XL time
                6..5 wake_dur, //Wake-up duration event. Default: 00
                    //1 LSB = 1/ODR_XL time
                3..0 sleep_dur //
                    //Duration to go in sleep mode. Default value:
                    //0000 (this corresponds to 16 ODR)
                    //1 LSB = 512/ODR_XL time
           },
           0x5D FREE_FALL rw
           {
                7..3 ff_dur[1..5], //Free-fall duration event. Default: 00000
                    //For the complete configuration of the free-fall duration, refer
                    //to FF_DUR_5 in the WAKE_UP_DUR (5Ch) configuration.
                2..0 ff_ths, //Free-fall threshold setting. Default: 000
                    //000: 156mg
                    //001: 219mg
                    //010: 250mg
                    //011: 312mg
                    //100: 344mg
                    //101: 406mg
                    //110: 469mg
                    //111: 500mg
           },
           0x5E MD1_CFG rw
               //Functions routing to INT1 pin register (R/W). Each bit in this
               //register enables a signal to be carried over the INT1 pin. The output
               //of the pin is the OR combination of the signals selected here and in
               //the INT1_CTRL (0Dh) register.
           {
                7 int1_sleep_change, //
                    //Routing activity/inactivity recognition event to INT1.
                    //Default: 0
                    //(0: routing activity/inactivity event to INT1 disabled;
                    //1: routing activity/inactivity event to INT1 enabled)
                6 int1_single_tap, //
                    //Routing single-tap recognition event to INT1. Default: 0
                    //(0: routing single-tap event to INT1 disabled;
                    //1: routing single-tap event to INT1 enabled)
                5 int1_wu, //
                    //Routing wake-up event to INT1. Default value: 0
                    //(0: routing wake-up event to INT1 disabled;
                    //1: routing wake-up event to INT1 enabled)
                4 int1_ff, //
                    //Routing free-fall event to INT1. Default value: 0
                    //(0: routing free-fall event to INT1 disabled;
                    //1: routing free-fall event to INT1 enabled)
                3 int1_double_tap, //
                    //Routing tap event to INT1. Default value: 0
                    //(0: routing double-tap event to INT1 disabled;
                    //1: routing double-tap event to INT1 enabled)
                2 int1_6d, //
                    //Routing 6D event to INT1. Default value: 0
                    //(0: routing 6D event to INT1 disabled;
                    //1: routing 6D event to INT1 enabled)
                1 int1_emb, //
                    //Routing embedded functions event to INT1. Default value: 0
                    //(0: routing embedded functions event to INT1 disabled;
                    //1: routing embedded functions event to INT1 enabled)
                0 int1_shub //
                    //Routing sensor hub communication concluded event to INT1.
                    //Default value: 0
                    //(0: routing sensor hub communication concluded event to
                    //INT1 disabled;
                    //1: routing sensor hub communication concluded event to INT1
                    //enabled)
           },
           0x5F MD2_CFG rw
               //Functions routing to INT2 pin register (R/W). Each bit in this
               //register enables a signal to be carried over the INT2 pin. The output
               //of the pin is the OR combination of the signals selected here and in
               //the INT2_CTRL (0Eh) register.
           {
                7 int2_sleep_change, //
                    //Routing activity/inactivity recognition event to INT1.
                    //Default: 0
                    //(0: routing activity/inactivity event to INT1 disabled;
                    //1: routing activity/inactivity event to INT1 enabled)
                6 int2_single_tap, //
                    //Routing single-tap recognition event to INT1. Default: 0
                    //(0: routing single-tap event to INT1 disabled;
                    //1: routing single-tap event to INT1 enabled)
                5 int2_wu, //
                    //Routing wake-up event to INT1. Default value: 0
                    //(0: routing wake-up event to INT1 disabled;
                    //1: routing wake-up event to INT1 enabled)
                4 int2_ff, //
                    //Routing free-fall event to INT1. Default value: 0
                    //(0: routing free-fall event to INT1 disabled;
                    //1: routing free-fall event to INT1 enabled)
                3 int2_double_tap, //
                    //Routing tap event to INT1. Default value: 0
                    //(0: routing double-tap event to INT1 disabled;
                    //1: routing double-tap event to INT1 enabled)
                2 int2_6d, //
                    //Routing 6D event to INT1. Default value: 0
                    //(0: routing 6D event to INT1 disabled;
                    //1: routing 6D event to INT1 enabled)
                1 int2_emb, //
                    //Routing embedded functions event to INT1. Default value: 0
                    //(0: routing embedded functions event to INT1 disabled;
                    //1: routing embedded functions event to INT1 enabled)
                0 int2_timestamp //
                    //Enables routing the alert for timestamp overflow within 5.6 ms
                    //to the INT2 pin.
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
