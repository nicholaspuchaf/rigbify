import { describe, expect, it, vi } from 'vitest';
import { createArtworkService } from './artworkService';

describe('artworkService', () => {
  it('converts cached file artwork through the native command only once', async () => {
    const invoke = vi.fn().mockResolvedValue('data:image/jpeg;base64,AA==');
    const service = createArtworkService(invoke);

    await expect(service.resolveArtworkSource('file:///cache/artwork/cover.jpg')).resolves.toBe('data:image/jpeg;base64,AA==');
    await expect(service.resolveArtworkSource('file:///cache/artwork/cover.jpg')).resolves.toBe('data:image/jpeg;base64,AA==');

    expect(invoke).toHaveBeenCalledTimes(1);
    expect(invoke).toHaveBeenCalledWith('load_artwork_data_uri', { uri: 'file:///cache/artwork/cover.jpg' });
  });

  it('keeps non-file artwork sources untouched', async () => {
    const invoke = vi.fn();
    const service = createArtworkService(invoke);

    await expect(service.resolveArtworkSource('https://example.test/cover.jpg')).resolves.toBe('https://example.test/cover.jpg');
    expect(invoke).not.toHaveBeenCalled();
  });

  it('limits concurrent native artwork reads', async () => {
    const complete: Array<(value: string) => void> = [];
    const mockedInvoke = vi.fn(() => new Promise<string>((resolve) => complete.push(resolve)));
    const service = createArtworkService(mockedInvoke as never);
    const requests = Array.from({ length: 5 }, (_, index) =>
      service.resolveArtworkSource(`file:///cache/artwork/${index}.jpg`)
    );

    expect(mockedInvoke).toHaveBeenCalledTimes(4);
    complete.shift()?.('data:image/jpeg;base64,first');
    await vi.waitFor(() => expect(mockedInvoke).toHaveBeenCalledTimes(5));

    complete.forEach((resolve) => resolve('data:image/jpeg;base64,next'));
    await Promise.all(requests);
  });
});
