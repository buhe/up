## Up
### Hardware wire
Todo...
### Only flash
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
### Compile
brower visit : http://api.bilibili.com/x/space/myinfo get mid feild.
```
export SSID="You wifi name"
export PASS="You wifi password"
export MID="You bilibili mid"
cargo build --release
```
### View
![E5B9D164-DBD5-4673-BD3D-71A6EEBC8EDC_1_105_c](https://tva1.sinaimg.cn/large/e6c9d24egy1gzizfgd8bvj20wu0ih0wa.jpg)