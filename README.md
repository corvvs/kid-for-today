# Kid For Today(KFS)

x86(i686) GRUB Bootable Kernel

## 依存

- Nightly Rust (by rustup)
- NASM
- Docker
- QEMU

## ビルド

```
$ make
$ qemu-system-i386 kfs1.iso 
```

## 操作

- スクリーン切り替え: Alt(Option) + 1 / 2 / 0
  - スクリーン1, 2はキーボード入力を受け付けるがスクリーン0は受け付けない(ログ表示専用)

## ファイル

```
.
├── .cargo
│   └── config.toml         cargo設定
├── build.rs                ビルドスクリプト; 主に src/boot/boot.s 用
├── Cargo.toml              クレート設定
├── compose.yaml            イメージ生成コンテナ compose.yaml
├── docker
│   └── Dockerfile          イメージ生成コンテナ Dockerfile
├── i686-unknown-none.json  ターゲットスペック
├── img                     イメージディレクトリ -> kfs1.iso
│   └── boot
│       ├── grub
│       │   └── grub.cfg
│       └── kfs             カーネル(ELF)
├── kfs1.iso                イメージファイル
├── linker.ld               リンカースクリプト
├── Makefile
├── README.md
└── src                     カーネルソース
    ├── boot
    │   └── boot.s
    ├── byte_io.rs
    ├── cursor.rs
    ├── key.rs
    ├── main.rs
    ├── printk.rs
    └── vga.rs
```


