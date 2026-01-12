# JobWeaver-RS

Control-M XML analyzer for Airflow migration planning written in Rust.

## Overview

JobWeaver-RS analyzes Control-M XML exports to assess job complexity and determine optimal migration strategies to Apache Airflow. It provides detailed analysis reports in multiple formats (JSON, CSV, HTML, Markdown) to help plan and execute Control-M to Airflow migrations.

## Features

- 📊 **Complexity Analysis**: Calculates complexity scores based on dependencies, conditions, resources, and scheduling patterns
- 🎯 **Migration Difficulty Assessment**: Categorizes jobs as Easy, Medium, or Hard to migrate
- 📈 **Migration Wave Planning**: Groups jobs into migration waves based on complexity and dependencies
- 🔍 **Dependency Analysis**: Builds dependency graphs and detects circular dependencies
- 📄 **Multiple Output Formats**: Generates reports in JSON, CSV, HTML, and Markdown
- 🏗️ **Clean Architecture**: Built with domain-driven design principles
- ✅ **Comprehensive Testing**: Unit tests for all core components

## Architecture

The project follows Clean Architecture principles:

```
src/
├── domain/              # Business logic and entities
│   ├── entities/        # Core domain models (Job, Folder, etc.)
│   ├── value_objects/   # Value objects (ComplexityScore, etc.)
│   └── repositories/    # Repository interfaces
├── application/         # Use cases and services
│   ├── use_cases/       # Application use cases
│   └── services/        # Domain services
├── infrastructure/      # External adapters
│   ├── parsers/         # XML parsing
│   ├── repositories/    # Repository implementations
│   └── output/          # Report generators
└── presentation/        # CLI and DTOs
    ├── cli/             # Command-line interface
    └── dto/             # Data transfer objects
```

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Build from Source

```bash
git clone <repository-url>
cd jobweaver-rs
cargo build --release
```

The binary will be available at `target/release/jobweaver`.

## Usage

### Basic Usage

```bash
jobweaver -i datasource/export_xml_260109.xml -o output
```

### Options

```
Options:
  -i, --input <FILE>      Path to Control-M XML export file
  -o, --output <DIR>      Output directory for reports [default: output]
  -f, --format <FORMAT>   Output format: json, csv, html, markdown, all [default: all]
  -v, --verbose           Enable verbose logging
  -h, --help              Print help
  -V, --version           Print version
```

### Examples

Generate all report formats:
```bash
jobweaver -i input.xml -o reports
```

Generate only JSON report:
```bash
jobweaver -i input.xml -o reports -f json
```

Generate CSV and HTML reports with verbose logging:
```bash
jobweaver -i input.xml -o reports -f csv -v
```

## Output Formats

### JSON Report (`analysis.json`)
Detailed analysis with all metrics, suitable for programmatic processing.

### CSV Report (`analysis.csv`)
Tabular format for easy import into spreadsheets and databases.

### HTML Report (`analysis.html`)
Interactive web-based report with styling and formatting.

### Markdown Report (`analysis.md`)
Human-readable documentation format with tables and sections.

## Complexity Scoring

The complexity score is calculated based on:

- **Dependencies** (3 points each): In-conditions and control resources
- **Dependency Depth** (5 points per level): Depth of dependency chain
- **Conditions** (2 points each): In/out conditions
- **Variables** (1 point each): Job variables and auto-edits
- **ON Conditions** (4 points each + complexity): Conditional logic
- **Cyclic Jobs** (15 points): Jobs with cyclic execution
- **Resources** (3 points each): Quantitative and control resources
- **Scheduling** (2 points per feature): Calendars, time windows, etc.

### Migration Difficulty Levels

- **Easy** (0-30): Simple jobs with minimal dependencies
- **Medium** (31-60): Moderate complexity with some dependencies
- **Hard** (61+): Complex jobs requiring careful planning

## Migration Waves

Jobs are automatically grouped into migration waves:

1. **Wave 1**: Low complexity, no dependencies (Quick wins)
2. **Wave 2**: Low to medium complexity, minimal dependencies
3. **Wave 3**: Medium complexity or critical jobs
4. **Wave 4**: Medium complexity with dependencies
5. **Wave 5**: High complexity requiring careful planning

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
cargo run -- -i input.xml -o output -v
```

### Code Coverage

```bash
cargo tarpaulin --out Html
```

## Project Structure

```
jobweaver-rs/
├── Cargo.toml           # Project dependencies
├── README.md            # This file
├── src/                 # Source code
├── datasource/          # Sample Control-M XML files
└── tests/               # Integration tests
```

## Dependencies

- **quick-xml**: XML parsing
- **serde**: Serialization/deserialization
- **clap**: Command-line argument parsing
- **petgraph**: Dependency graph analysis
- **csv**: CSV generation
- **tera**: HTML templating
- **anyhow**: Error handling
- **tracing**: Logging

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

See LICENSE file for details.

## Support

For issues and questions, please open an issue on the GitHub repository.

## Roadmap

- [ ] Support for Control-M SSD files
- [ ] Advanced dependency visualization
- [ ] Airflow DAG code generation
- [ ] Integration with Airflow REST API
- [ ] Support for multiple Control-M versions
- [ ] Performance optimization for large XML files
- [ ] Web UI for interactive analysis

## Authors

JobWeaver Team

## Acknowledgments

- Control-M documentation
- Apache Airflow community
- Rust community
