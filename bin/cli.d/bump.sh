#!/usr/bin/env bash
set -euo pipefail
eval "$(ergo stdlib)"

# ------------------------------------------------------------------------------
# Usage:
#
#   bump <selector> [options]
#
# Overview:
#
#   Utilities for managing versions, releases, and the CHANGELOG.md.
#
# Details:
#
#   $ ergo bump changelog
#
#     Rebuild the CHANGELOG.md, adding any additional changes from previous.
#
# Options:
#
#       --tag=<tag>   Apply a tag for the latest version.
#   -h, --help        Print help text.
# ------------------------------------------------------------------------------

arg::parse "$@"

main() {
  pushd ${REPO} > /dev/null
    case "${selector}" in
      changelog )
        bump::changelog
        ;;
      * )
        cmd::error "Unrecognized selector: '${selector}'"
        ;;
    esac
  popd > /dev/null
}

bump::changelog() {
  if [ "${tag}" != "" ] ; then
    git-cliff --latest --config etc/git/cliff.toml --tag "${tag}"
  else
    git-cliff --unreleased --config etc/git/cliff.toml
  fi
}

main "$@"
