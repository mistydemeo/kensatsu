prefix := /usr/local
bindir := ${prefix}/bin
INSTALL := install

target/release/main:
	cargo build --release

all: target/release/main

install: all
	${INSTALL} -d ${bindir}
	${INSTALL} target/release/main ${bindir}/kensatsu

.PHONY: all install
