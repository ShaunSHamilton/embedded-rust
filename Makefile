default:
	rm -rf target
	cargo rustc --bin $(p) -- -C link-arg=--script=./linker.ld
	mkdir -p $(p)/boot
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/$(p) ./$(p)/boot/kernel7.img
	cp -r firmware/* $(p)/boot