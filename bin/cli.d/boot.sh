#!/usr/bin/env bash
set -euo pipefail
eval "$(ergo stdlib)"

# ------------------------------------------------------------------------------
# Usage:
#
#   boot [<selector>] [options]
#
# Overview:
#
#   Bootstrap various repository dependencies and configuration.
#
# Details:
#
#   $ ergo boot
#
#     Execute all defined bootstrapping tasks. See below for further details.
#
#   $ ergo boot brew
#
#     Using Homebrew, install/upgrade packages found in 'Brewfile'.
#     Also ensures that Homebrew is intalled.
#
#   $ ergo boot rust
#
#     Ensure that the rustup toolchain is installed and updated.
#
# Options:
#
#   -h, --help      Print help text.
# ------------------------------------------------------------------------------

arg::parse "$@"

main() {
  fallback=${1:-all}
  selector=${selector:-$fallback}

  pushd ${REPO} > /dev/null
    case "${selector}" in
      all   )
        boot::all
        ;;
      brew  )
        boot::brew
        ;;
      rust  )
        boot::rust
        ;;
      * )
        cmd::error "Unrecognized selector: '${selector}'"
        ;;
    esac
  popd > /dev/null
}

boot::all() {
  boot::brew
  boot::rust
}

boot::brew() {
  echo::info 'installing/upgrading packages via `brew bundle`'

  located() {
    $(brew --version &> /dev/null)
  }

  install() {
    echo::info "first, installing Homebrew"
    url="https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"

    echo::exec "/bin/bash -c \"\$(curl -fsSL ${url})\""
    /bin/bash -c "$(curl -fsSL ${url})"
  }

  located || install
  brew bundle
}

boot::rust() {
  expected="rustup 1.26.0"
  echo::info "ensuring \`${expected}\` toolchain is installed"

  located() {
    rustup --version &> /dev/null
  }

  matched() {
    rustup --version 2> /dev/null | grep "${expected}" > /dev/null
  }

  install() {
    echo::exec "installing \`rustup\`"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  }

  upgrade() {
    echo::exec "upgrading \`rustup\`"
    rustup update
  }

  located || install
  matched || upgrade
}

main "$@"
