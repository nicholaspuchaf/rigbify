import { invoke } from '@tauri-apps/api/core';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

export function createArtworkService(tauriInvoke: Invoke = invoke): { resolveArtworkSource(uri: string | undefined): Promise<string | undefined> } {
  const dataUrls = new Map<string, Promise<string | undefined>>();

  return {
    resolveArtworkSource(uri) {
      if (!uri || !uri.startsWith('file:')) return Promise.resolve(uri);
      const cached = dataUrls.get(uri);
      if (cached) return cached;

      const request = tauriInvoke<string | null>('load_artwork_data_uri', { uri })
        .then((dataUrl) => dataUrl ?? undefined)
        .catch(() => undefined);
      dataUrls.set(uri, request);
      return request;
    }
  };
}

export const { resolveArtworkSource } = createArtworkService();
