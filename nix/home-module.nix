{lib, ...}: let
    inherit (lib) mkEnableOption mkOption mkIf types concatMapStrings;
in {
    flake.homeManagerModules.default = {config, ...}: let
        cfg = config.programs.project;

        generatedTemplate = ''
            name: {name}
            path: {path}

            windows:
              - name: code
                panes:
                  - commands:
                      - command: ${cfg.editor}
                        args:
                          ${concatMapStrings (a: "- ${a}\n") cfg.editorArgs}
              - name: shell
                panes:
                  - commands: []
        '';
    in {
        options.programs.project = {
            enable = mkEnableOption "project tmux session manager";
            editor = mkOption {
                type = types.str;
                default = "nvim";
                description = "Editor command launched in the code window.";
            };
            editorArgs = mkOption {
                type = types.listOf types.str;
                default = ["."];
                description = "Arguments passed to the editor.";
            };
            template = mkOption {
                type = types.nullOr types.lines;
                default = null;
                description = ''
                    Raw laio YAML template written to
                    $XDG_CONFIG_HOME/project/template.yaml.
                    Use {name} and {path} as session-name/path placeholders.
                    Overrides editor and editorArgs when set.
                '';
            };
        };
        config = mkIf cfg.enable {
            xdg.configFile."project/template.yaml".text =
                if cfg.template != null
                then cfg.template
                else generatedTemplate;
        };
    };
}
