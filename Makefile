CP		:= cp
RM 		:= rm -f
KERNEL	:= target/i686-unknown-none/debug/kfs
IMAGE	:= kfs1.iso

.PHONY: fmt build clean fclean wup wdown wit wruncmd cp_image all run
all: cp_image wruncmd

cp_image: build
	$(CP) $(KERNEL) img/boot
	ls -l $(KERNEL)

fmt:
	cargo fmt --all

build: fmt
	cargo build

clean:
	cargo clean

fclean: clean
	$(RM) target
	$(RM) $(IMAGE)
	$(RM) img/boot/kfs

wup:
	docker-compose -f compose.yaml up --build  --remove-orphans | tee docker.log

wdown:
	docker-compose -f compose.yaml down

wit:
	docker exec -it kfs-worker /bin/bash

wruncmd:
	docker exec -it kfs-worker sh -c "cd /work && grub-mkrescue -o $(IMAGE) img"

run: fclean all
	qemu-system-i386 $(IMAGE)
