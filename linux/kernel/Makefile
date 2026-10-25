# Out-of-tree build of snd-fireface-ucx against the running kernel's headers (or KDIR):
#     make -C linux/kernel
#     sudo insmod linux/kernel/snd-fireface-ucx.ko
# DKMS builds it with dkms.conf.

obj-m += snd-fireface-ucx.o
snd-fireface-ucx-y := fireface_ucx.o

KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) modules

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
