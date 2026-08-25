<script lang="ts">
  import { SvelteSet } from 'svelte/reactivity';
  import type { Track } from '../types/music';
  import Icon from '../components/Icon.svelte';
  import Artwork from '../components/Artwork.svelte';

  export let name: string;
  export let tracks: Track[];
  export let existingIds: string[];
  export let onback: () => void;
  export let onadd: (ids: string[]) => Promise<void>;
  export let ondone: () => void;

  let query = '';
  let filter: 'all' | 'favorites' | 'recent' = 'all';
  let added = new SvelteSet<string>();
  let adding = new SvelteSet<string>();
  let error = '';

  $: visible = tracks.filter(
    (track) =>
      (filter !== 'favorites' || track.favorite) &&
      `${track.title} ${track.artist} ${track.album}`
        .toLocaleLowerCase('pt-BR')
        .includes(query.toLocaleLowerCase('pt-BR'))
  );

  function isAdded(id: string) {
    return existingIds.includes(id) || added.has(id);
  }

  async function add(track: Track) {
    if (isAdded(track.id) || adding.has(track.id)) return;
    error = '';
    adding.add(track.id);
    try {
      await onadd([track.id]);
      added.add(track.id);
    } catch (reason) {
      error = reason instanceof Error ? reason.message : String(reason);
    } finally {
      adding.delete(track.id);
    }
  }
</script>

<div class="screen playlist-add-screen">
  <header class="subpage-header">
    <button onclick={onback} aria-label="Voltar"><Icon name="back" /></button>
    <div><h1>Adicionar à playlist</h1><small>{name}</small></div>
  </header>

  <div class="chip-bar">
    <button class:active={filter === 'all'} onclick={() => (filter = 'all')}>Músicas</button>
    <button class:active={filter === 'favorites'} onclick={() => (filter = 'favorites')}>Favoritas</button>
    <button class:active={filter === 'recent'} onclick={() => (filter = 'recent')}>Recentes</button>
  </div>

  <label class="playlist-search">
    <Icon name="search" />
    <input bind:value={query} placeholder="O que você gostaria de adicionar?" />
  </label>

  {#if error}
    <p class="playlist-add-error" role="alert">Não foi possível adicionar: {error}</p>
  {/if}

  <div>
    {#each visible as track (track.id)}
      <div class="add-track-row">
        <Artwork hue={track.hue} label="" />
        <span><b>{track.title}</b><small>{track.artist} · {track.album}</small></span>
        <button
          class:selected={isAdded(track.id)}
          disabled={isAdded(track.id) || adding.has(track.id)}
          onclick={() => add(track)}
          aria-label={isAdded(track.id) ? `${track.title} já adicionada` : `Adicionar ${track.title} à ${name}`}
        >
          {adding.has(track.id) ? '…' : isAdded(track.id) ? '✓' : '＋'}
        </button>
      </div>
    {:else}
      <div class="empty-state">
        <h2>Nenhuma música disponível</h2>
        <p>Nenhuma música corresponde à busca ou ao filtro.</p>
      </div>
    {/each}
  </div>

  {#if added.size}
    <button class="floating-confirm" onclick={ondone}>
      Concluir · {added.size} {added.size === 1 ? 'música adicionada' : 'músicas adicionadas'}
    </button>
  {/if}
</div>
