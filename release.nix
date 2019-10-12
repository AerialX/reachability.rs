{ pkgs ? import <nixpkgs> { } }: let
  toTomlString = val: with pkgs.lib;
    if isString val then ''"${val}"''
    else if isInt val then toString val
    else if val == true then "true"
    else if val == false then "false"
    else throw ''Unknown value "${toString val}"'';
  optLevels = map (optLevel: { debugAssertions = false; inherit optLevel; }) [ 0 1 2 3 "s" "z" ];
  ltos = map (lto: { inherit lto; }) [ false "thin" "fat" ]; # "plugin" -> -C linker-plugin-lto
  permuteOpts = with pkgs.lib; crossLists mergeAttrs [ optLevels ltos ] ++ [ {
    debugAssertions = true;
    optLevel = 0;
    lto = false;
  } ];
  tests = [ "opt1" "opt2" "lto" "fail" "fail-black-box" ];
  testsFor = { debugAssertions, optLevel, lto }: with pkgs.lib;
    optional (debugAssertions || isString optLevel || optLevel > 0) "opt1"
    ++ optional (debugAssertions || isString optLevel || optLevel > 1) "opt2"
    ++ optional (debugAssertions || ((isString optLevel || optLevel > (if lto == "fat" then 1 else 0)) && lto != false)) "lto"
    ++ optionals debugAssertions [ "fail" "fail-black-box" ];
  failingTests = { debugAssertions, optLevel, lto }@opt: with pkgs.lib;
    subtractLists (testsFor opt) tests;

  testFn = { rustPlatform, runCommand }: { debugAssertions, optLevel, lto }@opt: rustPlatform.buildRustPackage {
    name = "reachability-lto=${toString optLevel}${toString lto}";
    cargoVendorDir = runCommand "empty-cargo-vendor" { } ''
      mkdir $out
    '';

    src = pkgs.nix-gitignore.gitignoreSource [''
      *.nix
      .github/
    ''] ./.;

    optLevel = toTomlString optLevel;
    optLto = toTomlString lto;
    optDebugAssertions = toTomlString debugAssertions;
    preBuild = ''
      cat >> Cargo.toml <<EOF
      [profile.test]
      incremental = false
      debug-assertions = $optDebugAssertions
      opt-level = $optLevel
      lto = $optLto
      [profile.dev]
      incremental = false
      debug-assertions = $optDebugAssertions
      opt-level = $optLevel
      lto = $optLto
      EOF
      cargo generate-lockfile
    '';

    installPhase = "touch $out";

    ctests = testsFor opt;
    ctestArgs = map (t: "--test ${t}") (testsFor opt);
    ctestFailures = failingTests opt;

    # TODO: unstable feature for nightly rustc
    checkPhase = ''
      echo "[cargo] test $ctests"
      cargo test --features unstable-internal-test --doc
      cargo test --features unstable-internal-test --lib $ctestArgs

      for ctest in $ctestFailures; do
        echo "[cargo] test fail $ctest"
        if cargo build --features unstable-internal-test --test $ctest; then
          echo "expected $ctest failure" >&2
          exit 1
        fi
      done
    '';
  };
  testFn' = pkgs.callPackage testFn { };
in pkgs.lib.listToAttrs (map (f: pkgs.lib.nameValuePair f.name f) (map testFn' permuteOpts)) // {
  all = map testFn' permuteOpts;
}
