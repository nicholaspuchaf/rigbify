<script lang="ts">
  import type { Track } from '../types/music'; import Artwork from './Artwork.svelte'; import Icon from './Icon.svelte';
  export let track: Track; export let active = false; export let onplay: (track: Track) => void; export let onfavorite: (track: Track) => void; export let onmore: (track: Track) => void;
</script>
<div class:active class="track-row"><button class="track-main" onclick={() => onplay(track)}><Artwork hue={track.hue} label={`Capa de ${track.title}`} /><span><b>{track.title}</b><small>{track.artist} · {track.album}</small></span></button><button class:liked={track.favorite} class="row-action" onclick={() => onfavorite(track)} aria-label={`${track.favorite ? 'Remover' : 'Adicionar'} ${track.title} dos favoritos`}><Icon name="heart" size={19} /></button><button class="row-action" onclick={() => onmore(track)} aria-label={`Mais opções para ${track.title}`}><Icon name="more" size={20} /></button></div>
