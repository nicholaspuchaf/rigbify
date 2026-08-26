import { invoke } from '@tauri-apps/api/core';
import type { NativeQueueTrack, PlayerSnapshot } from '../types/player';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

export function createPlayerService(call: Invoke = invoke) {
  const run = (command: string) => call<PlayerSnapshot>(`plugin:media-player|${command}`);
  return {
    setQueue: (tracks: NativeQueueTrack[], startIndex = 0, positionMs = 0) =>
      call<PlayerSnapshot>('plugin:media-player|set_queue', { payload: { tracks, startIndex, positionMs } }),
    play: () => run('play'),
    pause: () => run('pause'),
    stop: () => run('stop'),
    skipNext: () => run('skip_next'),
    skipPrevious: () => run('skip_previous'),
    seekTo: (positionMs: number) => call<PlayerSnapshot>('plugin:media-player|seek_to', { payload: { positionMs } }),
    getState: () => run('get_state')
  };
}

export const playerService = createPlayerService();

export function calculatePlaybackProgress(positionSeconds: number, durationSeconds: number): number {
  if (!Number.isFinite(positionSeconds) || !Number.isFinite(durationSeconds) || durationSeconds <= 0) return 0;
  return Math.min(100, Math.max(0, positionSeconds / durationSeconds * 100));
}

export function shouldInitializeQueue(state: PlayerSnapshot | null, trackId: string): boolean {
  return state === null || state.queueSize === 0 || state.currentTrackId !== trackId;
}
