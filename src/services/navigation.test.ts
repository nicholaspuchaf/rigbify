import { describe, expect, it } from 'vitest';
import { fallbackView, viewBackStrategy } from './navigation';
import type { View } from '../types/music';

describe('navigation back strategy', () => {
  it('declares a return destination for every current view', () => {
    const views: View[] = ['home', 'search', 'library', 'library-collection', 'settings', 'player', 'playlist', 'playlist-add', 'playlist-edit', 'downloads'];
    expect(Object.keys(viewBackStrategy).sort()).toEqual([...views].sort());
  });

  it('keeps Home as the sole native exit point and nests playlist flows correctly', () => {
    expect(fallbackView('home')).toBeNull();
    expect(fallbackView('playlist')).toBe('library');
    expect(fallbackView('playlist-add')).toBe('playlist');
    expect(fallbackView('playlist-edit')).toBe('playlist');
    expect(fallbackView('library-collection')).toBe('library');
  });
});
