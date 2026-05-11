# Frontend

Simple attendance screen for the existing mock backend APIs.

## Run the backend

```sh
cd backend
cargo run
```

The backend listens on `http://localhost:3000`.

## Open the frontend page

From the repository root, start the small local frontend server:

```sh
node frontend/dev-server.mjs
```

Then open `http://127.0.0.1:5173` in a browser.

The frontend server serves `frontend/index.html` and proxies `/api/*` requests to the backend so the page can call the mock APIs without changing backend code.
