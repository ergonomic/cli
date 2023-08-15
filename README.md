# Ergonomic.dev :: `ergo-envrc`

## Usage

```bash
echo "LALA is ${LALA}"
```

## Setup

Create a `.local/envrc` from the following template:

```envrc
# ------------------------------------------------------------------------------
# Section: Example w/ 1Password
# ------------------------------------------------------------------------------
export SECRET="{{ op://ExampleVault/example-item/EXAMPLE }}"
```

Then, add this stuff too...

```envrc
# ------------------------------------------------------------------------------
# Section: Example w/ local (no secrets)
# ------------------------------------------------------------------------------
export SIMPLE="nothing special"
```
