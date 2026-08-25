<script lang="ts">
  import type { Track } from '../types/music'; import ScreenHeader from '../components/ScreenHeader.svelte'; import Icon from '../components/Icon.svelte'; import TrackRow from '../components/TrackRow.svelte'; import SectionHeading from '../components/SectionHeading.svelte';
  export let tracks: Track[]; export let current: Track; export let onplay: (t: Track) => void; export let onfavorite: (t: Track) => void; export let onsettings: () => void;
  let query = '';
  $: results = tracks.filter((track) => `${track.title} ${track.artist} ${track.album}`.toLocaleLowerCase('pt-BR').includes(query.toLocaleLowerCase('pt-BR')));
  const categories = [{name:'Músicas',hue:330},{name:'Artistas',hue:24},{name:'Álbuns',hue:208},{name:'Playlists',hue:102},{name:'Pastas',hue:272},{name:'Formatos',hue:45}];
</script>
<div class="screen"><ScreenHeader title="Buscar" {onsettings} />
  <label class="search-field"><Icon name="search" size={25} /><span class="sr-only">Buscar músicas, artistas ou álbuns</span><input bind:value={query} placeholder="O que você quer ouvir?" />{#if query}<button onclick={() => query = ''} aria-label="Limpar busca">×</button>{/if}</label>
  {#if query}<section><SectionHeading title="Resultados" action={`${results.length} encontrados`} />{#each results as track (track.id)}<TrackRow {track} active={current.id === track.id} {onplay} {onfavorite} />{:else}<div class="empty-state"><Icon name="search" size={34} /><h2>Nada por aqui</h2><p>Tente buscar outro título, artista ou álbum.</p></div>{/each}</section>
  {:else}<section><SectionHeading title="Explore sua biblioteca" /><div class="category-grid">{#each categories as category (category.name)}<button class="category-card" style={`--category-hue:${category.hue}`}><b>{category.name}</b><span><Icon name="music" size={29} /></span></button>{/each}</div></section>{/if}
</div>
