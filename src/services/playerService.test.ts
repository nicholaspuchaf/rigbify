import { describe, expect, it, vi } from 'vitest';
import { createPlayerService, shouldInitializeQueue } from './playerService';

describe('playerService', () => {
  it('maps queue and seek commands to the native contract', async () => {
    const invoke = vi.fn().mockResolvedValue({ status: 'paused' });
    const service = createPlayerService(invoke);
    await service.setQueue([{ id: '1', sourceUri: 'content://song', title: 'Song', artist: 'Artist', album: 'Album' }], 0, 1200);
    await service.seekTo(4500);
    expect(invoke).toHaveBeenNthCalledWith(1, 'plugin:media-player|set_queue', {
      payload: { tracks: expect.any(Array), startIndex: 0, positionMs: 1200 }
    });
    expect(invoke).toHaveBeenNthCalledWith(2, 'plugin:media-player|seek_to', { payload: { positionMs: 4500 } });
    const nativeQueuePayload = invoke.mock.calls[0][1]?.payload as Record<string, unknown>;
    expect(nativeQueuePayload).toHaveProperty('tracks');
    expect(nativeQueuePayload).not.toHaveProperty('payload');
  });

  it('uses the stable command names for playback controls', async () => {
    const invoke = vi.fn().mockResolvedValue({ status: 'playing' });
    const service = createPlayerService(invoke);
    await service.play(); await service.pause(); await service.skipNext();
    expect(invoke.mock.calls.map(([command]) => command)).toEqual([
      'plugin:media-player|play', 'plugin:media-player|pause', 'plugin:media-player|skip_next'
    ]);
  });
});

describe('shouldInitializeQueue', () => {
  const state = { status: 'paused' as const, currentTrackId: '1', positionMs: 0, durationMs: 10, queueIndex: 0, queueSize: 1, isPlaying: false };

  it('requires a queue when the native player is empty or has another track', () => {
    expect(shouldInitializeQueue(null, '1')).toBe(true);
    expect(shouldInitializeQueue({ ...state, queueSize: 0 }, '1')).toBe(true);
    expect(shouldInitializeQueue(state, '2')).toBe(true);
  });

  it('keeps an initialized queue for the current track', () => {
    expect(shouldInitializeQueue(state, '1')).toBe(false);
  });
});
