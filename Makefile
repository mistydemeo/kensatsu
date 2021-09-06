prefix := /usr/local
bindir := ${prefix}/bin
INSTALL := install

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	configdir := ${HOME}/Library/Application\ Support/rs.kensatsu
	configfile = config/default-nonlinux.yaml
else
	configdir := ${HOME}/.config/rs.kensatsu
	configfile = config/default.yaml
endif

target/release/main:
	cargo build --release

all: target/release/main

install: all install-config
	${INSTALL} -d ${bindir}
	${INSTALL} target/release/main ${bindir}/kensatsu

install-config:
	${INSTALL} -d ${configdir}
	${INSTALL} ${configfile} ${configdir}/default-config.yml

.PHONY: all install
