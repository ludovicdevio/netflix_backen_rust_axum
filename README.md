# 🦀 Netflix Clone — Backend API

> API REST haute performance construite avec **Rust** · **Axum 0.8** · **Tokio**

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://docs.rs/axum)
[![TMDB](https://img.shields.io/badge/TMDB-API-brightgreen)](https://www.themoviedb.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

Backend du projet **Netflix Clone**. Il expose une API REST qui agrège les données de l'API TMDB (films, séries, bandes-annonces) et assure le streaming de fichiers vidéo locaux. Le frontend React correspondant se trouve dans le dossier `netflix_frontend/`.

---

## Sommaire

- [Fonctionnalités](#-fonctionnalités)
- [Prérequis](#-prérequis)
- [Installation](#-installation)
- [Configuration](#-configuration)
- [Lancer le serveur](#-lancer-le-serveur)
- [Structure du projet](#-structure-du-projet)
- [API Reference](#-api-reference)
- [Docker](#-docker)
- [Stack technique](#-stack-technique)

---

## ✨ Fonctionnalités

- 🔥 **Tendances TMDB** — Films et séries tendance de la semaine, paginés
- 🔍 **Recherche multi-contenus** — Films + séries en un seul appel
- 🎬 **Bandes-annonces YouTube** — Clés de trailers via l'API TMDB
- 📡 **Streaming vidéo local** — Serveur de fichiers statiques (`/stream/*`) pour lire des `.mp4` placés dans `assets/`
- 🌐 **CORS ouvert** — Compatible avec tout frontend React/Vite
- ⚡ **Asynchrone** — 100 % non-bloquant grâce à Tokio

---

## 🛠️ Prérequis

| Outil | Version minimale | Lien |
|---|---|---|
| Rust + Cargo | 1.80+ | https://rust-lang.org/tools/install |
| Git | toute version récente | https://git-scm.com |
| Clé API TMDB | — | https://www.themoviedb.org/settings/api |

> **Obtenir une clé TMDB :** Créez un compte gratuit sur [themoviedb.org](https://www.themoviedb.org), allez dans *Paramètres → API* et générez une clé v3.

---

## 📦 Installation

### 1. Cloner le dépôt

```bash
git clone https://github.com/ludovicdevio/netflix_backen_rust_axum.git
cd netflix_backen_rust_axum
```

### 2. Installer les dépendances Rust

```bash
cargo build
```

Cargo télécharge et compile toutes les dépendances automatiquement.

---

## ⚙️ Configuration

Créez un fichier `.env` à la racine du projet :

```bash
cp .env.example .env   # si le fichier exemple existe
# ou directement :
touch .env
```

Contenu du `.env` :

```env
# Clé API TMDB (obligatoire)
TMDB_API_KEY=votre_cle_api_tmdb_ici

# Port d'écoute du serveur (défaut : 8080)
PORT=8080
```

> ⚠️ Le fichier `.env` est dans `.gitignore` — ne le commitez jamais.

### Dossier assets (streaming vidéo local)

Pour activer le streaming de vidéos locales, créez le dossier `assets/` et placez-y des fichiers `.mp4` :

```bash
mkdir -p assets
# Exemple : copiez une vidéo
cp /chemin/vers/film.mp4 assets/123.mp4   # 123 = ID du film TMDB
```

Les fichiers seront accessibles via `GET /stream/{nom_du_fichier}`.

---

## 🚀 Lancer le serveur

### Mode développement

```bash
cargo run
```

Sortie attendue :

```
Server listening on http://0.0.0.0:8080
```

### Mode production (optimisé)

```bash
cargo build --release
./target/release/netflix_backend
```

---

## 📁 Structure du projet

```
netflix_backend/
├── src/
│   ├── main.rs       # Point d'entrée, configuration du routeur Axum, CORS
│   ├── handlers.rs   # Logique métier de chaque route API
│   ├── models.rs     # Structs Rust (Movie, Video, TmdbResponse, etc.)
│   └── state.rs      # État partagé de l'application (clé API, client HTTP)
├── assets/           # Fichiers vidéo locaux (gitignorés)
├── .env              # Variables d'environnement (gitignorées)
├── .gitignore
├── Cargo.toml        # Dépendances Rust
└── Dockerfile        # Image Docker pour la production
```

---

## 📡 API Reference

URL de base en développement : `http://localhost:8080`

### `GET /`

Vérifie que le serveur est en ligne.

```bash
curl http://localhost:8080/
# → "Le Backend Netflix API est en ligne !"
```

---

### `GET /api/trending`

Retourne les films et séries tendance de la semaine (source : TMDB).

**Paramètres de requête**

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `page` | `integer` | `1` | Numéro de page (max selon TMDB) |

**Exemple**

```bash
curl "http://localhost:8080/api/trending?page=1"
```

**Réponse**

```json
{
  "page": 1,
  "results": [
    {
      "id": 912649,
      "title": "Venom: The Last Dance",
      "name": null,
      "overview": "Eddie et Venom sont en fuite...",
      "poster_path": "/aosm8NMQ3UyoBVpSxyimorCQykC.jpg",
      "backdrop_path": "/3V4kLQg0kSqe6yYQfJuE7osYhDn.jpg",
      "vote_average": 6.8,
      "release_date": "2024-10-22",
      "media_type": "movie"
    }
  ],
  "total_pages": 1000
}
```

---

### `GET /api/search`

Recherche multi-contenus (films + séries) sur TMDB.

**Paramètres de requête**

| Paramètre | Type | Requis | Description |
|---|---|---|---|
| `query` | `string` | ✅ | Terme de recherche |
| `page` | `integer` | ❌ | Numéro de page (défaut : 1) |

**Exemple**

```bash
curl "http://localhost:8080/api/search?query=matrix&page=1"
```

**Réponse** — même structure que `/api/trending`.

---

### `GET /api/movie/{id}/videos`

Retourne les bandes-annonces et clips associés à un film (YouTube principalement).

**Paramètres de chemin**

| Paramètre | Type | Description |
|---|---|---|
| `id` | `integer` | ID TMDB du film |

**Exemple**

```bash
# Bandes-annonces de The Matrix (ID: 603)
curl http://localhost:8080/api/movie/603/videos
```

**Réponse**

```json
{
  "id": 603,
  "results": [
    {
      "id": "533ec654c3a36854480003eb",
      "key": "vKQi3bBA1Dg",
      "name": "The Matrix - Theatrical Trailer",
      "site": "YouTube",
      "type": "Trailer"
    }
  ]
}
```

Le champ `key` permet de construire l'URL YouTube : `https://www.youtube.com/watch?v={key}`

---

### `GET /stream/{filename}`

Sert les fichiers vidéo locaux depuis le dossier `assets/`. Supporte les requêtes HTTP Range (status 206) pour le seek dans le lecteur vidéo.

**Exemple**

```bash
# Vérifier qu'un fichier est accessible
curl -I http://localhost:8080/stream/video.mp4

# Ouvrir dans le navigateur
http://localhost:8080/stream/video.mp4
```

---

## 🐳 Docker

### Build et lancement seul

```bash
docker build -t netflix-backend .
docker run -p 8080:8080 --env-file .env netflix-backend
```

### Avec Docker Compose (backend + frontend ensemble)

Un fichier `compose.yml` global se trouve à la racine du monorepo (`../compose.yml`) :

```bash
# Depuis le dossier parent (qui contient netflix_backend/ et netflix_frontend/)
docker compose up --build
```

- Backend disponible sur : `http://localhost:8080`
- Frontend disponible sur : `http://localhost:80`

---

## 🧱 Stack technique

| Crate | Version | Rôle |
|---|---|---|
| `axum` | 0.8 | Framework web HTTP |
| `tokio` | 1.48 | Runtime asynchrone |
| `reqwest` | 0.12 | Client HTTP vers TMDB (TLS rustls) |
| `serde` / `serde_json` | 1.0 | Sérialisation / désérialisation JSON |
| `tower-http` | 0.6 | Middleware CORS + serveur de fichiers statiques |
| `dotenv` | 0.15 | Chargement des variables d'environnement |

---

## 🔗 Projets liés

- **Frontend React** : [`netflix_frontend/`](https://github.com/ludovicdevio/netflix_frontend   ) — Interface Netflix-like (React 19, TypeScript, Tailwind CSS v4, Vite)
