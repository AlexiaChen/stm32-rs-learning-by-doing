## STM32 Rust learning by doing
daplink Flash command line
## How to build

```bash
cargo build  --workspace --release
cp  ./target/thumbv7m-none-eabi/release/freshman /mnt/d/
```

## How to Reduce bin size and Flash it

```bash
# download from it https://developer.arm.com/downloads/-/gnu-rm 
# use windows or linux version what you want
arm-none-eabi-objcopy -O binary ./freshman freshman.bin

python -m pyocd flash --help
python -m pyocd list --targets
pyocd pack find stm32f103c8
pyocd pack install stm32f103c8
python -m pyocd flash --format bin ./freshman.bin -t stm32f103c8
```

Flashed success:

```txt
0001593 I Loading D:\freshman.bin [load_cmd]
[==================================================] 100%
0006638 I Erased 0 bytes (0 sectors), programmed 0 bytes (0 pages), skipped 1024 bytes (1 page) at 0.20 kB/s  [loader]
```