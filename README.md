# Dương Văn Hiếu — Profile

🌐 **Live:** https://duonghieu147.github.io/profile/

Trang CV cá nhân viết bằng Rust ([Axum](https://github.com/tokio-rs/axum) + [Maud](https://maud.lambda.xyz)).

## Makefile

```bash
make             # liệt kê lệnh
make run         # chạy trang ở http://localhost:8080
make dev         # tạo lại PDF rồi chạy trang (sau khi sửa src/data.rs)
make pdf         # tạo lại assets/Duong-Van-Hieu-CV.pdf
make build       # xuất trang tĩnh ra dist/
make stop        # tắt server
make push MSG="cv: update GHN"   # tạo lại PDF, commit, push → GitHub Pages tự deploy
```

## Chạy bằng cargo

```bash
cargo run              # mở http://localhost:8080 (đổi cổng bằng PORT=3000)
cargo run -- build     # xuất trang tĩnh ra dist/ (index.html + cv.pdf)
cargo run -- pdf       # in trang ra assets/Duong-Van-Hieu-CV.pdf (cần Chrome/Chromium/Edge)
```

Sau khi sửa `src/data.rs`, chạy `cargo run -- pdf` để PDF khớp với trang, rồi `cargo run` lại.
Nếu Chrome nằm ở chỗ khác: `CHROME=/path/to/chrome cargo run -- pdf`.

Trang có nút đổi theme sáng/tối (nhớ lựa chọn) và nút tải CV PDF tại `/cv.pdf`.

## Cấu trúc

- `src/data.rs` — nội dung CV (sửa ở đây để cập nhật)
- `src/view.rs` — template HTML (Maud)
- `src/style.css` — giao diện, hỗ trợ dark mode, mobile và in ấn
- `src/main.rs` — web server / lệnh build tĩnh
- `assets/Duong-Van-Hieu-CV.pdf` — file CV cho nút tải, tạo bằng `cargo run -- pdf` (nhúng vào binary)
- `profile.md` — nội dung CV dạng Markdown
