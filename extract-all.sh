#!/bin/bash

peri=$1
mkdir -p regs/$peri

cargo build --release --manifest-path ../svd2rust/Cargo.toml

for f in `ls svd`; do
    f=${f#"stm32"}
    f=${f%".svd"}
    echo -n processing $f ...
    RUST_LOG=info ../svd2rust/target/release/svd4rust extract-peripheral --svd svd/stm32$f.svd --transform transform.yaml --peripheral $peri > regs/$peri/$f.yaml 2> regs/$peri/$f.yaml.out
    if [ $? -ne 0 ]; then 
        mv regs/$peri/$f.yaml.out regs/$peri/$f.err
        echo FAIL
    else
        rm regs/$peri/$f.yaml.out
        echo OK
    fi
done
