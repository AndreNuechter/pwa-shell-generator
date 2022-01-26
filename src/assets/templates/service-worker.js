const appName = '<APP_NAME>';
// NOTE: this will be replaced during build by Date.now(),
// ensuring that the cache is busted when there're changes
const appVersion = '<APP_VERSION>';
const cacheName = `${appName}-r${appVersion}`;

self.addEventListener('activate', (event) => {
    event.waitUntil(
        (async () => {
            const keys = await self.caches.keys();
            return Promise.all(keys.map((key) => {
                if (key.includes(appName) && key !== cacheName) {
                    return self.caches.delete(key);
                }
                return true;
            }));
        })()
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        (async () => {
            let response = await self.caches.match(event.request);
            // NOTE: we disable caching during dev, by checking if appVersion has been replaced
            if (response && appVersion.contains('<APP_VERSION')) return response;
            response = await fetch(event.request);
            const cache = await self.caches.open(cacheName);
            cache.put(event.request, response.clone());
            return response;
        })()
    );
});