<script lang="ts">
  import type { Track } from '../types/music'; import { calculatePlaybackProgress } from '../services/playerService'; import Artwork from './Artwork.svelte'; import Icon from './Icon.svelte';
  export let track: Track; export let playing: boolean; export let position = 0; export let duration = track.duration; export let onopen: () => void; export let ontoggle: () => void;
  $: progress = calculatePlaybackProgress(position, duration);
</script>
<div class="mini-player"><button class="mini-main" onclick={onopen}><Artwork hue={track.hue} src={track.artworkUri} label="" /><span><b>{track.title}</b><small>{track.artist}</small></span></button><button class="mini-device" aria-label="Escolher dispositivo"><Icon name="device" size={21} /></button><button class="mini-toggle" onclick={ontoggle} aria-label={playing ? 'Pausar' : 'Reproduzir'}><Icon name={playing ? 'pause' : 'play'} size={21} /></button><i class="mini-progress" style={`--mini-progress:${progress}%`} role="progressbar" aria-label="Progresso da música" aria-valuemin="0" aria-valuemax="100" aria-valuenow={Math.round(progress)}></i></div>
