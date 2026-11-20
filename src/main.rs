mod data;
mod view;

use axum::{
    Router,
    http::header,
    response::{Html, IntoResponse},
    routing::get,
};

/// File PDF được tạo bằng `cargo run -- pdf` từ chính trang web,
/// rồi nhúng thẳng vào binary nên không cần mang theo file khi deploy.
const CV_PDF: &[u8] = include_bytes!("../assets/Duong-Van-Hieu-CV.pdf");
const CV_PDF_PATH: &str = "assets/Duong-Van-Hieu-CV.pdf";
pub const CV_FILENAME: &str = "Duong-Van-Hieu-CV.pdf";

fn render() -> String {
    view::page(&data::PROFILE).into_string()
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
fn export_pdf() -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};

    let chrome = find_chrome().ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "Không tìm thấy Chrome/Chromium. Đặt biến CHROME=/đường/dẫn/tới/chrome rồi chạy lại.",
        )
    })?;

    let html_path = std::env::temp_dir().join("profile-cv-print.html");
    std::fs::write(&html_path, render())?;
    let out = std::env::current_dir()?.join(CV_PDF_PATH);
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let status = std::process::Command::new(&chrome)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-pdf-header-footer",
            // chờ web font tải xong trước khi in
            "--virtual-time-budget=10000",
            &format!("--print-to-pdf={}", out.display()),
            &format!("file://{}", html_path.display()),
        ])
        .status()?;
    let _ = std::fs::remove_file(&html_path);

    if !status.success() {
        return Err(Error::other(format!("Chrome thoát với mã {status}")));
    }
    println!("Wrote {CV_PDF_PATH} — chạy lại `cargo run` để nút Download dùng bản mới.");
    Ok(())
}

async fn download_cv() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{CV_FILENAME}\""),
            ),
        ],
        CV_PDF,
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // `cargo run -- pdf` → in trang ra assets/Duong-Van-Hieu-CV.pdf
    if args.first().map(String::as_str) == Some("pdf") {
        return export_pdf();
    }

    // `cargo run -- build` → xuất trang tĩnh ra dist/index.html
    if args.first().map(String::as_str) == Some("build") {
        std::fs::create_dir_all("dist")?;
        std::fs::write("dist/index.html", render())?;
        std::fs::write("dist/cv.pdf", CV_PDF)?;
        println!("Wrote dist/index.html, dist/cv.pdf");
        return Ok(());
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{port}");
    let app = Router::new()
        .route("/", get(|| async { Html(render()) }))
        .route("/cv.pdf", get(download_cv));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Profile page running at http://localhost:{port}");
    axum::serve(listener, app).await
}
