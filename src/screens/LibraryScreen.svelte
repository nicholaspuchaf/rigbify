<script lang="ts">
  import type { Collection, Track } from '../types/music'; import type { PlaylistListSort } from '../services/libraryCollections'; import ScreenHeader from '../components/ScreenHeader.svelte'; import ChipBar from '../components/ChipBar.svelte'; import Icon from '../components/Icon.svelte'; import Artwork from '../components/Artwork.svelte'; import TrackRow from '../components/TrackRow.svelte'; import CollectionCard from '../components/CollectionCard.svelte';
  export let tracks: Track[]; export let collections: Collection[]; export let current: Track; export let onplay: (t: Track) => void; export let onfavorite: (t: Track) => void; export let onmore: (t: Track) => void; export let onplaylist: (item: Collection) => void; export let oncollection: (item: Collection) => void; export let oncreateplaylist: () => void; export let onsettings: () => void; export let onadd: () => void; export let onimportbundle: () => void; export let importingbundle = false; export let playlistSort: PlaylistListSort; export let onplaylistsort: (sort: PlaylistListSort) => void;
  let filter = 'Músicas'; let grid = false; let sortOpen = false;
  const sortLabels: Record<PlaylistListSort, string> = { recent: 'Recentes', 'name-ascending': 'Nome: A–Z', 'name-descending': 'Nome: Z–A' };
</script>
<div class="screen">
  <ScreenHeader title="Sua biblioteca" {onsettings} action="plus" onaction={onadd} />
  <ChipBar items={['Músicas','Playlists','Álbuns','Artistas']} selected={filter} onselect={(value) => filter = value} />
  <div class="library-toolbar"><button onclick={() => sortOpen = true} aria-haspopup="dialog"><Icon name="sort" size={18} /> {sortLabels[playlistSort]}</button><button onclick={onimportbundle} disabled={importingbundle}>{importingbundle ? 'Importando bundle…' : 'Importar bundle'}</button><button onclick={() => grid = !grid} aria-label="Alternar visualização"><Icon name="grid" size={20} /></button></div>
  {#if filter === 'Músicas'}
    <div class:grid class="library-content">{#each tracks as track (track.id)}{#if grid}<button class="grid-track" onclick={() => onplay(track)} oncontextmenu={(event) => { event.preventDefault(); onmore(track); }}><Artwork hue={track.hue} src={track.artworkUri} label={`Capa de ${track.title}`} /><b>{track.title}</b><small>{track.artist}</small></button>{:else}<TrackRow {track} active={current.id === track.id} {onplay} {onfavorite} {onmore} />{/if}{/each}</div>
  {:else if filter === 'Playlists'}
    <button class="create-playlist-card" onclick={oncreateplaylist}>＋ <span><b>Criar playlist</b><small>Monte sua própria sequência</small></span></button>
    <div class="library-collections">{#each collections.filter(item => item.type === 'playlist') as item (item.id)}<CollectionCard {item} onopen={onplaylist} />{/each}</div>
  {:else}
    <div class="library-collections">{#each collections.filter(item => item.type === filter.slice(0,-1).toLocaleLowerCase('pt-BR')) as item (item.id)}<CollectionCard {item} onopen={oncollection} />{:else}<p class="empty-state">Nenhum {filter.toLocaleLowerCase('pt-BR')} encontrado.</p>{/each}</div>
  {/if}
</div>
{#if sortOpen}<div class="modal-backdrop" role="presentation" onclick={(event) => { if (event.target === event.currentTarget) sortOpen = false; }}><div class="action-sheet sort-sheet" role="dialog" aria-modal="true" aria-label="Ordenar playlists"><i></i><h2>Ordenar playlists</h2><div role="radiogroup">{#each Object.entries(sortLabels) as [value, label] (value)}<button role="radio" aria-checked={playlistSort === value} onclick={() => { onplaylistsort(value as PlaylistListSort); sortOpen = false; }}><span>{label}</span>{#if playlistSort === value}<Icon name="check" />{/if}</button>{/each}</div></div></div>{/if}
