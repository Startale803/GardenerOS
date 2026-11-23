# GardenerOS 实验2

RISC-V 裸机环境和最小化内核。完成启动入口、链接布局、64 KiB 启动栈、BSS 清零、SBI 输出和 panic 关机。

## 运行

使用 `nightly-2026-05-08`、`riscv64gc-unknown-none-elf`、cargo-binutils、make 和 QEMU。配套固件位于 `bootloader/opensbi.bin`。

```sh
cd os
make check
make run
```

ELF 入口为 `0x80200000`。内核输出两次 `Hello, world!`、各段地址和 panic 信息，随后通过 SBI 关机。

## 实测结果

2026-11-23 在 openEuler 24.03、Rust nightly-2026-05-08、QEMU 11.0.0 和 OpenSBI v0.6 环境中验证通过，QEMU 退出码为 0。原始编译、入口检查和运行日志见 `verification/`。

启动栈范围为 `[0x80202000, 0x80212000)`，大小为 64 KiB。本例的 `.data` 和普通 BSS 均为空。
