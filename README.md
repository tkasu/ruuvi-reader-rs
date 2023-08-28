# Ruuvi Data Reader

This program is WIP simple utility that reads data from Ruuvi Tags and outputs it as json.

The goal is to have lightweight program that:

1. Works cross-platform, and can therefore be developed and debugged on MacOS but deployed to Raspberry PI
2. Outputs general format, that can ve easily piped to other programs

## Requirements

- Rust toolchain (https://rustup.rs/)
- See btleplug [OS-spesific requirements](https://github.com/deviceplug/btleplug#buildinstallation-notes-for-specific-platforms) (e.g. required MacOS privacy settings)

## Example usage

### Ad-hoc

```shell
cargo build --release
```

```shell
./target/release/ruuvi-reader-rs
```

### Install to path

```shell
cargo install --path .
```

```shell
ruuvi-reader-rs
```

### Ouput format

Example output:

```
{"battery_potential":1941,"humidity":570925,"mac_address":[213,18,52,102,20,20],"measurement_sequence_number":559,"movement_counter":79,"pressure":100621,"temperature_millicelsius":22005,"tx_power":4}
{"battery_potential":2347,"humidity":657350,"mac_address":[254,38,136,122,102,102],"measurement_sequence_number":51235,"movement_counter":2,"pressure":100817,"temperature_millicelsius":-28260,"tx_power":4}
{"battery_potential":2347,"humidity":657350,"mac_address":[254,38,136,122,102,102],"measurement_sequence_number":51238,"movement_counter":2,"pressure":100817,"temperature_millicelsius":-28260,"tx_power":4}
```
