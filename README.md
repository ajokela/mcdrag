# MCDRAG - Drag Coefficient Estimation

A Rust implementation of the MCDRAG algorithm (December 1974, R. L. McCoy) for estimating drag coefficients of axisymmetric projectiles. This project includes both a native CLI application and a WebAssembly version with a web-based terminal interface.

## Features

- Calculate drag coefficients across Mach numbers from 0.5 to 5.0
- Support for three boundary layer models (Laminar/Laminar, Laminar/Turbulent, Turbulent/Turbulent)
- Detailed component breakdown (CD0, CDH, CDSF, CDBND, CDBT, CDB)
- Base pressure ratio calculations
- Diagnostic warnings for problematic geometries
- Web-based terminal interface using WebAssembly

## Native CLI Usage

### Build and Run
```bash
cargo build --release
cargo run
```

The program will interactively prompt for:
- Reference diameter (mm)
- Total length (calibers)
- Nose length (calibers)
- RT/R headshape parameter
- Boattail length (calibers)
- Base diameter (calibers)
- Meplat diameter (calibers)
- Rotating band diameter (calibers)
- Center of gravity location (optional)
- Boundary layer code (L/L, L/T, or T/T)
- Projectile identification

## Web Version

### Prerequisites
- Rust toolchain
- wasm-pack (will be installed automatically by build script)
- Python 3 (for the development server)

### Build and Run
```bash
# Build the WASM module
./build_wasm.sh

# Start the web server
python3 serve.py

# Open in browser
# Navigate to http://localhost:8000/index.html
```

### Web Terminal Commands
- `start` - Begin new projectile calculation
- `clear` - Clear the terminal
- `help` - Show available commands
- `exit` - Exit the program

## Project Structure
```
mcdrag/
├── src/
│   ├── main.rs      # Native CLI application
│   └── lib.rs       # WASM library with core calculations
├── Cargo.toml       # Rust dependencies
├── index.html       # Web terminal interface
├── build_wasm.sh    # WASM build script
└── README.md        # This file
```

## Technical Details

The implementation calculates various drag components:
- **CD0**: Total drag coefficient
- **CDH**: Head drag coefficient
- **CDSF**: Skin friction drag coefficient
- **CDBND**: Rotating band drag coefficient
- **CDBT**: Boattail drag coefficient
- **CDB**: Base drag coefficient
- **PB/PINF**: Base pressure ratio

The algorithm accounts for different flow regimes (subsonic, transonic, supersonic) and provides diagnostic warnings for:
- Nose too short or blunt
- Boattail too long or steep
- Conical flare tail issues

## Licensing

The original MCDRAG BASIC program (included as `mcdrag.txt`) is **Public Domain** software by R. L. McCoy (December 1974).

This Rust translation and web implementation is licensed under the **BSD 3-Clause License** (see LICENSE file).