# DevOps Rust Project - TP1

## Description

- Mettre en place son environnement de développement et les bases du projet avec le moins de dépendances possibles
- Développer une API qui retourne au format JSON les headers de la requête quand il y une requête HTTP GET sur **`/ping`**
- Le serveur doit écouter sur un port configurable via la variable d'environnement : PING_LISTEN_PORT ou par défaut sur un port au choix
- Réponse vide avec code 404 si quoi que ça soit d'autre que GET **`/ping`**

## Configuration

Le port sur lequel le serveur écoute peut être configuré via la variable d'environnement **`PING_LISTEN_PORT`**.

Si cette variable n'est pas définie, le serveur utilise le port par défaut 8080.

Afin de définir la variable d'environnement, il faut utiliser la commande **`export`** dans le terminal avant de build le projet.

```bash
export PING_LISTEN_PORT=8099
```

## Installation

### 1. Cloner le projet

```bash
git clone https://github.com/leo-grandmorcel/B3-Devops
cd B3-Devops/TP1
```

### 2. Construire et run le projet avec Cargo

```bash
cargo build
cargo run
```

Le serveur devrait être accessible à l'adresse http://127.0.0.1:8080 (ou sur le port spécifié par **PING_LISTEN_PORT**)
