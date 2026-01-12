# JobWeaver-RS

[🇬🇧 English](#english) | [🇹🇭 ภาษาไทย](#ภาษาไทย)

---

## English

### Overview

JobWeaver-RS is a Control-M XML analyzer for assessing job complexity and determining optimal migration strategies to Apache Airflow. It provides detailed analysis reports in multiple formats (JSON, CSV, HTML, Markdown) to help plan and execute Control-M to Airflow migrations.

Built with Rust using Clean Architecture principles.

### Features

- 📊 **Complexity Analysis**: Calculates complexity scores based on dependencies, conditions, resources, and scheduling patterns
- 🎯 **Migration Difficulty Assessment**: Categorizes jobs as Easy, Medium, or Hard to migrate
- 📈 **Migration Wave Planning**: Groups jobs into migration waves based on complexity and dependencies
- 🔍 **Dependency Analysis**: Builds dependency graphs and detects circular dependencies
- 📄 **Multiple Output Formats**: Generates reports in JSON, CSV, HTML, and Markdown
- 🏗️ **Clean Architecture**: Built with domain-driven design principles
- ✅ **Comprehensive Testing**: Unit tests for all core components

### Architecture

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

### Installation

#### Prerequisites

- Rust 1.70 or higher
- Cargo

#### Build from Source

```bash
git clone <repository-url>
cd jobweaver-rs
cargo build --release
```

The binary will be available at `target/release/jobweaver`

### Usage

#### Basic Usage

```bash
jobweaver -i datasource/export_xml_260109.xml -o output
```

#### Options

```
Options:
  -i, --input <FILE>      Path to Control-M XML export file
  -o, --output <DIR>      Output directory for reports [default: output]
  -f, --format <FORMAT>   Output format: json, csv, html, markdown, all [default: all]
  -v, --verbose           Enable verbose logging
  -h, --help              Print help
  -V, --version           Print version
```

#### Examples

Generate all report formats:
```bash
jobweaver -i input.xml -o reports
```

Generate only JSON report:
```bash
jobweaver -i input.xml -o reports -f json
```

Generate CSV report with verbose logging:
```bash
jobweaver -i input.xml -o reports -f csv -v
```

### Output Formats

#### JSON Report (`analysis.json`)
Detailed analysis with all metrics, suitable for programmatic processing.

#### CSV Report (`analysis.csv`)
Tabular format for easy import into spreadsheets and databases.

#### HTML Report (`analysis.html`)
Interactive web-based report with styling and formatting.

#### Markdown Report (`analysis.md`)
Human-readable documentation format with tables and sections.

### Complexity Scoring

The complexity score is calculated based on:

- **Dependencies** (3 points each): In-conditions and control resources
- **Dependency Depth** (5 points per level): Depth of dependency chain
- **Conditions** (2 points each): In/out conditions
- **Variables** (1 point each): Job variables and auto-edits
- **ON Conditions** (4 points each + complexity): Conditional logic
- **Cyclic Jobs** (15 points): Jobs with cyclic execution
- **Resources** (3 points each): Quantitative and control resources
- **Scheduling** (2 points per feature): Calendars, time windows, etc.

#### Migration Difficulty Levels

- **Easy** (0-30): Simple jobs with minimal dependencies
- **Medium** (31-60): Moderate complexity with some dependencies
- **Hard** (61+): Complex jobs requiring careful planning

### Migration Waves

Jobs are automatically grouped into migration waves:

1. **Wave 1**: Low complexity, no dependencies (Quick wins)
2. **Wave 2**: Low to medium complexity, minimal dependencies
3. **Wave 3**: Medium complexity or critical jobs
4. **Wave 4**: Medium complexity with dependencies
5. **Wave 5**: High complexity requiring careful planning

### Development

#### Running Tests

```bash
cargo test
```

#### Running with Debug Logging

```bash
cargo run -- -i input.xml -o output -v
```

#### Code Coverage

```bash
cargo tarpaulin --out Html
```

### Project Structure

```
jobweaver-rs/
├── Cargo.toml           # Project dependencies
├── README.md            # This file
├── src/                 # Source code
├── datasource/          # Sample Control-M XML files
└── tests/               # Integration tests
```

### Key Dependencies

- **roxmltree**: XML parsing
- **serde**: Serialization/deserialization
- **clap**: Command-line argument parsing
- **petgraph**: Dependency graph analysis
- **csv**: CSV generation
- **tera**: HTML templating
- **anyhow**: Error handling
- **tracing**: Logging
- **encoding_rs**: Multi-encoding support

### Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### License

See LICENSE file for details.

### Support

For issues and questions, please open an issue on the GitHub repository.

### Roadmap

- [ ] Support for Control-M SSD files
- [ ] Advanced dependency visualization
- [ ] Airflow DAG code generation
- [ ] Integration with Airflow REST API
- [ ] Support for multiple Control-M versions
- [ ] Performance optimization for large XML files
- [ ] Web UI for interactive analysis

### Authors

JobWeaver Team

### Acknowledgments

- Control-M documentation
- Apache Airflow community
- Rust community

---

## ภาษาไทย

### ภาพรวม

JobWeaver-RS เป็นเครื่องมือวิเคราะห์ไฟล์ XML จาก Control-M เพื่อประเมินความซับซ้อนของ Job และกำหนดกลยุทธ์การย้ายข้อมูลไปยัง Apache Airflow ที่เหมาะสม โปรแกรมสร้างรายงานการวิเคราะห์ในหลายรูปแบบ (JSON, CSV, HTML, Markdown) เพื่อช่วยในการวางแผนและดำเนินการย้ายข้อมูลจาก Control-M ไปยัง Airflow

พัฒนาด้วยภาษา Rust โดยใช้หลักการ Clean Architecture

### ฟีเจอร์หลัก

- 📊 **วิเคราะห์ความซับซ้อน**: คำนวณคะแนนความซับซ้อนจาก dependencies, conditions, resources และรูปแบบการ schedule
- 🎯 **ประเมินความยากในการย้าย**: จัดกลุ่ม Job เป็น Easy, Medium หรือ Hard ตามความยากในการย้าย
- 📈 **วางแผน Migration Wave**: จัดกลุ่ม Job เป็น wave ตามความซับซ้อนและ dependencies
- 🔍 **วิเคราะห์ Dependency**: สร้าง dependency graph และตรวจจับ circular dependencies
- 📄 **รายงานหลายรูปแบบ**: สร้างรายงานในรูปแบบ JSON, CSV, HTML และ Markdown
- 🏗️ **Clean Architecture**: พัฒนาตามหลัก Domain-Driven Design
- ✅ **Unit Tests**: ครอบคลุมทุก component หลัก

### สถาปัตยกรรม

โปรเจคใช้หลักการ Clean Architecture:

```
src/
├── domain/              # Business logic และ entities
│   ├── entities/        # Core domain models (Job, Folder, ฯลฯ)
│   ├── value_objects/   # Value objects (ComplexityScore, ฯลฯ)
│   └── repositories/    # Repository interfaces
├── application/         # Use cases และ services
│   ├── use_cases/       # Application use cases
│   └── services/        # Domain services
├── infrastructure/      # External adapters
│   ├── parsers/         # XML parsing
│   ├── repositories/    # Repository implementations
│   └── output/          # Report generators
└── presentation/        # CLI และ DTOs
    ├── cli/             # Command-line interface
    └── dto/             # Data transfer objects
```

### การติดตั้ง

#### ความต้องการของระบบ

- Rust 1.70 ขึ้นไป
- Cargo

#### Build จาก Source Code

```bash
git clone <repository-url>
cd jobweaver-rs
cargo build --release
```

ไฟล์ binary จะอยู่ที่ `target/release/jobweaver`

### วิธีใช้งาน

#### การใช้งานพื้นฐาน

```bash
jobweaver -i datasource/export_xml_260109.xml -o output
```

#### ตัวเลือก (Options)

```
Options:
  -i, --input <FILE>      ไฟล์ XML ที่ export จาก Control-M
  -o, --output <DIR>      โฟลเดอร์สำหรับเก็บรายงาน [default: output]
  -f, --format <FORMAT>   รูปแบบรายงาน: json, csv, html, markdown, all [default: all]
  -v, --verbose           แสดง log แบบละเอียด
  -h, --help              แสดงวิธีใช้งาน
  -V, --version           แสดงเวอร์ชัน
```

#### ตัวอย่างการใช้งาน

สร้างรายงานทุกรูปแบบ:
```bash
jobweaver -i input.xml -o reports
```

สร้างเฉพาะรายงาน JSON:
```bash
jobweaver -i input.xml -o reports -f json
```

สร้างรายงาน CSV พร้อม verbose logging:
```bash
jobweaver -i input.xml -o reports -f csv -v
```

### รูปแบบรายงาน

#### JSON Report (`analysis.json`)
รายงานแบบละเอียดพร้อม metrics ทั้งหมด เหมาะสำหรับการประมวลผลด้วยโปรแกรม

#### CSV Report (`analysis.csv`)
รูปแบบตาราง นำเข้า spreadsheet หรือ database ได้ง่าย

#### HTML Report (`analysis.html`)
รายงานแบบ web พร้อม styling และการจัดรูปแบบ

#### Markdown Report (`analysis.md`)
รูปแบบเอกสารที่อ่านง่าย มีตารางและหัวข้อแบ่งส่วน

### การคำนวณคะแนนความซับซ้อน

คะแนนความซับซ้อนคำนวณจาก:

- **Dependencies** (3 คะแนน/รายการ): In-conditions และ control resources
- **Dependency Depth** (5 คะแนน/ระดับ): ความลึกของ dependency chain
- **Conditions** (2 คะแนน/รายการ): In/out conditions
- **Variables** (1 คะแนน/รายการ): Job variables และ auto-edits
- **ON Conditions** (4 คะแนน/รายการ + ความซับซ้อน): Conditional logic
- **Cyclic Jobs** (15 คะแนน): Jobs ที่ทำงานแบบ cyclic
- **Resources** (3 คะแนน/รายการ): Quantitative และ control resources
- **Scheduling** (2 คะแนน/feature): Calendars, time windows ฯลฯ

#### ระดับความยากในการย้าย

- **Easy** (0-30): Jobs ง่าย มี dependencies น้อย
- **Medium** (31-60): ความซับซ้อนปานกลาง มี dependencies บ้าง
- **Hard** (61+): Jobs ซับซ้อน ต้องวางแผนอย่างรอบคอบ

### Migration Waves

Jobs จะถูกจัดกลุ่มเป็น migration waves อัตโนมัติ:

1. **Wave 1**: ความซับซ้อนต่ำ ไม่มี dependencies (Quick wins)
2. **Wave 2**: ความซับซ้อนต่ำถึงปานกลาง มี dependencies น้อย
3. **Wave 3**: ความซับซ้อนปานกลาง หรือ critical jobs
4. **Wave 4**: ความซับซ้อนปานกลาง มี dependencies
5. **Wave 5**: ความซับซ้อนสูง ต้องวางแผนอย่างรอบคอบ

### การพัฒนา

#### รัน Tests

```bash
cargo test
```

#### รันพร้อม Debug Logging

```bash
cargo run -- -i input.xml -o output -v
```

#### ตรวจสอบ Code Coverage

```bash
cargo tarpaulin --out Html
```

### โครงสร้างโปรเจค

```
jobweaver-rs/
├── Cargo.toml           # Dependencies ของโปรเจค
├── README.md            # ไฟล์นี้
├── src/                 # Source code
├── datasource/          # ไฟล์ตัวอย่าง Control-M XML
└── tests/               # Integration tests
```

### Dependencies หลัก

- **roxmltree**: XML parsing
- **serde**: Serialization/deserialization
- **clap**: Command-line argument parsing
- **petgraph**: Dependency graph analysis
- **csv**: CSV generation
- **tera**: HTML templating
- **anyhow**: Error handling
- **tracing**: Logging
- **encoding_rs**: รองรับ encoding หลายแบบ

### การมีส่วนร่วม

ยินดีรับ contributions! กรุณาปฏิบัติตามแนวทางนี้:

1. Fork repository
2. สร้าง feature branch
3. เขียน tests สำหรับ functionality ใหม่
4. ตรวจสอบให้ tests ทั้งหมดผ่าน
5. ส่ง pull request

### License

ดูรายละเอียดในไฟล์ LICENSE

### การสนับสนุน

สำหรับปัญหาและคำถาม กรุณาเปิด issue ใน GitHub repository

### Roadmap

- [ ] รองรับไฟล์ Control-M SSD
- [ ] Dependency visualization ขั้นสูง
- [ ] สร้างโค้ด Airflow DAG อัตโนมัติ
- [ ] เชื่อมต่อกับ Airflow REST API
- [ ] รองรับ Control-M หลายเวอร์ชัน
- [ ] เพิ่มประสิทธิภาพสำหรับไฟล์ XML ขนาดใหญ่
- [ ] Web UI สำหรับวิเคราะห์แบบ interactive

### ผู้พัฒนา

JobWeaver Team

### กิตติกรรมประกาศ

- เอกสาร Control-M
- ชุมชน Apache Airflow
- ชุมชน Rust
