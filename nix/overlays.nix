{inputs, ...}: {
    perSystem = {system, ...}: {
        _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [(_: _: {laio = inputs.laio-cli.packages.${system}.default;})];
        };
    };
}
