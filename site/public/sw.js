const CACHE = "mia-site-v10";
const SHELL = ["/demo/", "/privacy/", "/terms/", "/404.html", "/favicon.svg", "/mount-ledger-6b7fee8c.webp"];

self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    const home = await fetch("/", { cache: "reload" });
    const html = await home.clone().text();
    const builtAssets = [...html.matchAll(/(?:src|href)="(\/assets\/[^"]+)"/g)].map((match) => match[1]);
    await cache.put("/", home);
    const resources = [...SHELL, ...new Set(builtAssets)];
    await Promise.all(resources.map(async (url) => {
      const response = await fetch(url, { cache: "reload" });
      if (!response.ok) throw new Error(`Cannot precache ${url}: ${response.status}`);
      await cache.put(url, response);
    }));
  })());
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key)))));
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || new URL(event.request.url).origin !== self.location.origin) return;
  event.respondWith(fetch(event.request).then((response) => {
    const copy = response.clone();
    caches.open(CACHE).then((cache) => cache.put(event.request, copy));
    return response;
  }).catch(async () => {
    const cached = await caches.match(event.request, { ignoreVary: true });
    if (cached) return cached;
    if (event.request.mode === "navigate") return caches.match("/");
    return Response.error();
  }));
});
