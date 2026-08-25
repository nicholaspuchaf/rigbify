<script lang="ts">
  import type { Track, View } from './types/music'; import { collections, tracks as initialTracks } from './data/mock';
  import HomeScreen from './screens/HomeScreen.svelte'; import SearchScreen from './screens/SearchScreen.svelte'; import LibraryScreen from './screens/LibraryScreen.svelte'; import SettingsScreen from './screens/SettingsScreen.svelte'; import PlayerScreen from './screens/PlayerScreen.svelte'; import MiniPlayer from './components/MiniPlayer.svelte'; import BottomNavigation from './components/BottomNavigation.svelte';
  let view: View = 'home'; let previousView: View = 'home'; let tracks = initialTracks; let current = tracks[0]; let playing = false;
  function navigate(next: View) { view = next; }
  function openSettings() { previousView = view; view = 'settings'; }
  function openPlayer() { previousView = view; view = 'player'; }
  function play(track: Track) { current = track; playing = true; }
  function toggle() { playing = !playing; }
  function step(direction: number) { const index = tracks.findIndex((track) => track.id === current.id); current = tracks[(index + direction + tracks.length) % tracks.length]; playing = true; }
  function favorite(track: Track) { tracks = tracks.map((item) => item.id === track.id ? { ...item, favorite: !item.favorite } : item); current = tracks.find((item) => item.id === current.id) ?? current; }
</script>

{#if view === 'player'}<PlayerScreen track={current} {playing} onback={() => view = previousView} ontoggle={toggle} onnext={() => step(1)} onprevious={() => step(-1)} onfavorite={favorite} />
{:else}<div class="app-shell">{#if view === 'home'}<HomeScreen {tracks} {collections} {current} onplay={play} onfavorite={favorite} onsettings={openSettings} />{:else if view === 'search'}<SearchScreen {tracks} {current} onplay={play} onfavorite={favorite} onsettings={openSettings} />{:else if view === 'library'}<LibraryScreen {tracks} {collections} {current} onplay={play} onfavorite={favorite} onsettings={openSettings} />{:else}<SettingsScreen onback={() => view = previousView} />{/if}</div>
  <MiniPlayer track={current} {playing} onopen={openPlayer} ontoggle={toggle} /><BottomNavigation active={view} {navigate} />
{/if}
