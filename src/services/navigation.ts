import type { View } from '../types/music';

export const viewBackStrategy: Record<View, View | 'native'> = {
  home: 'native',
  search: 'home',
  library: 'home',
  'library-collection': 'library',
  settings: 'home',
  player: 'home',
  playlist: 'library',
  'playlist-add': 'playlist',
  'playlist-edit': 'playlist',
  downloads: 'home'
};

export function fallbackView(view: View): View | null {
  const strategy = viewBackStrategy[view];
  return strategy === 'native' ? null : strategy;
}
