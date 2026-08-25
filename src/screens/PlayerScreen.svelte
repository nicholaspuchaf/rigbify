<script lang="ts">
  import type { Track } from '../types/music'; import Icon from '../components/Icon.svelte'; import Artwork from '../components/Artwork.svelte';
  export let track: Track; export let playing: boolean; export let position = 0; export let onback: () => void; export let ontoggle: () => void; export let onnext: () => void; export let onprevious: () => void; export let onseek: (seconds: number) => void; export let onfavorite: (t: Track) => void;
  let shuffle = false; let repeat = false;
  const format = (seconds: number) => `${Math.floor(seconds / 60)}:${String(Math.floor(seconds % 60)).padStart(2,'0')}`;
</script>
<div class="player-screen" style={`--player-hue:${track.hue}`}><header><button onclick={onback} aria-label="Recolher player"><Icon name="back" size={28} /></button><div><small>TOCANDO AGORA</small><b>Fila local</b></div><button aria-label="Mais opções"><Icon name="more" /></button></header>
  <Artwork hue={track.hue} large label={`Capa de ${track.title}`} />
  <div class="player-info"><div><h1>{track.title}</h1><p>{track.artist} · {track.album}</p></div><button class:active={track.favorite} onclick={() => onfavorite(track)} aria-label="Favoritar"><Icon name="heart" size={26} /></button></div>
  <div class="timeline"><input type="range" min="0" max={track.duration} value={position} onchange={(event) => onseek(Number(event.currentTarget.value))} aria-label="Posição da música" /><div><span>{format(position)}</span><span>{format(track.duration)}</span></div></div>
  <div class="player-controls"><button class:active={shuffle} onclick={() => shuffle = !shuffle} aria-label="Embaralhar"><Icon name="shuffle" /></button><button onclick={onprevious} aria-label="Música anterior"><Icon name="previous" size={32} /></button><button class="play-main" onclick={ontoggle} aria-label={playing ? 'Pausar' : 'Reproduzir'}><Icon name={playing ? 'pause' : 'play'} size={34} /></button><button onclick={onnext} aria-label="Próxima música"><Icon name="next" size={32} /></button><button class:active={repeat} onclick={() => repeat = !repeat} aria-label="Repetir"><Icon name="repeat" /></button></div>
  <div class="player-secondary"><button><Icon name="device" /><span>Dispositivo</span></button><button><Icon name="queue" /><span>Fila</span></button></div>
  <button class="up-next"><span><small>A SEGUIR</small><b>Sol de Inverno · Aurora Sul</b></span><Icon name="chevron" /></button>
</div>
