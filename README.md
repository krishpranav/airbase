# Airbase

**Airbase** is an open-source, self-hosted backend for side projects.

Think **Firebase Lite**, without vendor lock-in, hidden costs, or magic rules.

> Auth · Document DB · File Storage · Realtime — in a single binary.

---

## Why Airbase?

Firebase is powerful — but:
- Hard to self-host
- Vendor lock-in
- Complex security rules
- Overkill for most side projects

Airbase focuses on **developer speed**, **clarity**, and **ownership**.

---

## 🚀 Features

### Authentication
- Email / password auth
- JWT access tokens
- OAuth (Google, GitHub)
- User roles (admin / user)

### Document Database
- Collection / document model
- JSON storage
- CRUD APIs
- Ownership-based access
- Auto timestamps

### File Storage
- Local filesystem (default)
- Public & private files
- Signed download URLs
- Metadata stored in DB
- S3-compatible (coming)

### Realtime
- WebSocket based
- Subscribe to collections
- Realtime DB change events
- Presence tracking
- Lightweight pub/sub

---

## Tech Stack

- **Backend**: Rust (Axum)
- **Database**: PostgreSQL (prod), SQLite (dev)
- **Realtime**: WebSockets
- **Auth**: JWT + OAuth
- **Deployment**: Docker / single binary

---

## Quick Start

```bash
git clone https://github.com/krishpranav/airbase
cd airbase
cp .env.example .env
docker compose up --build

Server runs at:

http://localhost:8080
ws://localhost:8080/realtime

```

## 🔐 Authentication

### Signup:

- POST /auth/signup:
```
{
 "email": "user@example.com",
  "password": "secret"
}
```

### Login

- POST /auth/login

Returns JWT token.

⸻

### Document Database

- Insert document

- POST /db/posts
```
Authorization: Bearer <token>

{
  "title": "Hello Airbase",
  "content": "Firebase but simpler"
}
```

### Get document

- GET /db/posts/{id}


⸻

### Realtime:

- Subscribe to DB events:
```
const ws = new WebSocket("ws://localhost:8080/realtime");

ws.onmessage = (msg) => {
  console.log(msg.data);
};

Events:
	•	db.insert
	•	db.update
	•	db.delete
	•	presence.join
	•	presence.leave

```

### Deployment
	•	Single Docker image
	•	One docker-compose.yml
	•	Environment variable configuration

No managed hosting (yet).

⸻

### Non-Goals
	•	Full Firebase API compatibility
	•	Complex rules language
	•	Enterprise IAM
	•	Billing (for now)

⸻

### Roadmap
	•	SQLite dev mode
	•	JS SDK
	•	Storage buckets
	•	Role-based rules
	•	Admin dashboard
	•	Plugins
	•	Managed hosting (optional)

⸻

## Philosophy

Airbase is:
	•	Hackable
	•	Understandable
	•	Self-hosted
	•	Indie-friendly

If you can read the code, you own your backend.

⸻

## 📄 License

- [Apache License](./LICENSE)

