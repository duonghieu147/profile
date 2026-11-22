mod data;
mod view;

use axum::{
    Router,
    http::header,
    response::{Html, IntoResponse},
    routing::get,
};

/// Tên file khi người xem tải về (giống nhau cho mọi bản).
pub const CV_FILENAME: &str = "Duong-Van-Hieu-CV.pdf";

/// Một bản PDF: có/không chân trang.
/// File được tạo bằng `cargo run -- pdf` rồi nhúng thẳng vào binary.
struct Variant {
    /// Tên file phục vụ trên web / dist (vd "cv-footer.pdf")
    url: &'static str,
    /// File nguồn trong repo
    asset: &'static str,
    bytes: &'static [u8],
    footer: bool,
}

const VARIANTS: [Variant; 2] = [
    Variant {
        url: "cv.pdf",
        asset: "assets/Duong-Van-Hieu-CV.pdf",
        bytes: include_bytes!("../assets/Duong-Van-Hieu-CV.pdf"),
        footer: false,
    },
    Variant {
        url: "cv-footer.pdf",
        asset: "assets/Duong-Van-Hieu-CV-footer.pdf",
        bytes: include_bytes!("../assets/Duong-Van-Hieu-CV-footer.pdf"),
        footer: true,
    },
];

fn render(footer: bool) -> String {
    view::page(&data::PROFILE, footer).into_string()
}

/// Đường dẫn Chrome/Chromium: lấy từ biến môi trường CHROME, nếu không có thì thử các vị trí quen thuộc.
fn find_chrome() -> Option<String> {
    if let Ok(p) = std::env::var("CHROME") {
        return Some(p);
    }
    [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
    ]
    .into_iter()
    .find(|p| std::path::Path::new(p).exists())
    .map(String::from)
}

/// Dựng trang rồi để Chrome headless in ra PDF (dùng CSS @media print → theme sáng, khổ A4).
fn export_pdf(v: &Variant) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};

    let chrome = find_chrome().ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "Không tìm thấy Chrome/Chromium. Đặt biến CHROME=/đường/dẫn/tới/chrome rồi chạy lại.",
        )
    })?;

    let html_path = std::env::temp_dir().join(format!("profile-{}.html", v.url));
    std::fs::write(&html_path, render(v.footer))?;
    let out = std::env::current_dir()?.join(v.asset);
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let output = std::process::Command::new(&chrome)
        .args([
            "--headless",
            "--disable-gpu",
            // tắt header/footer mặc định của Chrome; chân trang (nếu có) do CSS @page vẽ
            "--no-pdf-header-footer",
            // chờ web font tải xong trước khi in
            "--virtual-time-budget=10000",
            &format!("--print-to-pdf={}", out.display()),
            &format!("file://{}", html_path.display()),
        ])
        .output()?; // gom log của Chrome lại, chỉ in khi lỗi
    let _ = std::fs::remove_file(&html_path);

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err(Error::other(format!("Chrome thoát với mã {}", output.status)));
    }
    println!("Wrote {}", v.asset);
    Ok(())
}

/// Ưu tiên đọc file trong assets/ mỗi lần tải → chạy `make pdf` xong là tải được bản mới
/// ngay, không cần khởi động lại server. Bản nhúng trong binary chỉ là dự phòng.
fn pdf_response(asset: &'static str, embedded: &'static [u8]) -> impl IntoResponse {
    let bytes = std::fs::read(asset).unwrap_or_else(|_| embedded.to_vec());
    (
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{CV_FILENAME}\""),
            ),
        ],
        bytes,
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let has = |flag: &str| args.iter().any(|a| a == flag);

    // `cargo run -- pdf [footer|plain]` → mặc định tạo cả 2 bản
    if args.first().map(String::as_str) == Some("pdf") {
        let footer_ok = |f: bool| !(has("footer") && !f) && !(has("plain") && f);
        for v in VARIANTS.iter().filter(|v| footer_ok(v.footer)) {
            export_pdf(v)?;
        }
        println!("Xong. Server đang chạy sẽ tải bản mới ngay (đọc trực tiếp từ assets/).");
        return Ok(());
    }

    // `cargo run -- build` → xuất trang tĩnh ra dist/
    if args.first().map(String::as_str) == Some("build") {
        std::fs::create_dir_all("dist")?;
        std::fs::write("dist/index.html", render(false))?;
        for v in &VARIANTS {
            let bytes = std::fs::read(v.asset).unwrap_or_else(|_| v.bytes.to_vec());
            std::fs::write(format!("dist/{}", v.url), bytes)?;
        }
        println!("Wrote dist/index.html + {} PDFs", VARIANTS.len());
        return Ok(());
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{port}");
    let mut app = Router::new().route("/", get(|| async { Html(render(false)) }));
    for v in &VARIANTS {
        let (asset, bytes) = (v.asset, v.bytes);
        app = app.route(&format!("/{}", v.url), get(move || async move { pdf_response(asset, bytes) }));
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Profile page running at http://localhost:{port}");
    axum::serve(listener, app).await
}
