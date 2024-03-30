{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.aarch64-darwin;
  in
  {
    devShell.aarch64-darwin = pkgs.mkShell {
      buildInputs = with pkgs; [
        libiconv
        maturin
        napi-rs-cli
        nodejs_20
        openssl
        pkg-config
        python311
        rustup
        yarn
      ];
    };
  };
}
