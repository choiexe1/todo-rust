# Todo Rust + SvelteKit

https://github.com/choiexe1/todo-rust/assets/161970433/fc96ac0a-97ce-4b08-bd77-19ad8215888f

- Rust
  - Axum (Web Framework)
  - Tokio (Async)
  - Serde (Serialization)
- SvelteKit
  - Skeleton (UI Toolkit)
  - TailwindCSS (CSS Framework)

# 설치

- 이 레포지토리를 클론하고 `backend` 폴더로 이동합니다.

```
cargo install sqlx-cli
```

# 실행

> Backend

```
cd backend/

sqlx database setup
cargo run
```

> Frontend

```
cd frontend/

npm install
npm run dev
```
