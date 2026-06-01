# Frontend

Simple attendance screen for the attendance backend APIs. The initial status and Clock In flows use the seeded PostgreSQL user `00000000-0000-0000-0005-000000000001`.

## Run the backend

Create and seed the PostgreSQL database with the DDL and seed files in `docs/database/`, then run the backend with `DATABASE_URL` set:

```sh
cd backend
DATABASE_URL=postgres://USER:PASSWORD@localhost:5432/rich_time_card cargo run
```

The backend listens on `http://localhost:3000`.

## Open the frontend page

From the repository root, start the small local frontend server:

```sh
node frontend/dev-server.mjs
```

Then open `http://127.0.0.1:5173` in a browser.

The frontend server serves `frontend/index.html` and proxies `/api/*` requests to the backend without changing API endpoint paths.

## Manual browser test

1. Open `http://127.0.0.1:5173`.
2. Confirm the current attendance status loads for the seeded database user.
3. Click **Clock In**.
4. Confirm the attendance status becomes `WORKING`.

Checkout, break-start, and break-end remain wired to their existing mock/in-memory backend behavior for now.
