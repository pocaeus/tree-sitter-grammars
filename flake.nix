{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs.darwin.apple_sdk.frameworks) Security;
      in {
        devShell = with pkgs;
          mkShell {
            buildInputs =
              [ cargo clippy rustc rustfmt rust-analyzer libiconv pkg-config ]
              ++ lib.optionals stdenv.isDarwin [ Security ];
            shellHook = ''
              echo && echo && echo "Entering dev shell for 'tree-sitter-grammars' project."
            '';
          };
      });
}
