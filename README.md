# Cosmonic standup plugin

## Prerequisites

- `cargo` >=1.80, nightly `wasm32-wasip2` installed
- [`wash`](https://wasmcloud.com/docs/installation) >=0.28.1

## Building

You can build your plugin using either `cargo`:

```bash
cargo +nightly build --release --target wasm32-wasip2
```

## Using the plugin

### 1. Install your newly built plugin with `wash`

`wash` plugins are stored on disk at a location you choose (by specifying `WASH_PLUGIN_DIR`) -- by default it is `~/.wash/plugins`.

You can use the `wash install` subcommand to install your newly built plugin:

```bash
wash plugin install oci://ghcr.io/brooksmtownsend/standup_plugin:0.3.0
```

You can confirm your plugin is installed with `wash plugin list`:

```bash
➜ wash plugin list


  Name                          ID                            Version                       Author
  Standup Plugin                standup                       0.3.0                         Brooks
    └ Let wash roll your standup initiative
```

### 2. Use your new plugin with `wash`

This plugin only requires a single `name` argument:

```bash
wash standup Brooks
```

### 3. Include your name

If you want to automatically use your name, you can use [wasi-virt](https://github.com/bytecodealliance/WASI-Virt) to virtualize the environment of your component

```bash
wasi-virt -e STANDUP_NAME=Brooks \
  --allow-env=STANDUP_NAME \
  --allow-http \
  --allow-clocks \
  --allow-exit \
  --allow-stdio \
  --allow-random true \
  ~/.wash/plugins/standup \
  --out ~/.wash/plugins/standup
```
