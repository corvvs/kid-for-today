CP		:= cp
RM 		:= rm -f
KERNEL	:= target/i686-unknown-none/debug/kfs

all: cp_image wruncmd

cp_image: build
	$(CP) $(KERNEL) img/boot
	ls -l $(KERNEL)

build: fclean
	cargo build

clean:
	cargo clean

fclean: clean
	$(RM) target
	$(RM) img/kfs1.iso
	$(RM) img/boot/kfs

wup:
	docker-compose -f compose.yaml up --build  --remove-orphans | tee docker.log

wdown:
	docker-compose -f compose.yaml down

wit:
	docker exec -it kfs-worker /bin/bash

wruncmd:
	docker exec -it kfs-worker sh -c "cd /work && grub-mkrescue -o kfs1.iso img"
