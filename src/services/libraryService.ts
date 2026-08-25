import { invoke } from '@tauri-apps/api/core';
import type { LibraryTrack, NewLibraryTrack, TrackMetadataUpdate } from '../types/library';

export interface LibraryService {
  listTracks(): Promise<LibraryTrack[]>;
  importFromDevice(): Promise<LibraryTrack[]>;
  importTrack(input: NewLibraryTrack): Promise<LibraryTrack>;
  setFavorite(id: string, favorite: boolean): Promise<LibraryTrack>;
  removeTrack(id: string): Promise<void>;
  updateMetadata(id: string, metadata: TrackMetadataUpdate): Promise<LibraryTrack>;
  addToPlaylist(trackId: string, playlistId: string, playlistName: string): Promise<void>;
}

class TauriLibraryService implements LibraryService {
  listTracks(): Promise<LibraryTrack[]> {
    return invoke<LibraryTrack[]>('list_tracks');
  }

  importTrack(input: NewLibraryTrack): Promise<LibraryTrack> {
    return invoke<LibraryTrack>('import_track', { input });
  }

  async importFromDevice(): Promise<LibraryTrack[]> {
    const selected = await invoke<NewLibraryTrack[]>('plugin:media-source|pick_audio');
    return Promise.all(selected.map((input) => this.importTrack(input)));
  }

  setFavorite(id: string, favorite: boolean): Promise<LibraryTrack> {
    return invoke<LibraryTrack>('set_track_favorite', { id, favorite });
  }

  removeTrack(id: string): Promise<void> {
    return invoke<void>('remove_track', { id });
  }

  updateMetadata(id: string, metadata: TrackMetadataUpdate): Promise<LibraryTrack> {
    return invoke<LibraryTrack>('update_track_metadata', { id, metadata });
  }

  addToPlaylist(trackId: string, playlistId: string, playlistName: string): Promise<void> {
    return invoke<void>('add_track_to_playlist', { trackId, playlistId, playlistName });
  }
}

export const libraryService: LibraryService = new TauriLibraryService();
