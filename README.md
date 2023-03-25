# emulot

A configuration manager for QEMU

Detailed documentation can be found [here](https://jeff.caffreyhill.com/emulot).

# Example config

```toml
arch = "aarch64"
memory = 2048
accel = "hvf"
cpu = "host
[boot]
order = "d"
[machine]
type = "virt"
highmem = "on"
```
