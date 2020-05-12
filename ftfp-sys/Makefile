THIS_DIR := $(dir $(realpath $(firstword $(MAKEFILE_LIST))))
SRC_DIR := $(THIS_DIR)/libfixedtimefixedpoint
ifndef BUILD_DIR 
    $(error BUILD_DIR is not set)
endif
DEFAULT_TARGET := $(BUILD_DIR)/default
SGX_TARGET := $(BUILD_DIR)/sgx
LIB_DIR := $(BUILD_DIR)/lib

FTFP_INTBITS ?= 32

######## SGX SDK Settings ########

SGX_SDK ?= /opt/intel/sgxsdk
SGX_ARCH ?= x64

ifeq ($(shell getconf LONG_BIT), 32)
	SGX_ARCH := x86
else ifeq ($(findstring -m32, $(CXXFLAGS)), -m32)
	SGX_ARCH := x86
endif

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_FLAGS := -m32
else
	SGX_COMMON_FLAGS := -m64
endif

SGX_COMMON_CFLAGS := $(SGX_COMMON_FLAGS)

######## Enclave Settings ########

Enclave_Include_Paths := -I$(THIS_DIR) -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc
Enclave_C_Flags := $(Enclave_Include_Paths) -nostdinc -fvisibility=hidden -fpie -ffunction-sections -fdata-sections $(MITIGATION_CFLAGS)
CC_BELOW_4_9 := $(shell expr "`$(CC) -dumpversion`" \< "4.9")
ifeq ($(CC_BELOW_4_9), 1)
	Enclave_C_Flags += -fstack-protector
else
	Enclave_C_Flags += -fstack-protector-strong
endif

##################################

include $(SRC_DIR)/Makefile

default_lib := $(LIB_DIR)/$(libs)
sgx_lib := $(LIB_DIR)/libftfp_sgx.a 
pseudo_src := $(THIS_DIR)/pseudofunctions.c
pseudo_obj := ${subst $(THIS_DIR),$(SGX_TARGET),$(pseudo_src:.c=.o)}
sgx_obj := ${addprefix $(SGX_TARGET)/,$(ftfp_obj)} $(pseudo_obj)

ifeq ($(SGX), 1)
    TARGET_DIR := $(SGX_TARGET)
    CFLAGS += $(SGX_COMMON_CFLAGS) $(Enclave_C_Flags)
    target := $(sgx_lib)
else
    TARGET_DIR := $(DEFAULT_TARGET)
    target := $(default_lib) 
endif
$(info $(pseudo_obj))

.PHONY: target clean

.DEFAULT_GOAL := target

target: $(target)

$(TARGET_DIR): $(SRC_DIR)
	mkdir -p $(TARGET_DIR)
	cp $(realpath $(SRC_DIR))/* $(TARGET_DIR)
	(cd $(TARGET_DIR) && \
	    python3 generate_base.py --file base.h --pyfile base.py --intbits $(FTFP_INTBITS))

$(default_lib): $(TARGET_DIR) 
	$(MAKE) -C $(TARGET_DIR)
	mkdir -p $(LIB_DIR)
	cp -ans $(TARGET_DIR)/$(libs) $@

$(sgx_lib): sgx_obj_make $(pseudo_obj)
	mkdir -p $(LIB_DIR)
	ar rcs $@ $(sgx_obj)

$(pseudo_obj): $(pseudo_src) $(ftfp_inc:%.h=$(TARGET_DIR)/%.h) | sgx_obj_make
	$(CC) -c -o $@ $(CFLAGS) $<

sgx_obj_make: $(TARGET_DIR)
	$(MAKE) -C $(TARGET_DIR) -f $(THIS_DIR)/Makefile all

clean:
	rm -rf $(DEFAULT_TARGET) $(SGX_TARGET) $(pseudo_obj) $(sgx_lib) $(default_lib)
