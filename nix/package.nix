{inputs, ...}: {
    perSystem = {
        pkgs,
        lib,
        system,
        ...
    }: let
        laio = inputs.laio-cli.packages.${system}.default;
    in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "project";
            version = "0-unstable";
            src = ../.;
            cargoLock.lockFile = ../Cargo.lock;
            nativeBuildInputs = [pkgs.makeWrapper];
            postInstall = ''
                wrapProgram $out/bin/project \
                  --prefix PATH : ${lib.makeBinPath [pkgs.fzf pkgs.tmux pkgs.bat laio]}
            '';
            meta.description = "Open a git project in a tmux session";
        };
    };
}
