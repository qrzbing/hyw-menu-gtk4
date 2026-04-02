{ lib
, rustPlatform
, pkg-config
, wrapGAppsHook4
, gtk4
, gtk4-layer-shell
}:

rustPlatform.buildRustPackage {
  pname = "hyw-menu-gtk4";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    src = ./.;
    filter = path: type:
      let
        baseName = baseNameOf path;
      in
      lib.cleanSourceFilter path type
      && baseName != "target"
      && baseName != ".direnv"
      && baseName != "result";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    gtk4
    gtk4-layer-shell
  ];

  strictDeps = true;

  meta = {
    description = "GTK4 launcher/menu for Hyprland";
    homepage = "https://github.com/qrzbing/hyw-menu-gtk4";
    mainProgram = "hyw-menu-gtk4";
    platforms = lib.platforms.linux;
  };
}
