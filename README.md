# TrailSense Core
This repository houses the backend API for TrailSense. Written in ``rust`` using the ``axum`` web framework.

## Local development (mise + cargo)
Copy the `.env.example` file and adjust if necessary.
```sh
cp .env.example .env
```

Run local development via mise (starts TimescaleDB + backend live-reload task).
```sh
mise run dev
```

## Production deployment (Uncloud)
The production image is built from `Dockerfile` and deployed with `compose.production.yaml`.

Prepare production environment values:
```sh
cp .env.production.example .env.production
```
`.env.production` is the single source of runtime environment values for app and database.

Deploy to Uncloud:
```sh
mise run deploy
```
Migrations are applied automatically by the app on startup.

Redeploy after any change to `compose.production.yaml` or `.env.production`:
```sh
mise run deploy
```

The app replica count is declared in `compose.production.yaml` (`deploy.replicas`).
Use CLI scaling only for temporary overrides:
```sh
uc scale trailsense-core-app=3
```
