## Up
### Hardware wire
Todo...
### Compile
1. Install rust.

2. Add Xtensa architecture support.

3. Get your mid.
brower visit : http://api.bilibili.com/x/space/myinfo get mid feild.
```
export SSID="You wifi name"
export PASS="You wifi password"
export MID="You bilibili mid"
cargo build --release
```
### Flash
#### Install toolchain
```
pip install esptool
```
#### Use it
Download bin from https://github.com/buhe/up/releases
```
esptool.py --chip esp32 -p /dev/cu.usbserial-0001 -b 460800 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size detect 0x10000 up.bin
```
-p {set your dev port}
### View
![E5B9D164-DBD5-4673-BD3D-71A6EEBC8EDC_1_105_c](https://tva1.sinaimg.cn/large/e6c9d24egy1gzizfgd8bvj20wu0ih0wa.jpg)