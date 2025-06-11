# Rustyvault-TUI

**Author:** Cyberhunter (Sriram19g) – [https://github.com/Sriram19g](https://github.com/Sriram19g) – gsriram200@gmail.com

Rustyvault-TUI is a terminal-based password manager written in Rust. It provides a secure, intuitive interface for storing, retrieving, and managing site credentials directly from your terminal. Featuring Diesel ORM with SQLite for storage and XChaCha20-Poly1305 for encryption, Rustyvault-TUI ensures your sensitive data remains protected.

## Features

- Secure master-password login using Argon2 hashing
- Store site name, URL, email, username, and encrypted password
- Add, update, delete, and filter entries
- Copy decrypted passwords to clipboard
- Change master password and re-encrypt all entries
- Intuitive TUI built with `ratatui`
- SQLite backend via Diesel ORM
- Lightweight, cross-platform, and zero dependencies beyond the Rust ecosystem

## Installation

```bash
# Clone the repository
git clone https://github.com/Sriram19g/rustyvault-tui.git
cd rustyvault-tui

# Install Diesel CLI (if not already installed)
cargo install diesel_cli --no-default-features --features sqlite

# Set up the database
diesel setup

# Build and run the application
cargo run
```

## Usage

1. On first run, create your master password.
2. Navigate between fields with Tab or arrow keys (`j`/`k`).
3. Fill in site credentials and press Enter to save.
4. On the Main Screen, use the following keys:
   - `a`: Add entry
   - `d`: Delete entry
   - `f`: Filter entries
   - `c`: Copy password to clipboard
   - `u`: Update master password
   - `q` / `Esc`: Quit application
5. The database file `rustyvault.db` is created in the project root.

## Configuration

- Migrations are located under the `migrations/` directory.
- Database file: `rustyvault.db`.
- Configuration in `.env` and `diesel.toml`.

## Contributing

Contributions are welcome! Please:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/my-feature`.
3. Commit your changes: `git commit -m 'Add my feature'`.
4. Push to the branch: `git push origin feature/my-feature`.
5. Open a Pull Request.

## License

This project is licensed under the MIT License.
