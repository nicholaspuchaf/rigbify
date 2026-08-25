<script lang="ts">
  import type { Collection, Track } from '../types/music'; import ScreenHeader from '../components/ScreenHeader.svelte'; import ChipBar from '../components/ChipBar.svelte'; import Icon from '../components/Icon.svelte'; import TrackRow from '../components/TrackRow.svelte'; import CollectionCard from '../components/CollectionCard.svelte';
  export let tracks: Track[]; export let collections: Collection[]; export let current: Track; export let onplay: (t: Track) => void; export let onfavorite: (t: Track) => void; export let onmore: (t: Track) => void; export let onplaylist: (item: Collection) => void; export let oncreateplaylist: () => void; export let onsettings: () => void; export let onadd: () => void;
  let filter = 'Músicas'; let grid = false;
</script>
<div class="screen">
  <ScreenHeader title="Sua biblioteca" {onsettings} action="plus" onaction={onadd} />
  <ChipBar items={['Músicas','Playlists','Álbuns','Artistas']} selected={filter} onselect={(value) => filter = value} />
  <div class="library-toolbar"><button><Icon name="sort" size={18} /> Recentes</button><button onclick={() => grid = !grid} aria-label="Alternar visualização"><Icon name="grid" size={20} /></button></div>
  {#if filter === 'Músicas'}
    <div class:grid class="library-content">{#each tracks as track (track.id)}{#if grid}<button class="grid-track" onclick={() => onplay(track)} oncontextmenu={(event) => { event.preventDefault(); onmore(track); }}><span class="grid-art" style={`--h:${track.hue}`}>♫</span><b>{track.title}</b><small>{track.artist}</small></button>{:else}<TrackRow {track} active={current.id === track.id} {onplay} {onfavorite} {onmore} />{/if}{/each}</div>
  {:else if filter === 'Playlists'}
    <button class="create-playlist-card" onclick={oncreateplaylist}>＋ <span><b>Criar playlist</b><small>Monte sua própria sequência</small></span></button>
    <div class="library-collections">{#each collections.filter(item => item.type === 'playlist') as item (item.id)}<CollectionCard {item} onopen={onplaylist} />{/each}</div>
  {:else}
    <div class="library-collections">{#each collections.filter(item => item.type === filter.slice(0,-1).toLocaleLowerCase('pt-BR')) as item (item.id)}<CollectionCard {item} />{/each}</div>
  {/if}
</div>
