export type View = 'home' | 'search' | 'library' | 'settings' | 'player';

export type Track = {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: number;
  hue: number;
  favorite?: boolean;
};

export type Collection = {
  id: string;
  title: string;
  subtitle: string;
  hue: number;
  type: 'playlist' | 'album' | 'artist';
};
