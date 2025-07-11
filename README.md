# 🐰 Bunny SQL Assistant

**Bunny SQL Assistant** is an open-source CLI tool built with Rust that enables non-technical users to write and execute SQL queries using natural language. It leverages local LLMs like DeepSeek-Coder via [Ollama](https://ollama.com/) to generate SQL queries from prompts in Indonesian or English.

---

## ✨ Features

-  Database configuration (currently supports SQLite)
-  Natural language input (Indonesian or English)
-  Generate and execute SQL directly in the terminal
-  Uses local LLM via Ollama (offline & private)
-  Neatly formatted table output

---

## Installation

### Prerequisites

- Rust >= 1.73
- SQLite (as example database)
- [Ollama](https://ollama.com/) with a local model such as `deepseek-coder` installed

### Clone & Build

```bash
git clone https://github.com/yourname/bunny-sql-assistant.git
cd bunny-sql-assistant
cargo build --release
````

### Global Install (Optional)

```bash
cargo install --path .
```


## Usage

 1. Configure the database connection (e.g. SQLite)

```bash
bunny config sqlite://test.db
```

 2. Run a query using natural language

```bash
bunny query "Show top 3 products by sales"
```

### Sample Output:

```
📜 Generated SQL:
SELECT * FROM produk ORDER BY penjualan DESC LIMIT 3;

+----+------------+-----------+
| id | name       | sales     |
+----+------------+-----------+
| 2  | Smartphone | 300       |
| 8  | Flashdisk  | 250       |
| 3  | Mouse      | 200       |
+----+------------+-----------+
```



## Run with Docker

 1. Build the Docker image

```bash
docker build -t bunny .
```

 2. Run a query

```bash
docker run --rm -v $(pwd)/test.db:/app/test.db bunny query "Show all products"
```



## `.env` Configuration

Create a `.env` file to store default settings:

```env
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=deepseek-coder
```


## Roadmap

* Support for PostgreSQL & MySQL
* SQL validation before execution
* Query logging
* Editable query results
* Plugin system


## 🤝 Contributing

Pull requests are welcome! Feel free to fork the repo, add features or improvements, and open a PR.

---

## 📄 License

MIT License © 2025 - Albany Siswanto
