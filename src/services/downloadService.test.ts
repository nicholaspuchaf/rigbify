import { describe, expect, it, vi } from 'vitest';
import { createDownloadService } from './downloadService';

describe('downloadService', () => {
  it('uses typed app commands instead of exposing yt-dlp arguments', async () => {
    const invoke = vi.fn().mockResolvedValue({ downloadId: 'download-1' });
    const listen = vi.fn();
    const service = createDownloadService(invoke, listen);
    const options = { source: { kind: 'single' as const, url: 'https://youtu.be/abc' }, outputFormat: 'mp3' as const, mediaKind: 'audio' as const, title: 'Song', sourceId: 'abc' };
    await service.start(options);
    await service.cancel('download-1');
    expect(invoke).toHaveBeenNthCalledWith(1, 'start_media_download', { options });
    expect(invoke).toHaveBeenNthCalledWith(2, 'cancel_media_download', { downloadId: 'download-1' });
  });
});

