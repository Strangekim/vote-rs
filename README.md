<div align="center">

# 🗳️ High-Performance Vote Backend

<p align="center">
  <strong>Rust + Axum + PostgreSQL 기반의 고성능 대국민 투표 시스템</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
</p>

<p align="center">
  단일 서버에서의 극한의 성능(RPS)을 추구하며<br>
  <strong>Compile-time Safety</strong>와 <strong>Raw SQL 최적화</strong>를 목표로 합니다.
</p>

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🚀 High Performance
- Zero-Cost Abstractions
- No Garbage Collection
- 비동기 I/O (Tokio Runtime)
- Database Connection Pooling

</td>
<td width="50%">

### 🛡️ Type Safety
- Compile-time Query Validation
- Memory Safety without GC
- No Null Pointer Exceptions
- Exhaustive Pattern Matching

</td>
</tr>
</table>

---

## 🛠 Tech Stack

<div align="center">

| Category | Technology | Version | Description |
|:---:|:---:|:---:|:---|
| 🦀 **Language** | **Rust** | 2021 Edition | GC 없는 안정적인 성능, 메모리 안전성 |
| 🌐 **Framework** | **Axum** | v0.7 | `tokio` 기반 비동기 웹 프레임워크 |
| 🐘 **Database** | **PostgreSQL** | v16 Alpine | 신뢰성 높은 RDBMS |
| 🔗 **DB Driver** | **sqlx** | v0.8 | 컴파일 타임 쿼리 검증, No ORM |
| 📊 **Load Testing** | **k6** | Latest | 성능 측정 및 병목 구간 확인 |
| 🐳 **Infrastructure** | **Docker** | Compose | 로컬 개발 환경 통일 |

</div>

---

## 🏗 Database Schema (Database-First)

> ORM(Sequelize, TypeORM) 대신 **Raw SQL Migration**을 사용하여 최적화된 스키마를 직접 관리합니다.

### 🎯 Key Strategy

<table>
<tr>
<td align="center" width="33%">

#### 🆔 UUID PK
분산 환경 및 보안을 위해<br>
`SERIAL` 대신 `UUID` 사용

</td>
<td align="center" width="33%">

#### 🔒 Unique Constraints
애플리케이션 로직 대신<br>
DB 제약조건으로 무결성 보장

</td>
<td align="center" width="33%">

#### ⚡ Denormalization
캐싱 컬럼으로<br>
조회 성능(Read) 최적화

</td>
</tr>
</table>

### 📋 Tables

```sql
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   users     │      │   agendas   │      │    votes    │
├─────────────┤      ├─────────────┤      ├─────────────┤
│ id (UUID)   │──┐   │ id (UUID)   │──┐   │ id (UUID)   │
│ email       │  │   │ title       │  │   │ user_id  ◄──┘
│ password    │  │   │ description │  │   │ agenda_id ◄──┘
│ name        │  │   │ creator_id◄─┘  │   │ vote_type   │
│ created_at  │  │   │ agree_count │  │   │ created_at  │
└─────────────┘  └──►│ disagree_   │  │   └─────────────┘
                     │   count     │  │   UNIQUE(user_id,
                     │ created_at  │  │         agenda_id)
                     └─────────────┘  │
```

- **`users`**: 유권자 정보 (이메일, 비밀번호, 이름)
- **`agendas`**: 투표 안건 (제목, 생성자, **찬/반 캐싱 컬럼**)
- **`votes`**: 투표 내역 (User-Agenda 복합 유니크 인덱스로 **중복 투표 원천 봉쇄**)

---

## 🚀 Getting Started

### 📋 Prerequisites

<table>
<tr>
<td>

- 🦀 **Rust** (Cargo)
- 🐳 **Docker** & Docker Compose
- 🛠️ **sqlx-cli** (마이그레이션 도구)

</td>
<td>

```bash
# sqlx-cli 설치
cargo install sqlx-cli
```

</td>
</tr>
</table>

### 1️⃣ Environment Setup

프로젝트 루트에 `.env` 파일을 생성합니다.

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/vote_db
```

### 2️⃣ Database Initialization

Docker로 DB를 띄우고 테이블을 생성합니다.

```bash
# 1️⃣ PostgreSQL 컨테이너 실행
docker-compose up -d

# 2️⃣ 데이터베이스 생성 및 마이그레이션 적용
sqlx database create
sqlx migrate run
```

### 3️⃣ Run Server

개발 모드 또는 릴리즈 모드로 실행합니다.

```bash
# 🔧 개발용 실행 (Debug Mode)
cargo run

# 🚀 성능 테스트용 실행 (Release Mode) - 훨씬 빠름!
cargo run --release
```

<div align="center">

✅ 서버가 뜨면 [`http://localhost:3000/health`](http://localhost:3000/health) 에서 상태를 확인하세요!

</div>

---

## 📊 Performance Testing

단일 서버의 최대 처리량(RPS)을 측정하기 위해 **k6**를 사용합니다.

<table>
<tr>
<td width="50%">

### 🏃 실행 방법

```bash
# 1️⃣ Release 모드로 서버 실행
cargo run --release

# 2️⃣ k6 부하 테스트 (100 VUs, 10s)
k6 run script.js
```

</td>
<td width="50%">

### 📈 측정 지표

- **RPS** (Requests Per Second)
- **P95 Latency**
- **Error Rate**
- **Throughput**

</td>
</tr>
</table>

---

## � API Documentation

### 🔐 Auth

| Method | Endpoint | Summary | Request / Response |
|:---:|:---|:---|:---|
| `POST` | **/auth/signup** | 회원가입 | **Req**: `{ "username": "홍길동" }`<br>**Res**: `201 Created` (User) |
| `POST` | **/auth/login** | 로그인 | **Req**: `{ "username": "홍길동" }`<br>**Res**: `200 OK` `{ "token": "...", "user_id": "...", "username": "..." }` |

### 🗳️ Agendas

| Method | Endpoint | Summary | Request / Response |
|:---:|:---|:---|:---|
| `GET` | **/agendas** | 안건 목록 조회 | **Res**: `200 OK` (Agenda List) |
| `POST` | **/agendas** | 안건 생성 | **Req**: `{ "title": "...", "description": "..." }`<br>**Header**: `Authorization: Bearer <token>` |
| `POST` | **/agendas/:id/vote** | 찬반 투표 | **Req**: `{ "is_agree": true }`<br>**Header**: `Authorization: Bearer <token>` |

> 자세한 스펙은 [APIDog](https://apidog.com) 프로젝트를 참고하세요.

---

## �📂 Project Structure

> Express 개발자에게 익숙한 구조로 발전시켜 나갈 예정입니다.

```plaintext
vote-rs/
├── 📁 migrations/         # DB 스키마 변경 이력 (.sql)
├── 📁 src/
│   └── 📄 main.rs         # 앱 진입점 및 라우터 설정
├── 🔒 .env                # 환경 변수 (Git 제외)
├── 📦 Cargo.toml          # 의존성 관리 (≈ package.json)
├── 🐳 docker-compose.yml  # PostgreSQL 컨테이너 설정
└── 📊 script.js           # k6 부하 테스트 스크립트
```

---

## 📝 Roadmap

<table>
<tr>
<td width="50%" valign="top">

### ✅ Completed

- [x] 프로젝트 초기화 및 환경 설정
- [x] DB 스키마 설계 및 마이그레이션
- [x] 기본 웹 서버 구동 (Health Check)
- [x] 부하 테스트 환경 구축 (k6)

</td>
<td width="50%" valign="top">

### 🚧 In Progress

- [ ] 회원가입/로그인 API 구현
- [ ] 안건(Agenda) 생성/조회 API
- [ ] **투표(Vote) 트랜잭션 구현**
- [ ] 동시성 제어 및 성능 최적화

</td>
</tr>
</table>

---

<div align="center">

### 🌟 Made with ❤️ using Rust

**고성능 · 타입 안전성 · 동시성**

</div>