# 🐰 Bunny SQL Assistant

**Bunny SQL Assistant** is a command-line interface (CLI) tool that transforms natural language commands (in Indonesian or English) into valid SQL queries and executes them directly on your local database. This project is designed to simplify database interactions using everyday language.

Currently, **SQLite** is the supported database backend.

---

## ✨ Key Features

- **Natural Language Conversion**: Transform natural language commands into valid SQL queries.
- **AI Integration**: Powered by the Groq API for intelligent language processing.
- **Direct Execution**: Run SQL queries directly on your local database.
- **Secure Configuration**: Store database connection details securely using a configuration file.

---

## ⚙️ Installation

### Prerequisites

Before you begin, ensure you have installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [SQLite3](https://www.sqlite.org/download.html)
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

Below are the steps to use **Bunny SQL Assistant** after installation:

### 1. Configure the Database
To connect Bunny to a SQLite database, use the following command to set up the connection:

```bash
bunny config sqlite://database_file.db
```

**Example**:
```bash
bunny config sqlite://test.db
```

> **Note**: Ensure the SQLite database file exists or it will be created automatically at the specified location.

### 2. Run Queries with Natural Language
Use the `query` command to translate natural language commands into SQL queries and execute them:

```bash
bunny query "Show all products"
```

**Additional Example**:
```bash
bunny query "Find users older than 25 years"
```

**Output**:
The query will be translated into SQL, for example:
```sql
SELECT * FROM products;
```
or
```sql
SELECT * FROM users WHERE age > 25;
```

> **Tip**: Use clear and specific language for the best results. For example, mention table or column names when necessary.

### 3. View Query History (Optional)
To view the history of executed queries:
```bash
bunny history
```

### 4. Command Help
To see a list of available commands:
```bash
bunny --help
```

---

## 🧠 AI Model Configuration

Bunny SQL Assistant uses the **Groq API** for natural language processing. To use it, configure the API key and AI model via a `.env` file.

### Creating the `.env` File
Create a `.env` file in the project directory with the following content:

```env
GROQ_API_KEY=your_api_key_here
GROQ_MODEL=your_models
```

**Supported Models**:
- `llama3-8b-8192`
- `mixtral-8x7b-32768`
- Other models available at [Groq API](https://console.groq.com/docs/models).

---

## 🧪 Development (Optional)

For developers who want to use **compile-time checking** for SQL queries with SQLx:

1. **Install sqlx-cli**:
   ```bash
   cargo install sqlx-cli --no-default-features --features sqlite
   ```

2. **Generate Query Cache**:
   Ensure `DATABASE_URL` is set in the `.bunny_db_url` file, then run:
   ```bash
   DATABASE_URL=$(cat .bunny_db_url) cargo sqlx prepare
   ```

---

## ⚠️ Limitations

- **Supported**: SQLite as the database backend.
- **Not Yet Supported**:
  - Other databases such as PostgreSQL or MySQL.
  - Complex queries with cross-table joins automatically.

---

## 📄 License

Licensed under the [MIT License](LICENSE) © 2025 [Your Name or Organization].

---

## 🐇 Contribution

We warmly welcome contributions! Please follow these steps to contribute:

1. Fork this repository.
2. Create a new branch (`git checkout -b your-feature`).
3. Make changes and commit (`git commit -m "Add feature X"`).
4. Push to your branch (`git push origin your-feature`).
5. Create a Pull Request on GitHub.

If you find a bug or have a suggestion, please open an [issue](https://github.com/albanysiswanto/bunny-sql-assistant/issues).

---

## 📬 Contact

For questions or support, reach out via [GitHub Issues](https://github.com/albanysiswanto/bunny-sql-assistant/issues) or email at [Albany Siswanto](mailto:albanysiswantoo@gmail.com).

---

Thank you for using **Bunny SQL Assistant**! 🐰
