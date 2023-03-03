# emulot

A configuration manager for QEMU

# Example config

```yaml
arch: aarch64
memory: 2048
accel: hvf
cpu: host
boot:
  order: d
machine:
  type: virt
  highmem: on
```
