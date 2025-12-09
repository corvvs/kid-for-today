FROM debian:stable-slim

# 必要なツールをインストール
# - grub-pc-bin / grub-common: BIOS用GRUB + grub-mkrescue
# - xorriso: ISOイメージ生成
# - mtools: 一部のgrub-mkrescueが要求
# - make / gcc / nasm: コンテナ内でビルドしたくなったとき用（任意）
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        grub-pc-bin grub-common \
        xorriso mtools \
        make gcc nasm \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリ
WORKDIR /work

# デフォルトでシェルに入る
ENTRYPOINT ["/bin/bash"]
