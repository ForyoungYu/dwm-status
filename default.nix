with import <nixpkgs> { };

let
  binPath = stdenv.lib.makeBinPath [
    alsaUtils     # audio:   alsactl, amixer
    coreutils     # audio:   stdbuf
    dnsutils      # network: dig
    iproute       # network: ip
    wirelesstools # network: iwgetid
  ];
in

rustPlatform.buildRustPackage rec {
  inherit cargoSha256;

  name = "dwm-status";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "0l6x59bzzilc78gsi5rlgq9zjvp8qjphfsds776ljzmkbdq8q4iz";

  nativeBuildInputs = [ makeWrapper pkgconfig ];
  buildInputs = [ dbus gdk_pixbuf libnotify xorg.libX11 ];

  postInstall = ''
    wrapProgram $out/bin/${name} --prefix "PATH" : "${binPath}"
  '';
}
