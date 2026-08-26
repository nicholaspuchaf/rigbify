<script lang="ts">
  import type { Collection, Track } from '../types/music'; import Avatar from '../components/Avatar.svelte'; import ChipBar from '../components/ChipBar.svelte'; import SectionHeading from '../components/SectionHeading.svelte'; import CollectionCard from '../components/CollectionCard.svelte'; import TrackRow from '../components/TrackRow.svelte';
  export let tracks: Track[]; export let collections: Collection[]; export let current: Track; export let onplay: (t: Track) => void; export let onfavorite: (t: Track) => void; export let onmore: (t: Track) => void; export let onsettings: () => void; export let onadd: () => void; export let ondownload: () => void; export let oncollection: (item: Collection) => void;
  let filter = 'Tudo';
</script>
<div class="screen home-screen"><div class="home-top"><Avatar onclick={onsettings} /><ChipBar items={['Tudo','Músicas','Playlists']} selected={filter} onselect={(value) => filter = value} /></div>
  <section class="quick-grid" aria-label="Acesso rápido">{#each collections as item (item.id)}<CollectionCard {item} onopen={oncollection} />{/each}</section>
  <section><SectionHeading title="Feito para você" /><div class="feature-scroll">{#each collections.slice(1) as item (item.id)}<article class="feature-card" style={`--card-hue:${item.hue}`}><span class="feature-mark">R</span><small>MISTURA LOCAL</small><h3>{item.title}</h3><p>{item.subtitle}</p></article>{/each}</div></section>
  <button class="import-banner" onclick={onadd}><span><b>Adicione suas músicas</b><small>MP3, M4A, OGG, WAV e FLAC</small></span><strong>＋</strong></button>
  <button class="download-banner" onclick={ondownload}><span><b>Baixar mídia autorizada</b><small>YouTube e YouTube Music</small></span><strong>↓</strong></button>
  <section class="continue-card"><div><span class="eyebrow">CONTINUE OUVINDO</span><h2>{current.title}</h2><p>{current.artist} · {current.album}</p></div><button onclick={() => onplay(current)} aria-label={`Tocar ${current.title}`}>▶</button></section>
  <section><SectionHeading title="Adicionadas recentemente" action="Ver tudo" />{#each tracks.slice(0,4) as track (track.id)}<TrackRow {track} active={current.id === track.id} {onplay} {onfavorite} {onmore} />{/each}</section>
</div>
