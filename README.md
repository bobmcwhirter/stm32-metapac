# stm32-metapac

    # clone custom svd2rust
    git clone https://github.com/Dirbaio/svd2rust --branch stm32 ../svd2rust

    # this is how to extract a regblock
    RUST_LOG=info cargo run --manifest-path ../svd2rust/Cargo.toml -- extract --svd svd/stm32f411.svd --xfrm transform.yaml --peri GPIOA > regs/gpio_v2.yaml

    # This generates the metapac from all regblocks in `regs/`
    ./update.sh


The SVDs in `svd/` are the patched SVDs from https://github.com/stm32-rs/stm32-rs