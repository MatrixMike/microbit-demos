#!/bin/bash
arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/debug/leds out.hex
cp out.hex /media/mikeh/MICROBIT/

