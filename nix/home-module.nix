{
    lib,
    inputs,
    ...
}: let
    inherit (lib) mkEnableOption mkOption mkIf types concatStringsSep;
in {
    flake.homeManagerModules.default = {
        config,
        pkgs,
        ...
    }: let
        cfg = config.programs.project;
    in {
        options.programs.project = {
            enable = mkEnableOption "project tmux session manager";
            package = mkOption {
                type = types.package;
                default = inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.default;
                description = "The project package to install.";
            };
            palette = mkOption {
                type = types.nullOr (types.listOf types.str);
                default = null;
                description = ''
                    16 hex colors (color0..color15) for syntax highlighting in glow previews.
                    Overrides automatic terminal palette detection.
                    For Stylix users:
                      palette = with config.lib.stylix.colors.withHashtag; [
                        base00 base08 base0B base0A base0D base0E base0C base05
                        base03 base09 base01 base02 base0F base06 base07 base04
                      ];
                '';
            };
        };
        config = mkIf cfg.enable {
            home.packages = [cfg.package];
            home.sessionVariables = mkIf (cfg.palette != null) {
                PROJECT_PALETTE = concatStringsSep "," cfg.palette;
            };
        };
    };
}
