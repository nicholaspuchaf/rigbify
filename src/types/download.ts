import type { LibraryTrack } from './library';

export type MediaFormat = {
  id: string; container: string | null; audioCodec: string | null; videoCodec: string | null;
  bitrate: number | null; filesize: number | null; resolution: string | null;
  audioOnly: boolean; videoOnly: boolean;
};
export type DownloadMediaInfo = {
  id: string; url: string; title: string; artist: string | null; channel: string | null;
  album: string | null; thumbnail: string | null; durationMs: number | null; formats: MediaFormat[];
};
export type OutputFormat = 'original' | 'mp3' | 'm4a' | 'opus' | 'mp4';
export type DownloadMediaKind = 'audio' | 'video';
export type DownloadOptions = {
  source: { kind: 'single' | 'playlist'; url: string };
  outputFormat: OutputFormat; mediaKind: DownloadMediaKind; quality?: string; formatId?: string;
  title: string; artist?: string; album?: string; thumbnail?: string; durationMs?: number; sourceId: string;
};
export type DownloadStatusName = 'download_started' | 'download_progress' | 'post_processing' | 'completed' | 'failed' | 'cancelled';
export type DownloadStatus = {
  jobId: string; downloadId: string; status: DownloadStatusName; progress: number;
  downloadedBytes: number; totalBytes: number | null; speed: number | null; eta: number | null;
  message: string | null; sourceUri: string | null; fileSize: number | null;
  title: string | null; artist: string | null; album: string | null; thumbnail: string | null;
  durationMs: number | null; sourceUrl: string | null; sourceId: string | null;
};
export type DownloadProgressListener = { unregister(): Promise<void> };
export type RegisterDownloadResult = LibraryTrack;

