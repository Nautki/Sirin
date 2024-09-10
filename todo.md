- [ ] Add GND/VCC to GPIO pins.
- [ ] Add SPI/I2C (non-overlapping) to GPIO pins.
- [ ] Put a bang (!) at the end of the phrase on the back.
- [ ] Get headers that enforce the orientation of the debug.
- [ ] Our PCBs use GND. ST's use VSS. Use VSS.

Core (very important!)


Analog
- [ ] VDDA not connected. It's got a cap to ground but nothing else. https://community.st.com/t5/stm32-mcus-products/can-vdda-vref-and-vssa-connect-to-vss-ground/td-p/619100
- [ ] VREF+ not in our schematic?
- [ ] Reference impl has VREF+ to resistor to VDDA (which is to inductor to VDD). Ours has the pin that would be VREF+ straight to VDD. That should be checked.