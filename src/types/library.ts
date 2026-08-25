export type MediaSourceType = 'local_document' | 'app_file' | 'remote_download';
export type MediaAvailability = 'available' | 'missing' | 'downloading' | 'failed';

export type LibraryTrack = {
  id: string;
  sourceType: MediaSourceType;
  sourceUri: string;
  title: string;
  artist: string | null;
  album: string | null;
  durationMs: number;
  artworkUri: string | null;
  mimeType: string | null;
  fileSize: number | null;
  trackNumber: number | null;
  discNumber: number | null;
  year: number | null;
  genre: string | null;
  addedAt: number;
  updatedAt: number;
  lastPlayedAt: number | null;
  playCount: number;
  favorite: boolean;
  availability: MediaAvailability;
};

export type NewLibraryTrack = {
  sourceType: MediaSourceType;
  sourceUri: string;
  title?: string;
  fileName?: string;
  artist?: string;
  album?: string;
  durationMs: number;
  artworkUri?: string;
  mimeType?: string;
  fileSize?: number;
  trackNumber?: number;
  discNumber?: number;
  year?: number;
  genre?: string;
};
