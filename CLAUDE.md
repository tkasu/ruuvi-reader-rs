# ruuvi-reader-rs

## Overview

**ruuvi-reader-rs** is a Rust-based Bluetooth Low Energy (BLE) scanner that reads sensor data from physical Ruuvi Tags and outputs the readings as newline-delimited JSON. This is the data ingestion component of the Ruuvitag telemetry ecosystem.

**Status:** Work in Progress (WIP)

**Language:** Rust (Edition 2021)

## Purpose

This utility serves as the data source in the telemetry pipeline by:
- Scanning for BLE advertisements from Ruuvi Tag sensors
- Parsing the Ruuvi-specific manufacturer data protocol
- Outputting standardized JSON telemetry data to stdout
- Providing real-time sensor readings including temperature, humidity, pressure, battery level, and motion data

## Architecture

### Tech Stack

- **Runtime:** Tokio (async multi-threaded)
- **BLE Library:** btleplug v0.11.0 (cross-platform Bluetooth LE)
- **Protocol Parser:** ruuvi-sensor-protocol v0.6.1
- **Serialization:** serde + serde_json
- **Async Utilities:** futures

### Project Structure

```
ruuvi-reader-rs/
├── src/
│   ├── main.rs           # Entry point - starts the event loop
│   ├── lib.rs            # Module declarations
│   └── scanner.rs        # Core BLE scanning logic (79 lines)
├── Cargo.toml            # Package manifest and dependencies
├── Cargo.lock            # Locked dependency versions
└── README.md             # Basic usage documentation
```

### Key Components

**scanner.rs:79** - Core scanning module
- `read_events()` - Main async function that:
  1. Initializes Bluetooth manager via btleplug
  2. Retrieves the first available BLE adapter
  3. Starts BLE scanning with default filter
  4. Listens for manufacturer-specific data events
  5. Parses Ruuvi sensor protocol data
  6. Serializes to JSON and prints to stdout

- `get_central()` - Helper to get first available Bluetooth adapter

**main.rs** - Minimal entry point that calls `scanner::read_events()`

## Data Format

Each line output is a JSON object with the following schema:

```json
{
  "mac_address": [213, 18, 52, 102, 20, 20],
  "humidity": 570925,
  "temperature_millicelsius": 22005,
  "pressure": 100621,
  "battery_potential": 1941,
  "tx_power": 4,
  "movement_counter": 79,
  "measurement_sequence_number": 559,
  "measurement_ts_ms": 1693460275133
}
```

**Field Descriptions:**
- `mac_address`: 6-byte array representing the Ruuvi Tag's MAC address
- `temperature_millicelsius`: Temperature in millidegrees Celsius (divide by 1000 for °C)
- `humidity`: Relative humidity in 0.0001% units
- `pressure`: Atmospheric pressure in Pa
- `battery_potential`: Battery voltage in mV
- `tx_power`: Bluetooth transmission power in dBm
- `movement_counter`: Accelerometer-based motion counter
- `measurement_sequence_number`: Sequential measurement ID
- `measurement_ts_ms`: Timestamp in milliseconds since epoch

## Building and Running

### Prerequisites

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Platform-Specific Bluetooth Requirements**
   - **macOS:** Grant Bluetooth permissions in System Settings > Privacy & Security
   - **Linux:** Install bluez libraries (`libbluetooth-dev` on Debian/Ubuntu)
   - **Windows:** Bluetooth LE support built into Windows 10+

### Build Commands

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Install globally to ~/.cargo/bin
cargo install --path .
```

### Running

```bash
# Run from target directory
./target/release/ruuvi-reader-rs

# Or if installed globally
ruuvi-reader-rs

# Pipe output to file
ruuvi-reader-rs > sensor-data.jsonl

# Pipe to data forwarder (integration)
ruuvi-reader-rs | java -jar ../ruuvi-data-forwarder/target/scala-3.3.0/ruuvi-data-forwarder-assembly-*.jar
```

### Testing

Currently no automated tests. Manual testing:
```bash
cargo run
# Verify JSON output appears when Ruuvi Tags are in range
```

## Configuration

**Zero Configuration Required**
- Automatically detects and uses the first available Bluetooth adapter
- Scans for all Ruuvi Tags in range without filtering
- No config files or environment variables needed

## Integration

This project is designed to integrate via Unix pipes:

```bash
# Basic pipeline
ruuvi-reader-rs | tee raw-data.jsonl | <processing-tool>

# With data forwarder
ruuvi-reader-rs | java -jar ruuvi-data-forwarder-assembly.jar

# Filter by jq
ruuvi-reader-rs | jq 'select(.temperature_millicelsius > 20000)'
```

## Development Notes

### Recent Changes (Git History)
- `f3b34b5` - Add measurement_ts_ms field for timestamping
- `4887805` - Add installation instructions to README
- `f31654b` - Initial proof of concept

### Known Limitations
- No filtering by MAC address (receives all Ruuvi Tags)
- No error recovery - crashes on Bluetooth errors
- Single adapter only - doesn't support multiple BLE adapters
- No retry logic for transient BLE failures

### Future Enhancements
- MAC address filtering via CLI arguments
- Configuration file support
- Better error handling and recovery
- Support for multiple adapters
- Signal handling for graceful shutdown
- Metrics (tags discovered, read rate, errors)

## Dependencies

Key dependencies from Cargo.toml:

| Dependency | Version | Purpose |
|------------|---------|---------|
| tokio | latest | Async runtime with multi-threading |
| btleplug | 0.11.0 | Cross-platform BLE library |
| ruuvi-sensor-protocol | 0.6.1 | Ruuvi data format parser |
| serde | 1.0 | Serialization framework |
| serde_json | latest | JSON serialization |
| futures | latest | Async utilities |

## Troubleshooting

**Issue: No output appears**
- Ensure Bluetooth is enabled
- Check that Ruuvi Tags are powered and in range (within ~10m)
- Verify Bluetooth permissions granted to terminal/application

**Issue: Permission denied (macOS)**
- Go to System Settings > Privacy & Security > Bluetooth
- Grant permission to Terminal or your terminal application

**Issue: Compilation errors with btleplug**
- Ensure platform-specific Bluetooth libraries are installed
- On Linux: `sudo apt-get install libbluetooth-dev pkg-config`

**Issue: "No adapters found"**
- Ensure Bluetooth adapter is enabled and detected by OS
- Try `bluetoothctl` (Linux) or System Settings (macOS) to verify

## Related Projects

- **ruuvi-data-forwarder** - Downstream consumer that processes and routes telemetry
- **ruuvi-api** - REST API for serving telemetry data

## Resources

- [Ruuvi Tag Documentation](https://docs.ruuvi.com/)
- [Ruuvi Sensor Protocol](https://github.com/ruuvi/ruuvi-sensor-protocols)
- [btleplug Documentation](https://github.com/deviceplug/btleplug)
