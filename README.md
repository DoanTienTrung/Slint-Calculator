# 🧮 Simple Calculator (Rust + Slint)

A simple yet stylish calculator built with **Rust** and **Slint** UI toolkit.  
This project demonstrates how to combine **Rust’s logic layer** with **modern declarative UI** design using Slint.
🖼️ Screenshot
Here’s what the final app looks like:

<img width="415" height="641" alt="image" src="https://github.com/user-attachments/assets/9a566314-63b3-4229-a1fc-c6b649463a56" />


---

## 🚀 How to Build & Run

### 🧰 Requirements
Make sure you have installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Slint runtime](https://slint.dev/docs/rust/slint/)
- (Optional) `cargo` for managing Rust packages

### ▶️ Run the app
Clone this repository and run:
```bash
git clone https://github.com/<your-username>/simple_calculator.git
cd simple_calculator
cargo run
That’s it! 🎉
Your calculator window should open automatically.

🏗️ Architecture Overview
bash
Copy code
simple_calculator/
├── src/
│   ├── main.rs        # Rust entry point (connects UI & logic)
│   ├── calc.rs        # Handles all math operations and calculator state
│   └── ui.slint       # UI layout and styling (written in Slint markup)
├── Cargo.toml         # Rust project manifest
└── README.md
🧩 How it works
ui.slint defines the visual layout (buttons, display, colors, etc.).

main.rs loads that UI and binds button events to Rust functions.

calc.rs contains the calculator logic: operations, error handling, etc.

The connection between UI and Rust uses callbacks and properties —
for example, when a Slint button is clicked, it triggers a Rust function that updates the display.

🧑‍💻 Author
Đoàn Tiên Trung
Built with ❤️ using Rust + Slint

📜 License
This project is released under the MIT License.

yaml
Copy code

---


### ✅ Gợi ý thêm
Sau khi bạn chạy ứng dụng và chụp ảnh màn hình:
1. Tạo thư mục `docs/`
2. Lưu hình vào `docs/screenshot.png`
3. Commit lại:
   ```bash
   git add docs/screenshot.png README.md
   git commit -m "Add README and screenshot"
   git push
