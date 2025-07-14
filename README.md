# 🐰 Bunny SQL Assistant

**Bunny SQL Assistant** is a command-line interface (CLI) tool that transforms natural language commands (in Indonesian or English) into valid SQL queries and executes them directly on your local database. This project is designed to simplify database interactions using everyday language.

---

## ✨ Key Features

- **Natural Language Conversion**: Transform natural language commands into valid SQL queries
- **Multi-Database Support**: Works with both SQLite and PostgreSQL
- **AI Integration**: Powered by the Groq API for intelligent language processing
- **Direct Execution**: Run SQL queries directly on your local database
- **Secure Configuration**: Store database connection details securely using configuration files
- **Rich Output Formatting**: Beautifully formatted table outputs for query results

---

## ⚙️ Installation

### Prerequisites

Before you begin, ensure you have installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [SQLite3](https://www.sqlite.org/download.html) (for SQLite support)
- [PostgreSQL](https://www.postgresql.org/download/) (for PostgreSQL support)
- An API key from [Groq](https://console.groq.com/keys)

### Installation Steps

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/bunny-sql-assistant.git
   cd bunny-sql-assistant
   ```

2. **Build the Project**:
   ```bash
   cargo build --release
   ```

3. **Verify Installation**:
   Ensure the `bunny` binary is available in the `./target/release/` directory.

---

## 🚀 Usage

### 1. Configure the Database Connection

#### For SQLite:
```bash
bunnysql config --db-type sqlite://database_file.db
```
**Example**:
```bash
bunnysql config --db-type sqlite://test.db
```

#### For PostgreSQL:
```bash
bunnysql config --db-type postgres://username:password@localhost:5432/database_name
```
**Example**:
```bash
bunnysql config --db-type postgres://postgres:bunny@localhost:5432/mydb
```

> **Note**: Database files will be created automatically for SQLite. For PostgreSQL, ensure the database exists.

### 2. Run Queries with Natural Language

Basic query format:
```bash
bunnysql query "Your natural language query"
```

**SQLite Examples**:
```bash
bunnysql query "Show all products"
bunnysql query "Find users older than 25 years"
```

**PostgreSQL Examples**:
```bash
bunnysql query "Show top 5 customers by total purchases"
bunnysql query "List all orders from last month"
```

### 3. Get Help
```bash
bunnysql --help
```

---

## 🧠 AI Model Configuration

Configure the Groq API by creating a `.env` file:

```env
GROQ_API_KEY=your_api_key_here
GROQ_MODEL=mixtral-8x7b-32768  # or other supported models
```

**Supported Models**:
- `llama3-8b-8192`
- `mixtral-8x7b-32768`
- Other models available at [Groq API](https://console.groq.com/docs/models)

---

## 🧪 Development

### For SQLite Development:
```bash
cargo install sqlx-cli --no-default-features --features sqlite
DATABASE_URL=sqlite://test.db cargo sqlx prepare
```

### For PostgreSQL Development:
```bash
cargo install sqlx-cli --no-default-features --features postgres
DATABASE_URL=postgres://user:pass@localhost:5432/db cargo sqlx prepare
```

---

## ⚠️ Limitations

- **Supported Databases**:
  - SQLite
  - PostgreSQL
- **Query Complexity**: Very complex queries may require manual refinement
- **Connection Limits**: Ensure your PostgreSQL server allows connections

---

## 📄 License

MIT License © 2025 Albany Siswanto

---

## 🐇 Contribution

We welcome contributions! Please:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

Report issues on our [GitHub Issues](https://github.com/albanysiswanto/bunny-sql-assistant/issues) page.

---

## 📬 Contact

For questions or support:
- [GitHub Issues](https://github.com/albanysiswanto/bunny-sql-assistant/issues)
- Email: [albanysiswantoo@gmail.com](mailto:albanysiswantoo@gmail.com)

---

Thank you for using **Bunny SQL Assistant**! 🐰
