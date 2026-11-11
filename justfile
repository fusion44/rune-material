# New files must be Git-tracked before Nix can include them in a build.
set positional-arguments
system := arch() + "-linux"

# Run the component gallery. Codringher lives in ../codringher now.
run *args:
    nix run .#gallery -- "$@"

test:
    nix build .#checks.{{system}}.tests --no-link --print-build-logs

check:
    nix flake check --print-build-logs

fmt:
    cargo fmt --check

build:
    nix build .#library .#gallery .#dms-theme .#calculator

snaps output="screenshots":
    nix build .#gallery
    mkdir -p "{{output}}"
    ./result/bin/gallery --snap "{{output}}/light.png" --scene light
    ./result/bin/gallery --snap "{{output}}/dark.png" --scene dark

default:
    @just --list
