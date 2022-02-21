## Up
### Compile
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
cargo install espmonitor
```
#### Use it
```
make flash
make look
```
### View
![E5B9D164-DBD5-4673-BD3D-71A6EEBC8EDC_1_105_c](https://tva1.sinaimg.cn/large/e6c9d24egy1gzizfgd8bvj20wu0ih0wa.jpg)