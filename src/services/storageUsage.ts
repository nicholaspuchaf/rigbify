import type { LibraryTrack } from '../types/library';

export type StorageUsage = {
  trackCount: number;
  knownSizeTrackCount: number;
  unknownSizeTrackCount: number;
  musicBytes: number;
};

export function calculateStorageUsage(tracks: LibraryTrack[]): StorageUsage {
  const knownSizes = tracks
    .map((track) => track.fileSize)
    .filter((size): size is number => typeof size === 'number' && Number.isFinite(size) && size >= 0);
  return {
    trackCount: tracks.length,
    knownSizeTrackCount: knownSizes.length,
    unknownSizeTrackCount: tracks.length - knownSizes.length,
    musicBytes: knownSizes.reduce((total, size) => total + size, 0)
  };
}

export function formatBytes(bytes: number): string {
  if (bytes < 1_024) return `${bytes} B`;
  const units = ['KB', 'MB', 'GB', 'TB'];
  let value = bytes;
  let unit = -1;
  do { value /= 1_024; unit += 1; } while (value >= 1_024 && unit < units.length - 1);
  return `${new Intl.NumberFormat('pt-BR', { maximumFractionDigits: value >= 10 ? 0 : 1 }).format(value)} ${units[unit]}`;
}
