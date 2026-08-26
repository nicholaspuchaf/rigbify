import { addPluginListener, invoke } from '@tauri-apps/api/core';
import type { DownloadMediaInfo, DownloadOptions, DownloadProgressListener, DownloadStatus, RegisterDownloadResult } from '../types/download';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;
type Listener = (plugin: string, event: string, callback: (payload: DownloadStatus) => void) => Promise<DownloadProgressListener>;

export function createDownloadService(call: Invoke = invoke, listen: Listener = addPluginListener) {
  return {
    getVersion: () => call<{ version: string; versionName: string }>('get_media_downloader_version'),
    updateYtdlp: (channel: 'stable' | 'nightly' = 'stable') => call<{ status: string; version: string; versionName: string }>('update_media_downloader', { channel }),
    getMediaInfo: (url: string) => call<DownloadMediaInfo>('get_download_media_info', { url }),
    start: (options: DownloadOptions) => call<DownloadStatus>('start_media_download', { options }),
    cancel: (downloadId: string) => call<DownloadStatus>('cancel_media_download', { downloadId }),
    getStatus: (downloadId: string) => call<DownloadStatus>('get_media_download_status', { downloadId }),
    registerCompleted: (downloadId: string) => call<RegisterDownloadResult>('register_completed_download', { downloadId }),
    onProgress: (callback: (status: DownloadStatus) => void) => listen('media-downloader', 'download-progress', callback)
  };
}

export const downloadService = createDownloadService();
