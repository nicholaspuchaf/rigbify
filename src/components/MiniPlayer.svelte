<script lang="ts">
  import type { Track } from '../types/music'; import Artwork from './Artwork.svelte'; import Icon from './Icon.svelte';
  export let track: Track; export let playing: boolean; export let onopen: () => void; export let ontoggle: () => void;
</script>
<div class="mini-player"><button class="mini-main" onclick={onopen}><Artwork hue={track.hue} label="" /><span><b>{track.title}</b><small>{track.artist}</small></span></button><button class="mini-device" aria-label="Escolher dispositivo"><Icon name="device" size={21} /></button><button class="mini-toggle" onclick={ontoggle} aria-label={playing ? 'Pausar' : 'Reproduzir'}><Icon name={playing ? 'pause' : 'play'} size={21} /></button><i class="mini-progress"></i></div>
