import type { Collection, Track } from '../types/music';

export const tracks: Track[] = [
  { id: '1', title: 'Midnight Drive', artist: 'Neon Harbour', album: 'After Hours', duration: 238, hue: 266, favorite: true, playCount: 18 },
  { id: '2', title: 'Sol de Inverno', artist: 'Aurora Sul', album: 'Horizonte', duration: 194, hue: 34, playCount: 12 },
  { id: '3', title: 'Static Hearts', artist: 'The Low Tides', album: 'Signals', duration: 221, hue: 188, playCount: 8 },
  { id: '4', title: 'Café às Seis', artist: 'Violeta', album: 'Dias Comuns', duration: 176, hue: 328, playCount: 5 },
  { id: '5', title: 'Weightless', artist: 'Lumen', album: 'Bloom', duration: 263, hue: 92, playCount: 3 },
  { id: '6', title: 'Cidade Acesa', artist: 'Norte', album: 'Concreto', duration: 207, hue: 14, playCount: 1 }
];

export const collections: Collection[] = [
  { id: 'liked', title: 'Músicas favoritas', subtitle: '6 músicas', hue: 260, type: 'playlist' },
  { id: 'recent', title: 'Chegaram agora', subtitle: 'Adicionadas esta semana', hue: 42, type: 'playlist' },
  { id: 'night', title: 'Noite adentro', subtitle: 'Sua mistura tranquila', hue: 205, type: 'playlist' },
  { id: 'focus', title: 'Modo foco', subtitle: 'Sem distrações', hue: 110, type: 'playlist' }
];
