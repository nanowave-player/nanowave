# Battery Pal Review

This document is a listing of all tasks performed for a full review of [Battery Pal](https://pnlabs.ca/batterypal/), a custom battery and power module for embedded systems.

## Scope

The scope of this review is targeted towards a portable audio player based on the LicheeRV Nano, a 1500mah battery and different USB-C to 3.5mm audio adapters.

## test cases

### USB-C Power only (no battery)
Check the following aspects while no battery is attached:
- [] Power consumption
- [] Voltage / stability
- [] Compare with LX-LCBST / TP4057 / ESP32C6

### Battery Power only (no USB)
- [] Power consumption
- [] Voltage / stability
- [] Undervoltage protection

### Battery Charging
- [] Charging speed
- [] Recharging Undervolted battery
- [] Overvoltage Protection
- [] Recharging a known bad battery
- [] Different battery types and sizes (all LiPo)
- [] Battery Voltage curve

### Power to Battery switch
- [] General stability
- [] Multiple quick switches
- [] 

### Battery Gauge tests (MAX17043 - later maybe MAX17048, MAX17055 or BQ27441)
- [] General connectivity
- [] Discharging curve


### Audio Noise

- [] "Personal impression" test - subjective comparison to other modules in regards of background noise
- [] Line-in recording tests (probably different USB-C 3.5mm adapters)
  - [] Silence playback on USB-C Power (no battery)
  - [] Silence playback on Battery Power
  - [] Silence playback on Power to Battery switch
  - [] Silence playback on Battery to Power switch
  - [] Silence playback while battery charging

