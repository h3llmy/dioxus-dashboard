# Dioxus Dashboard

A full-stack dashboard application built with Dioxus for the frontend and a Rust-based backend server (likely using Axum or Dioxus fullstack server functions) to power APIs and business logic. ([dioxus.dev][1]).

This repository serves as a Dioxus-powered dashboard template, including migrations, assets, and configurations to get you started quickly.

---

## 🚀 Features

* 🦀 Built with **Rust** and **Dioxus**
* 📦 Includes assets and migrations for backend/DB setup
* 🛠️ Supports TailwindCSS for styling
* 🐳 Includes Docker and docker-compose configurations
* 📁 Modular structure ready for expansion

---

## 🧱 Project Structure

```
├── assets/                  # Static assets (images, CSS, fonts, etc.) used in the frontend
├── migrations/              # Database migration scripts (e.g., SQL or ORM migrations) to create/modify schema
├── src/                     # Main Rust source code directory
    ├── backend/             # Backend API and server implementation
        ├── database/        # Database connection setup
        ├── domains/         # Domain-specific logic (business rules, services, handlers)
    ├── components/          # Reusable Dioxus UI components (buttons, cards, charts, etc.)
    ├── hooks/               # Custom Dioxus hooks (state management and logic extracted for reuse)
    ├── pages/               # UI page views (dashboard pages, auth pages, settings, etc.)
    ├── utils/               # Utility functions and helpers (shared across frontend/backend)
├── .env.example             # Example environment variables for configuration (DB credentials, ports, etc.)
├── Cargo.toml               # Rust package manifest (dependencies, features, metadata)
├── Dioxus.toml              # Dioxus configuration (platform build settings)
├── Dockerfile               # Docker build instructions for containerizing the app
├── docker-compose.yml       # Docker Compose configuration for local development (app, DB, services)
├── tailwind.css             # TailwindCSS base file for styling and design system
└── README.md                # This README with setup, usage, and documentation

```

---

## 💡 Prerequisites

Before you get started, make sure you have the following installed:

* **Rust & Cargo**
  Install from [https://rustup.rs](https://rustup.rs)

* **Dioxus CLI (`dx`)**
  Make sure you have the Dioxus CLI installed:

  ```sh
  cargo install dioxus-cli
  ```

---

## 📦 Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/h3llmy/dioxus-dashboard.git
   cd dioxus-dashboard
   ```

2. Install Rust dependencies:

   ```sh
   cargo fetch
   ```

3. Copy and configure your environment variables:

   ```sh
   cp .env.example .env
   ```

---

## 🧪 Running the App

To start the dashboard in development mode:

```sh
dx serve --platform web
```

Or for a desktop build:

```sh
dx serve --platform desktop
```

---

## 🐳 Using Docker

Build and run with Docker Compose:

```sh
docker compose up --build
```

This will build the application and start any configured services (e.g., database migrations).

---

## 🛠 Development Tips

* Tailwind support is automatic if **tailwind.css** exists in your project root — `dx serve` will process it.
* You can access hot-reloading and DevTools via the `dioxus` CLI.
* Add routes and UI components in `src/`, following the Rust + Dioxus idioms.

---

## 🧠 About Dioxus

This project uses **Dioxus**, a Rust Fullstack framework for building modern apps across platforms — web, desktop, mobile, even backend — from a single codebase. It offers ergonomic state management, hot-reload, and Type-safe UI with Rust safety and performance ([dioxus.dev][1]).

---

## 📫 Contribution

Feel free to open issues or pull requests! This dashboard is a template — improvements and extensions are always welcome.
