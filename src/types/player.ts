import type { Track } from './music';

export type PlaybackStatus = 'idle' | 'loading' | 'playing' | 'paused' | 'stopped' | 'error';

export type PlayerSnapshot = {
  status: PlaybackStatus;
  currentTrackId: string | null;
  positionMs: number;
  durationMs: number;
  queueIndex: number;
  queueSize: number;
  isPlaying: boolean;
};

export type NativeQueueTrack = Pick<Track, 'id' | 'title' | 'artist' | 'album'> & {
  sourceUri: string;
  artworkUri?: string;
};
