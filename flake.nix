{
    description = "Open a git project in a tmux session";

    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";
        flake-parts = {
            url = "github:hercules-ci/flake-parts";
            inputs.nixpkgs-lib.follows = "nixpkgs";
        };
        import-tree.url = "github:vic/import-tree";
        laio-cli = {
            url = "github:ck3mp3r/laio-cli";
            inputs = {
                nixpkgs.follows = "nixpkgs";
                flake-parts.follows = "flake-parts";
            };
        };
    };

    outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} (inputs.import-tree ./nix);
}
