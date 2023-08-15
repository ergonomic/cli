# Ergonomic.dev :: `ergo-envrc`

## Usage

```bash
echo "LALA is ${LALA}"
```

## Setup

Create a `.local/envrc` from the following template:

```envrc
# ------------------------------------------------------------------------------
# Section: GitHub (git)
#
# Notes:
#
# - The token:
#   - Is owned by `mechanical-bot` and named "root/mechanics".
#   - Has read/write access to members.
# ------------------------------------------------------------------------------
export TF_VAR_git='{
  organization = "{{ op://${ENVRC_VAULT}/${ENVRC_ENTRY}/git/ORGANIZATION }}"
  token        = "{{ op://${ENVRC_VAULT}/${ENVRC_ENTRY}/git/TOKEN }}"
}'
```

Then, add this stuff too...

```envrc
# ------------------------------------------------------------------------------
# Section: BLABS!
# ------------------------------------------------------------------------------
export LALA="lala"
```
