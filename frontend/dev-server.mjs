import { createReadStream } from "node:fs";
import { stat } from "node:fs/promises";
import { createServer } from "node:http";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const hostname = "127.0.0.1";
const port = 5173;
const backendOrigin = "http://127.0.0.1:3000";
const rootDir = fileURLToPath(new URL(".", import.meta.url));

const contentTypes = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
};

function sendJsonError(response, statusCode, message) {
  response.writeHead(statusCode, { "Content-Type": "application/json" });
  response.end(JSON.stringify({ error: message }));
}

async function proxyApi(request, response) {
  // Keep frontend calls same-origin while forwarding /api traffic to Axum.
  const target = new URL(request.url, backendOrigin);
  const backendResponse = await fetch(target, {
    method: request.method,
    headers: request.headers,
    body: request.method === "GET" || request.method === "HEAD" ? undefined : request,
    duplex: "half",
  });

  response.writeHead(backendResponse.status, Object.fromEntries(backendResponse.headers));

  if (backendResponse.body) {
    await backendResponse.body.pipeTo(
      new WritableStream({
        write(chunk) {
          response.write(chunk);
        },
        close() {
          response.end();
        },
      }),
    );
  } else {
    response.end();
  }
}

async function serveFile(request, response) {
  const requestPath = new URL(request.url, `http://${request.headers.host}`).pathname;
  const normalizedPath = normalize(requestPath === "/" ? "/index.html" : requestPath);
  const filePath = join(rootDir, normalizedPath);

  // Prevent paths such as /../ from escaping the frontend directory.
  if (!filePath.startsWith(rootDir)) {
    sendJsonError(response, 403, "Forbidden");
    return;
  }

  try {
    const fileStat = await stat(filePath);

    if (!fileStat.isFile()) {
      sendJsonError(response, 404, "Not found");
      return;
    }

    response.writeHead(200, {
      "Content-Type": contentTypes[extname(filePath)] ?? "application/octet-stream",
    });
    createReadStream(filePath).pipe(response);
  } catch {
    sendJsonError(response, 404, "Not found");
  }
}

const server = createServer(async (request, response) => {
  try {
    if (request.url?.startsWith("/api/")) {
      await proxyApi(request, response);
      return;
    }

    await serveFile(request, response);
  } catch (error) {
    sendJsonError(response, 502, error instanceof Error ? error.message : "Proxy error");
  }
});

server.listen(port, hostname, () => {
  console.log(`Frontend: http://${hostname}:${port}`);
  console.log(`Proxying API requests to ${backendOrigin}`);
});
