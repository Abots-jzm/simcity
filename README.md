# Sim City Command LIne Simulator

## Project Description
This project is a simulation tool designed to model the development of an urban area over time. The simulation adheres to specific zoning rules and outputs various data about the urban region, including population distribution, pollution levels, and regional analysis.

---

## Compilation Instructions
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Navigate to the project directory.
3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
   This will create an optimized executable in the `target/release` directory.

---

## Execution Instructions
1. Run the compiled program using Cargo or the generated executable:
   ```bash
   cargo run --release
   ```
   Or:
   ```bash
   ./target/release/urban_simulation
   ```
2. When prompted, input the filename containing the simulation configuration (ensure the file exists in the same directory or provide the full path).
3. Follow the prompts to input required parameters, including a rectangular area for detailed analysis.

---

## Simulation Features
- **Initial Setup**: Reads and stores simulation configuration and initial region layout from input files.
- **Zoning Simulation**:
  - **Residential Zones**: Follows rules for population growth based on adjacency to powerlines and other cells.
  - **Industrial Zones**: Simulates worker allocation, pollution generation, and goods production.
  - **Commercial Zones**: Handles worker and goods allocation for population growth.
- **Pollution Tracking**: Monitors pollution spread and calculates total pollution.
- **Dynamic Output**: Displays region states at specified intervals.
- **Analysis Tool**: Allows detailed analysis of a user-specified rectangular region.

---

## Output Overview
1. Initial state of the region at time step 0.
2. Periodic state outputs at the refresh rate specified in the configuration.
3. Final simulation results, including:
   - Regional population (residential, industrial, commercial).
   - Regional and area-specific pollution levels.
4. Analysis results for a user-defined area.
