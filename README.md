# 🧠 YMB AI Bot

A microservice AI assistant designed to generate human-like responses using powerful RAG (Retrieval-Augmented Generation) techniques and Google's **Gemini** AI, deployed on **Google Cloud Run** using Docker.

---

## 🚀 Features

- ✨ **AI-powered messaging**: Uses **Gemini via ARC** to generate smart, context-aware group messages.
- 📂 **Retrieval-Augmented Generation**: Embeds your past messages as context to improve the accuracy and tone of future messages.
- 🐳 **Dockerized microservice**: Easily deployable as an isolated container.
- ☁️ **Cloud Run integration**: Automatically deployed with each push to `master`, enabling continuous development.
- 🔄 **Build triggers**: CI/CD enabled via Google Cloud Build for rapid iteration and reliable production deployment.

---

## 🛠️ Technology Stack

- **Rust + Axum** backend
- **Tokio** for async runtime
- **Serenity** for Discord/Telegram integrations
- **Google Gemini (via ARC)** for LLM capabilities
- **Custom embedding pipeline** to index and retrieve past messages

---

## 📁 Project Structure

```
ymb-ai-bot/
│
├── src/                # Rust source files
│   ├── app.rs          # Axum app entry
│   ├── logger.rs       # Tracing + logging setup
│   └── errors.rs       # Error handling
│
├── config/             # Environment-specific configs
├── documents/          # Text files for embedding into RAG pipeline
├── Dockerfile          # For containerization
├── compose.yaml        # (optional) Docker Compose setup
├── Cargo.toml          # Rust dependencies
└── README.md
```

---

## ☁️ Deployment (Google Cloud Run)

This project is deployed using **Google Cloud Run**:

- ✅ Containerized using the provided `Dockerfile`
- 🔄 Deployed using **Cloud Build triggers** with every push to the `master` branch
- 🧪 Enables **continuous development** and quick feedback loops

### Sample Build Trigger Config (GCP):

- **Trigger Source**: GitHub repo
- **Branch**: `master`
- **Build Config**: Dockerfile
- **Deploy To**: Cloud Run service

---

## 🧠 AI Integration (Gemini + ARC)

This bot is enhanced with **AI Agent capabilities** via Google’s **Gemini API** through ARC:

- 📄 **Past messages** are embedded as vector representations
- 🧠 These embeddings are used during inference to **retrieve relevant past contexts**
- 💬 The model can then **generate personalized, relevant responses** that feel consistent with past human-written messages

---

## 📦 Running Locally

```bash
# Build and run using Docker
docker build -t ymb-ai-bot .
docker run -p 8080:8080 ymb-ai-bot
```

---
