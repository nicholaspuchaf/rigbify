export type View = 'home' | 'search' | 'library' | 'settings' | 'player' | 'playlist' | 'playlist-add' | 'playlist-edit';

export type Track = {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: number;
  hue: number;
  favorite?: boolean;
  sourceUri?: string;
  artworkUri?: string;
};

export type Collection = {
  id: string;
  title: string;
  subtitle: string;
  hue: number;
  type: 'playlist' | 'album' | 'artist';
};
