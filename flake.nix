{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flakelight = {
      url = "github:nix-community/flakelight";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mill-scale = {
      url = "github:icewind1991/mill-scale";
      inputs.flakelight.follows = "flakelight";
    };
  };
  outputs = { mill-scale, ... }: mill-scale ./. {
    packages.mitemp-prometheus = import ./package.nix;

    nixosModules = { outputs, ... }: {
      default =
        { pkgs
        , config
        , lib
        , ...
        }: {
          imports = [ ./module.nix ];
          config = lib.mkIf config.services.mitemp.enable {
            nixpkgs.overlays = [ outputs.overlays.default ];
            services.mitemp.package = lib.mkDefault pkgs.mitemp-prometheus;
          };
        };
    };
  };
}
