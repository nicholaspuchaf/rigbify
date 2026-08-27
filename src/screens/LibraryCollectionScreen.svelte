<script lang="ts">
  import type { Collection, Track } from '../types/music';
  import Icon from '../components/Icon.svelte';
  import Artwork from '../components/Artwork.svelte';
  import TrackRow from '../components/TrackRow.svelte';

  export let collection: Collection;
  export let tracks: Track[];
  export let current: Track;
  export let onback: () => void;
  export let onplay: (track: Track, source: Track[]) => void;
  export let onfavorite: (track: Track) => void;
  export let onmore: (track: Track) => void;
</script>

<div class="screen library-collection-screen">
  <header class="playlist-hero"><button onclick={onback} aria-label="Voltar"><Icon name="back" size={28} /></button><Artwork hue={collection.hue} large label={`Capa de ${collection.title}`} /></header>
  <h1>{collection.title}</h1>
  <p class="playlist-description">{collection.subtitle}</p>
  {#if tracks.length}
    <div class="playlist-track-list">{#each tracks as track (track.id)}<TrackRow {track} active={current.id === track.id} onplay={(selected) => onplay(selected, tracks)} {onfavorite} {onmore} />{/each}</div>
  {:else}
    <div class="empty-state"><Icon name="music" size={38} /><h2>Nenhuma música encontrada</h2></div>
  {/if}
</div>
