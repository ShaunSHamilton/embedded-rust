default:
	rm -rf target
	cargo rustc --bin $(p) -- -C link-arg=--script=./linker.ld
	mkdir -p $(p)/boot
	arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/$(p) ./$(p)/boot/kernel.img
	cp -r firmware/* $(p)/boot
