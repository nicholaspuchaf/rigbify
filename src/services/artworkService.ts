import { invoke } from '@tauri-apps/api/core';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;
const MAX_CONCURRENT_LOADS = 4;

function createQueue() {
  let running = 0;
  const pending: Array<() => void> = [];

  function runNext() {
    if (running >= MAX_CONCURRENT_LOADS) return;
    pending.shift()?.();
  }

  return function enqueue<T>(operation: () => Promise<T>): Promise<T> {
    return new Promise((resolve, reject) => {
      pending.push(() => {
        running += 1;
        void operation().then(resolve, reject).finally(() => {
          running -= 1;
          runNext();
        });
      });
      runNext();
    });
  };
}

export function createArtworkService(tauriInvoke: Invoke = invoke): { resolveArtworkSource(uri: string | undefined): Promise<string | undefined> } {
  const dataUrls = new Map<string, Promise<string | undefined>>();
  const enqueue = createQueue();

  return {
    resolveArtworkSource(uri) {
      if (!uri || !uri.startsWith('file:')) return Promise.resolve(uri);
      const cached = dataUrls.get(uri);
      if (cached) return cached;

      const request = enqueue(() => tauriInvoke<string | null>('load_artwork_data_uri', { uri }))
        .then((dataUrl) => dataUrl ?? undefined)
        .catch(() => undefined);
      dataUrls.set(uri, request);
      return request;
    }
  };
}

export const { resolveArtworkSource } = createArtworkService();
