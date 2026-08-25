import { describe, expect, it, vi } from 'vitest';
import { createPlaylistService } from './playlistService';

describe('playlistService', () => {
  it('maps batch addition and optimistic ordering contracts', async () => {
    const invoke = vi.fn().mockResolvedValue({ id: 'p1', tracks: [] });
    const service = createPlaylistService(invoke);
    await service.addTracks('p1', ['t1', 't2']);
    await service.replaceOrder('p1', ['t2'], 4);
    expect(invoke).toHaveBeenNthCalledWith(1, 'add_tracks_to_playlist', { id: 'p1', trackIds: ['t1', 't2'] });
    expect(invoke).toHaveBeenNthCalledWith(2, 'replace_playlist_order', { id: 'p1', input: { trackIds: ['t2'], expectedVersion: 4 } });
  });

  it('uses descending order for recently added when requested', async () => {
    const invoke = vi.fn().mockResolvedValue({ id: 'p1', tracks: [] });
    await createPlaylistService(invoke).setSort('p1', 'recently_added', 'descending');
    expect(invoke).toHaveBeenCalledWith('set_playlist_sort', { id: 'p1', mode: 'recently_added', direction: 'descending' });
  });
});
