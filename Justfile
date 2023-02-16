build:
    cargo build --target riscv64gc-unknown-linux-musl -Zbuild-std=std,panic_abort

build-release:
    cargo build --target riscv64gc-unknown-linux-musl -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --release

assemble_datapack: build
    rm ./datapack_build/oc_rs.zip
    rm -rf ./datapack_build/filesystem/
    rm -rf ./datapack_build/build/

    mkdir -p ./datapack_build/filesystem/oc_rs/
    mkdir ./datapack_build/build/
    mkdir -p ./datapack_build/build/data/oc2/file_systems/

    cp ./target/riscv64gc-unknown-linux-musl/debug/oc_rs_cli ./datapack_build/filesystem/oc_rs/
    cd ./datapack_build/filesystem/ && zip -r ../build/data/oc2/file_systems/oc_rs.zip ./oc_rs/ 
    cp ./datapack_build/pack.mcmeta ./datapack_build/build/
    cp ./datapack_build/oc_rs.json ./datapack_build/build/data/oc2/file_systems/

    cd ./datapack_build/build/ && zip -r ../oc_rs.zip ./*

