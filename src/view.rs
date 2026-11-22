use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::data::{Job, Profile};

const STYLE: &str = include_str!("style.css");

/// Chạy trong <head> trước khi vẽ trang để áp theme + font đã lưu, tránh nháy.
const THEME_INIT: &str = r#"try{var r=document.documentElement,t=localStorage.getItem('theme'),f=localStorage.getItem('font');if(t==='light'||t==='dark')r.dataset.theme=t;if(f==='classic')r.dataset.font=f}catch(e){}"#;

/// Bấm nút: chuyển qua lại giữa font mặc định (JetBrains Mono) và Fraunces + Inter, lưu lại lựa chọn.
const FONT_TOGGLE: &str = r#"(function(){var b=document.getElementById('font-toggle'),l=b.querySelector('.font-name'),r=document.documentElement;function show(){l.textContent=r.dataset.font==='classic'?'Inter':'JetBrains Mono'}show();b.addEventListener('click',function(){if(r.dataset.font==='classic'){delete r.dataset.font;try{localStorage.removeItem('font')}catch(e){}}else{r.dataset.font='classic';try{localStorage.setItem('font','classic')}catch(e){}}show()})})();"#;

/// Bấm nút: đảo theme đang hiển thị (theme đã chọn, hoặc theme hệ thống) và lưu lại.
const THEME_TOGGLE: &str = r#"document.getElementById('theme-toggle').addEventListener('click',function(){var r=document.documentElement,c=r.dataset.theme,d=c?c==='dark':matchMedia('(prefers-color-scheme: dark)').matches,n=d?'light':'dark';r.dataset.theme=n;try{localStorage.setItem('theme',n)}catch(e){}});"#;

/// Menu Download: đóng khi bấm ra ngoài, nhấn Esc, hoặc sau khi chọn một bản.
const DOWNLOAD_MENU: &str = r#"(function(){var d=document.querySelector('.download-menu');document.addEventListener('click',function(e){if(d.open&&!d.contains(e.target))d.open=false});document.addEventListener('keydown',function(e){if(e.key==='Escape')d.open=false});d.querySelectorAll('a').forEach(function(a){a.addEventListener('click',function(){d.open=false})})})();"#;

/// Chân trang khi in PDF (chỉ dùng cho bản có footer): trái là tên, phải là số trang.
fn footer_page_css(name: &str) -> String {
    let font = r#"font: 400 8.5px "JetBrains Mono", ui-monospace, monospace; color: #8a857b;"#;
    format!(
        r#"@page cv-footer {{
  @bottom-left {{ content: "{name} — Curriculum Vitae"; {font} }}
  @bottom-right {{ content: counter(page) " / " counter(pages); {font} }}
}}
@media print {{ body {{ page: cv-footer; }} }}"#
    )
}

fn toolbar() -> Markup {
    html! {
        nav.toolbar aria-label="Page actions" {
            details.download-menu {
                summary.download {
                    svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                        path d="M12 3v12" {}
                        path d="m7 10 5 5 5-5" {}
                        path d="M5 21h14" {}
                    }
                    span { "Download CV" }
                    svg.caret width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                        path d="m6 9 6 6 6-6" {}
                    }
                }
                ul.menu {
                    li { a href="cv-footer.pdf" download=(crate::CV_FILENAME) { "PDF — with page footer" } }
                    li { a href="cv.pdf" download=(crate::CV_FILENAME) { "PDF — no footer" } }
                }
            }
            button #theme-toggle .icon-btn type="button" aria-label="Toggle light / dark theme" title="Toggle light / dark theme" {
                // mặt trăng: hiện ở theme sáng
                svg.moon width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                    path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" {}
                }
                // mặt trời: hiện ở theme tối
                svg.sun width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                    circle cx="12" cy="12" r="4" {}
                    path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4" {}
                }
            }
            button #font-toggle .font-btn type="button" aria-label="Switch font" title="Switch font" {
                span.aa aria-hidden="true" { "Aa" }
                span.font-name { "JetBrains Mono" }
            }
        }
    }
}

/// `footer`: bản in PDF có chân trang (tên · số trang).
pub fn page(p: &Profile, footer: bool) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (p.name) " — " (p.title) }
                meta name="description" content=(format!("{} — {}", p.name, p.title));
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,500;9..144,700&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500;600;700&display=swap";
                script { (PreEscaped(THEME_INIT)) }
                style { (PreEscaped(STYLE)) }
                @if footer {
                    style { (PreEscaped(footer_page_css(p.name))) }
                }
            }
            body {
                (toolbar())
                main.sheet {
                    (header(p))
                    (section("Profile", "01", html! {
                        ul.summary {
                            @for (lead, rest) in p.summary {
                                li { strong { (lead) } " " (rest) }
                            }
                        }
                    }))
                    (section("Experience", "02", html! {
                        ol.timeline {
                            @for job in p.experience { (job_item(job)) }
                        }
                    }))
                    (section("Skills", "03", html! {
                        dl.skills {
                            @for (k, v) in p.skills {
                                dt { (k) }
                                dd { (v) }
                            }
                        }
                    }))
                    (section("Education", "04", html! {
                        @for e in p.education {
                            div.edu {
                                div {
                                    h3 { (e.school) }
                                    p.muted { (e.major) }
                                }
                                span.period { (e.period) }
                            }
                        }
                    }))
                }
                script { (PreEscaped(THEME_TOGGLE)) }
                script { (PreEscaped(FONT_TOGGLE)) }
                script { (PreEscaped(DOWNLOAD_MENU)) }
            }
        }
    }
}

fn header(p: &Profile) -> Markup {
    html! {
        header.hero {
            p.eyebrow { "Curriculum Vitae" }
            h1 { (p.name) }
            p.role { (p.title) }
            ul.contact {
                li { a href=(format!("tel:{}", p.phone.replace(' ', ""))) { span.k { "tel" } (p.phone) } }
                li { a href=(format!("mailto:{}", p.email)) { span.k { "mail" } (p.email) } }
                li { a href=(format!("https://{}", p.linkedin)) target="_blank" rel="noopener" { span.k { "in" } (p.linkedin) } }
            }
        }
    }
}

fn section(title: &str, index: &str, body: Markup) -> Markup {
    html! {
        section {
            h2 { span.idx { (index) } (title) }
            (body)
        }
    }
}

fn job_item(job: &Job) -> Markup {
    html! {
        li.job {
            div.job-head {
                div {
                    h3 { (job.company) }
                    p.job-role { (job.role) }
                }
                span.period { (job.period) }
            }
            @if let Some(intro) = job.intro {
                p.intro { (intro) }
            }
            @for g in job.groups {
                @if let Some(h) = g.heading {
                    h4 { (h) }
                }
                ul.bullets {
                    @for item in g.items { li { (item) } }
                }
            }
            @if !job.awards.is_empty() {
                ul.awards {
                    @for a in job.awards {
                        li {
                            svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                                path d="M8 21h8M12 17v4M7 4h10v5a5 5 0 0 1-10 0V4z" {}
                                path d="M17 5h3v2a3 3 0 0 1-3 3M7 5H4v2a3 3 0 0 0 3 3" {}
                            }
                            span { (a) }
                        }
                    }
                }
            }
            ul.tags {
                @for t in job.tech { li { (t) } }
            }
        }
    }
}
