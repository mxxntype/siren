{
    inputs,
    pkgs,
    ...
}:

let
    toolchain = (pkgs.rustChannelOf {
        rustToolchain = ../../rust-toolchain.toml;
        sha256 = "3jVIIf5XPnUU1CRaTyAiO0XHVbJl12MSx3eucTXCjtE=";
    }).rust;

    naersk' = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
    };

in naersk'.buildPackage {
    src = ../..;
}

