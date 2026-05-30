{
    perSystem = {
        pkgs,
        lib,
        ...
    }: let
        b = env: pkg: {inherit env pkg;};
        backends = with pkgs; {
            multiplexer.tmux = b "tmux" tmux;
            launcher.laio = b "laio" laio;
            markdown = {
                glow = b "glow" glow;
                bat = b "bat" bat;
            };
            picker = {
                fzf = b "fzf" fzf;
                tv = b "tv" television;
            };
        };

        # Each arg accepts a { pkg; env; } record.
        # Use backends.<group>.<name> for predefined choices, or supply your own.
        mkPackage = {
            multiplexer ? backends.multiplexer.tmux,
            launcher ? backends.launcher.laio,
            markdown ? backends.markdown.glow,
            picker ? backends.picker.tv,
        }:
            pkgs.rustPlatform.buildRustPackage {
                pname = "project";
                version = "0-unstable";
                src = ../.;
                cargoLock.lockFile = ../Cargo.lock;
                nativeBuildInputs = [pkgs.makeWrapper];
                postInstall = ''
                    wrapProgram $out/bin/project \
                      --prefix PATH : ${lib.makeBinPath [
                          multiplexer.pkg
                          launcher.pkg
                          markdown.pkg
                          picker.pkg
                      ]} \
                      --set PROJECT_MULTIPLEXER       ${multiplexer.env} \
                      --set PROJECT_LAUNCHER          ${launcher.env} \
                      --set PROJECT_MARKDOWN_RENDERER ${markdown.env} \
                      --set PROJECT_PICKER            ${picker.env}
                '';
                meta.description = "Open a git project in a tmux session";
                passthru = {inherit backends mkPackage;};
            };
    in {
        packages.default = lib.makeOverridable mkPackage {};
    };
}
