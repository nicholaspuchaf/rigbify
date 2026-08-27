import { describe, expect, it } from 'vitest';
import { mostPlayedTracks } from './homeRecommendations';

describe('home recommendations', () => {
  it('prioritizes listening count and has a stable title tie-breaker', () => {
    const tracks = [
      { id: '1', title: 'Zebra', artist: '', album: '', duration: 1, hue: 0, playCount: 3 },
      { id: '2', title: 'Água', artist: '', album: '', duration: 1, hue: 0, playCount: 5 },
      { id: '3', title: 'Beta', artist: '', album: '', duration: 1, hue: 0, playCount: 3 }
    ];
    expect(mostPlayedTracks(tracks, 2).map(({ id }) => id)).toEqual(['2', '3']);
  });
});
