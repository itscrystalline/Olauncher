{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  android = {
    enable = true;
    ndk.enable = true;
    android-studio = {
      enable = true;
    };
    buildTools.version = ["35.0.0"];
    platforms.version = ["34"];
  };

  languages.rust = {
    enable = true;
    channel = "stable";
    components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
    targets = ["aarch64-linux-android" "x86_64-linux-android"];
  };
  languages.python.enable = true;
  # # https://devenv.sh/basics/

  env.QT_QPA_PLATFORM = "xcb";
  #
  # # https://devenv.sh/packages/
  # packages = [ pkgs.git ];
  #
  # # https://devenv.sh/languages/
  # # languages.rust.enable = true;
  #
  # # https://devenv.sh/processes/
  # # processes.cargo-watch.exec = "cargo-watch";
  #
  # # https://devenv.sh/services/
  # # services.postgres.enable = true;
  #
  # # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';
  #
  # enterShell = ''
  #   hello
  #   git --version
  # '';
  #
  # # https://devenv.sh/tasks/
  # # tasks = {
  # #   "myproj:setup".exec = "mytool build";
  # #   "devenv:enterShell".after = [ "myproj:setup" ];
  # # };
  #
  # # https://devenv.sh/tests/
  # enterTest = ''
  #   echo "Running tests"
  #   git --version | grep --color=auto "${pkgs.git.version}"
  # '';
  #
  # # https://devenv.sh/git-hooks/
  # # git-hooks.hooks.shellcheck.enable = true;
  #
  # # See full reference at https://devenv.sh/reference/options/
}
