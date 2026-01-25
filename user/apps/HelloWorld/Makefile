ifeq ($(ARCH), x86_64)
	CROSS_COMPILE=x86_64-linux-musl-
else ifeq ($(ARCH), riscv64)
	CROSS_COMPILE=riscv64-linux-musl-
else ifeq ($(ARCH), loongarch64)
	CROSS_COMPILE=loongarch64-unknow-linux-gnu-
endif

CC=$(CROSS_COMPILE)gcc

.PHONY: all
all: main.c
	$(CC) -static -o HelloWorld main.c

.PHONY: install clean
install: all
	mv HelloWorld $(DADK_CURRENT_BUILD_DIR)/HelloWorld

clean:
	rm HelloWorld *.o


