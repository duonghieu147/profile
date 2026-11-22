# Dương Văn Hiếu

**Senior Software Engineer · Backend**

📞 +84 395 455 399 · ✉️ duonghieu4299@gmail.com · 🔗 [linkedin.com/in/duonghieubku](https://linkedin.com/in/duonghieubku)

---

## Profile

- **Senior Backend Engineer** with 6 years of experience building high-scale systems in logistics and fintech, primarily with Golang, Kafka and event-driven architecture.
- **Currently at GHN**, building the salary engine for 20K+ employees across 1,000+ post offices and 4 major warehouse sorting centers, on an event-driven platform handling ~1M orders per day.
- **Previously at OCB**, built card and payment systems: ISO8583 messaging, HSM-based encryption (Thales payShield 10K), and integrations with Visa, Mastercard and Napas.

---

## Experience

### Giao Hàng Nhanh (GHN)
**Senior Software Engineer** · *Jul 2025 — Present*

- Designed and operated event-driven backend systems handling ~1M delivery orders per day (1.5–2M during peak sale events); the salary computation service sustains ~1K requests/s at peak with P99 latency under 200 ms.
- Built and optimized salary calculation policies for 20K+ employees (truck drivers, couriers, post office staff, and workers at warehouse sorting centers), handling complex business rules based on operational actions and order lifecycle events.
- Built rerun and simulation capabilities for the salary engine: rerunning 30 days of salary data in under 6 hours, and simulating 30 days under a new policy in under 1 minute.
- Optimized the payroll engine to eliminate CPU/RAM spikes and slowdowns during payroll closing: removed N+1 API calls in the payslip hot loop through batch prefetching and ID deduplication, added in-memory caching for core HRM APIs, and fixed OOM crashes by bounding goroutine concurrency.
- Delivered personal income tax (PIT) finalization, and proposed and implemented the sensitive-data protection solution using hybrid RSA + AES-256-GCM encryption to secure payroll and employee personal data.
- Built a shared team knowledge base on OpenClaw powered by Claude, and developed shared Claude Code plugins, skills and MCP servers to automate the team's development pipeline.

🏆 **1st Prize (Team) — AI Revolution 2.0 competition at GHN**

**Tech:** `Event-Driven Architecture` `Golang` `React` `Kafka` `MongoDB` `AlloyDB` `PostgreSQL` `PLG Stack` `Claude Code` `MCP` `OpenClaw`

### Orient Commercial Joint Stock Bank (OCB)
**Software Engineer (Backend-focused Full-stack)** · *Jan 2024 — Jun 2025*

- Developed and maintained satellite systems around the WAY4 card platform across issuer (card issuance, cash loan) and acquirer (POS, ECOM) domains.
- Built end-to-end encryption services on Thales payShield 10K HSMs to protect card and wallet data, covering PIN block translation, key exchange (ZMK/ZPK), and RSA, 3DES and AES encryption.
- Built and processed raw ISO8583 messages for POS and ATM transactions, including key-management messages, routed through Visa, Mastercard and Napas.
- Built the QR cash withdrawal and Napas QR payment applications, compliant with Napas QR standards.
- Developed a checkout payment gateway inspired by MPGS (Mastercard Payment Gateway Services) for secure, scalable online payments.
- Built financial reports and internal analytics.

**Tech:** `Golang` `React` `WAY4` `Thales payShield 10K HSM` `Redis` `Oracle` `PLG Stack` `Docker` `Kubernetes`

### Phenikaa MaaS (BusMap)
**Software Engineer** · *May 2022 — Dec 2023*

**Backend Developer — VinBus / BusMap public transportation app**

- Designed and built the payment gateway, scheduling and ticketing modules for a route-planning and bus-ticketing app, integrating ZaloPay, MoMo and VNPAY.
- Built bus arrival/departure detection at stops from real-time GPS data using k-nearest neighbors (KNN) matching against stop locations, powering arrival-time predictions and "bus approaching" notifications for app users.
- Integrated with an AFC (Automated Fare Collection) card issuance partner as VinBus's implementation partner.
- Owned deployments from test environments to production.

**Frontend Developer**

- Built an admin portal for service planning, form creation, review workflows and statistical reports on public transport services, including electric buses.

**Tech:** `Golang` `PostgreSQL` `MongoDB` `Angular` `Redis` `RabbitMQ` `GCP`

### FPT Software
**Frontend Developer** · *2021 — Apr 2022*

- Built responsive UI features for ClassPad, a web-based learning platform by Casio (Japan) for teachers and students, optimized for web, tablet and mobile (Android & iOS).

**Tech:** `Angular` `Angular Material` `TypeScript`

---

## Skills

| Area | Skills |
|---|---|
| Backend | Golang, Microservices, Event-Driven Architecture, gRPC, REST, Concurrency, Caching & performance tuning |
| Databases | AlloyDB, PostgreSQL, Oracle, MongoDB, Redis |
| Frontend | React, Angular, TypeScript |
| Messaging | Kafka, RabbitMQ |
| DevOps & Cloud | Docker, Kubernetes, GCP, Grafana, Loki (PLG Stack), Nginx, CI/CD |
| Payments & Security | WAY4, ISO8583, Napas QR, Thales payShield HSM, PIN & key management, End-to-end encryption (RSA, AES-256-GCM, 3DES) |
| Testing | Unit & integration testing, benchmarking |
| AI Tooling | Claude Code, MCP, OpenClaw, Gemini |

---

## Education

**HCM City University of Technology (HCMUT)** · *2017 — 2021*
Computer Science
