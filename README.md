# Sleep mode TIM2

OFF with sleep: 700uA
snake: 2.36 mA
blink: 1.56 mA
On: 7.96mA

# sleep mode when active, but stop mode when idle
OFF with sleep: 70 uA good enough.

`
cargo objcopy --release --target=thumbv6m-none-eabi -- -O ihex firmware.hex
`