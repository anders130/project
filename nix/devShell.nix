{
    perSystem = {pkgs, ...}: {
        devShells.default = pkgs.mkShell {
            packages = with pkgs; [
                cargo
                rustc
                rustfmt
                clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
    };
}
