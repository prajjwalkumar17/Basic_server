<div align="center">

# 🔮 Basic Server

```
 ██████╗  █████╗ ███████╗███████╗███████╗███████╗ █████╗ ████████╗███████╗██████╗ 
 ██╔════╝ ██╔══██╗██╔════╝██╔════╝██╔════╝██╔════╝██╔══██╗╚══██╔══╝██╔════╝██╔══██╗
 ██║  ███╗███████║███████╗███████╗███████╗███████╗███████║   ██║   █████╗  ██████╔╝
 ██║   ██║██╔══██║╚════██║╚════██║╚════██║╚════██║██╔══██║   ██║   ██╔══╝  ██╔══██╗
 ╚██████╔╝██║  ██║███████║███████║███████║███████║██║  ██║   ██║   ███████╗██║  ██║
  ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝
```

### *A minimal HTTP/1.1 server built from scratch in Rust*

**No frameworks. No dependencies. Just pure `std`.**

[![Rust Edition](https://img.shields.io/badge/rust-2021-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-green?style=for-the-badge&logo=none)](Cargo.toml)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=for-the-badge)](https://www.rust-lang.org/)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🚀 Core Capabilities

- ✅ **HTTP/1.1 Compliant** — Full request parsing with method, path, query string validation
- ✅ **Zero Dependencies** — Built entirely on Rust's `std` library
- ✅ **Static File Serving** — Automatic MIME-type detection
- ✅ **Security First** — Directory traversal protection built-in
- ✅ **Custom Routing** — Flexible endpoint handling

</td>
<td width="50%">

### 🛠️ Developer Experience

- ✅ **Hot Reload Ready** — Configure custom public paths via env vars
- ✅ **Clean Architecture** — Modular codebase with separation of concerns
- ✅ **Well Tested** — Automated curl test suite included
- ✅ **Async-Ready** — Foundation ready for async upgrades
- ✅ **Educational** — Perfect for learning HTTP internals

</td>
</tr>
</table>

---

## ⚡ Quick Start

> **💡 New to Rust?** Install it from [rust-lang.org](https://rust-lang.org/tools/install)

```bash
# Clone and run in seconds
$ git clone https://github.com/prajjwalkumar17/Basic_server.git
$ cd Basic_server
$ cargo run
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/basic_server`
🔮 Server listening on http://127.0.0.1:8080
```

**That's it!** Your server is now serving requests. 🎉

---

## 🎯 API Endpoints

| Method | Endpoint | Description | Response |
|:------:|:--------:|-------------|:--------:|
| `GET` | `/` | Homepage | 📄 `index.html` |
| `GET` | `/hello` | Greeting endpoint | 💬 `200 OK` |
| `GET` | `/hello?name=You` | Personalized greeting | 💬 `Hello, You!` |
| `GET` | `/*` | Static files | 📁 `200/404` |
| `*` | `/*` | Other methods | ❌ `404` |

### 🖥️ Try It Live

```bash
# 🏠 Homepage
$ curl http://127.0.0.1:8080/

# 👋 Hello endpoint  
$ curl http://127.0.0.1:8080/hello

# 🎨 Static assets
$ curl http://127.0.0.1:8080/style.css

# 📝 Query string magic
$ curl "http://127.0.0.1:8080/hello?name=World&greeting=Hola"

# ❓ Missing routes → 404
$ curl -i http://127.0.0.1:8080/nowhere
HTTP/1.1 404 Not Found
```

---

## 🏗️ Project Architecture

```
📦 Basic_server
├── 📄 Cargo.toml              # Workspace configuration
├── 📁 crates/
│   ├── 📁 basic_server/       # Binary entry point
│   │   └── 📄 main.rs         # Server bootstrap
│   └── 📁 basic_server_lib/   # Core library
│       ├── 📄 server.rs       # TCP listener & connections
│       ├── 📄 website_handler.rs  # Route logic
│       └── 📁 http/           # HTTP protocol implementation
│           ├── 📄 method.rs       # GET, POST, etc.
│           ├── 📄 request.rs      # Request parsing
│           ├── 📄 response.rs     # Response building
│           ├── 📄 query_string.rs # Query parsing
│           └── 📄 status_code.rs  # HTTP status codes
├── 📁 public/                 # Static file root
│   ├── 📄 index.html
│   ├── 📄 hello.html
│   └── 📄 style.css
└── 📁 scripts/
    └── 📄 test-curls.sh       # Integration tests
```

---

## 🧪 Testing

Run the full test suite with one command:

```bash
$ bash ./scripts/test-curls.sh
```

**What it validates:**
- ✅ `/` → Returns index page (`200 OK`)
- ✅ `/hello` → Returns greeting (`200 OK`)
- ✅ `/style.css` → Returns CSS (`200 OK`)
- ✅ Invalid routes → Returns `404 Not Found`
- ✅ Server lifecycle (start, test, cleanup)

---

## ⚙️ Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `PUBLIC_PATH` | `./public` | Directory for static files |
| `HOST` | `127.0.0.1` | Server bind address |
| `PORT` | `8080` | Server port |

### Custom Static Path

```bash
$ PUBLIC_PATH=/var/www/static cargo run
🔮 Server listening on http://127.0.0.1:8080
📁 Serving files from: /var/www/static
```

---

## 💡 Why This Project?

<div align="center">

| 🎓 Learning | 🏎️ Performance | 🔒 Security |
|-------------|-----------------|-------------|
| Understand HTTP/1.1 protocol internals | Zero overhead from frameworks | Learn about directory traversal attacks |
| See how Rust handles TCP networking | Compile-time optimizations | Implement security from day one |
| Master request/response lifecycle | Minimal memory footprint | Build defensive coding habits |

</div>

**This project is perfect for:**
- 📚 **Students** learning web server internals
- 🦀 **Rustaceans** exploring `std::net` and TCP
- 🔧 **Developers** who want to understand what frameworks hide
- 🏗️ **Builders** needing a minimal, dependency-free HTTP server

---

## 🤝 Contributing

Contributions are welcome! Here's how to help:

1. 🍴 Fork the repository
2. 🌿 Create a feature branch (`git checkout -b feature/amazing`)
3. 💾 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing`)
5. 🎉 Open a Pull Request

### Code Style
- Follow standard Rust conventions (`cargo fmt`)
- Add tests for new functionality
- Document public APIs with doc comments

---

## 📜 License

This project is open-sourced under the **MIT License**.

See [LICENSE](LICENSE) for the full text.

---

<div align="center">

### ⭐ Found this useful? Give it a star!

**Built with ❤️ and 🦀 Rust**

*Happy coding! 🔮*

</div>
