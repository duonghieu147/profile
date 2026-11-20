//! Nội dung CV — sửa ở đây để cập nhật trang.

pub struct Profile {
    pub name: &'static str,
    pub title: &'static str,
    pub phone: &'static str,
    pub email: &'static str,
    pub linkedin: &'static str,
    pub summary: &'static [(&'static str, &'static str)],
    pub education: &'static [Education],
    pub experience: &'static [Job],
    pub skills: &'static [(&'static str, &'static str)],
}

pub struct Education {
    pub school: &'static str,
    pub major: &'static str,
    pub period: &'static str,
}

pub struct Job {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub intro: Option<&'static str>,
    pub groups: &'static [Group],
    /// Thành tích / giải thưởng tại công ty này (hiển thị nổi bật dưới phần mô tả).
    pub awards: &'static [&'static str],
    pub tech: &'static [&'static str],
}

/// Một nhóm gạch đầu dòng, có thể có tiêu đề phụ (vd: vai trò trong một dự án).
pub struct Group {
    pub heading: Option<&'static str>,
    pub items: &'static [&'static str],
}

pub const PROFILE: Profile = Profile {
    name: "Dương Văn Hiếu",
    title: "Senior Software Engineer · Backend",
    phone: "+84 395 455 399",
    email: "duonghieu4299@gmail.com",
    linkedin: "linkedin.com/in/duonghieubku",
    summary: &[
        (
            "Senior Backend Engineer",
            "with 6 years of experience building high-scale systems in logistics and fintech, primarily with Golang, Kafka and event-driven architecture.",
        ),
        (
            "Currently at GHN,",
            "building the salary engine for 20K+ employees across 1,000+ post offices and 4 major warehouse sorting centers, on an event-driven platform handling ~1M orders per day.",
        ),
        (
            "Previously at OCB,",
            "built card and payment systems: ISO8583 messaging, HSM-based encryption (Thales payShield 10K), and integrations with Visa, Mastercard and Napas.",
        ),
    ],
    education: &[Education {
        school: "HCM City University of Technology (HCMUT)",
        major: "Computer Science",
        period: "2017 — 2021",
    }],
    experience: &[
        Job {
            company: "Giao Hang Nhanh",
            role: "Senior Software Engineer",
            period: "Jul 2025 — Present",
            intro: None,
            groups: &[Group {
                heading: None,
                items: &[
                    "Designed and operated event-driven backend systems handling ~1M delivery orders per day (1.5–2M during peak sale events); the salary computation service sustains ~1K requests/s at peak with P99 latency under 200 ms.",
                    "Built and optimized salary calculation policies for 20K+ employees (truck drivers, couriers, post office staff, and workers at warehouse sorting centers), handling complex business rules based on operational actions and order lifecycle events.",
                    "Built rerun and simulation capabilities for the salary engine: rerunning 30 days of salary data in under 6 hours, and simulating 30 days under a new policy in under 1 minute.",
                    "Delivered personal income tax (PIT) finalization, and proposed and implemented the sensitive-data protection solution using hybrid RSA + AES-256-GCM encryption to secure payroll and employee personal data.",
                    "Built a shared team knowledge base on Openclaw powered by Claude, and developed shared Claude Code plugins, skills and MCP servers to automate the team's development pipeline.",
                ],
            }],
            awards: &["1st Prize (Team) — AI Revolution 2.0 competition at GHN"],
            tech: &[
                "Event-Driven Architecture", "Golang", "React", "Kafka", "MongoDB", "AlloyDB",
                "PostgreSQL", "PLG Stack", "Claude Code", "MCP", "Openclaw",
            ],
        },
        Job {
            company: "Orient Commercial Joint Stock Bank (OCB)",
            role: "Software Engineer (Backend-focused Fullstack)",
            period: "Jan 2024 — Jun 2025",
            intro: None,
            groups: &[Group {
                heading: None,
                items: &[
                    "Developed and maintained satellite systems around the WAY4 card platform across issuer (card issuance, cash loan) and acquirer (POS, ECOM) domains.",
                    "Built end-to-end encryption services on Thales payShield 10K HSMs to protect card and wallet data, covering PIN block translation, key exchange (ZMK/ZPK), and RSA, 3DES and AES encryption.",
                    "Built and processed raw ISO8583 messages for POS and ATM transactions, including key-management messages, routed through Visa, Mastercard and Napas.",
                    "Built the QR cash withdrawal and Napas QR payment applications, compliant with Napas QR standards.",
                    "Developed a checkout payment gateway inspired by MPGS (Mastercard Payment Gateway Services) for secure, scalable online payments.",
                    "Built financial reports and internal analytics.",
                ],
            }],
            awards: &[],
            tech: &[
                "Golang", "React", "WAY4", "Thales payShield 10K HSM", "Redis", "OracleDB",
                "PLG Stack", "Docker", "Kubernetes",
            ],
        },
        Job {
            company: "Phenikaa MaaS (BusMap)",
            role: "Software Engineer",
            period: "May 2022 — Dec 2023",
            intro: None,
            groups: &[
                Group {
                    heading: Some("Back-End Developer — Vinbus / BusMap public transportation app"),
                    items: &[
                        "Designed and built the payment gateway, scheduling and ticketing modules for a route-planning and bus-ticketing app, integrating ZaloPay, MoMo and VNPAY.",
                        "Integrated with an AFC (Automated Fare Collection) card issuance partner as VinBus's implementation partner.",
                        "Owned deployments from test environments to production.",
                    ],
                },
                Group {
                    heading: Some("Front-End Developer"),
                    items: &[
                        "Built an admin portal for service planning, form creation, review workflows and statistical reports on public transport services, including electric buses.",
                    ],
                },
            ],
            awards: &[],
            tech: &[
                "Golang", "PostgreSQL", "MongoDB", "Angular", "Redis", "RabbitMQ", "GCP",
            ],
        },
        Job {
            company: "FPT Software",
            role: "Front-End Developer",
            period: "2021 — Apr 2022",
            intro: None,
            groups: &[Group {
                heading: None,
                items: &[
                    "Built responsive UI features for ClassPad, Casio's (Japan) web-based learning platform for teachers and students, optimized for web, tablet and mobile (Android & iOS).",
                ],
            }],
            awards: &[],
            tech: &["Angular", "Angular Material", "TypeScript"],
        },
    ],
    skills: &[
        ("Back-end", "Golang, Microservices, Event-Driven Architecture, gRPC, REST"),
        ("Databases", "AlloyDB, PostgreSQL, Oracle, MongoDB, Redis"),
        ("Front-end", "React, Angular, TypeScript"),
        ("Messaging", "Kafka, RabbitMQ"),
        ("DevOps & Cloud", "Docker, Kubernetes, GCP, PLG Stack, Nginx, CI/CD"),
        (
            "Payments & Security",
            "WAY4, ISO8583, Thales payShield HSM, PIN & key management, End-to-end encryption (RSA, AES-256-GCM, 3DES)",
        ),
        ("AI Tooling", "Claude Code, MCP, Openclaw, Gemini, Antigravity"),
    ],
};
