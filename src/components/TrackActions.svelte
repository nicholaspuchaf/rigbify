<script lang="ts">
  import type { Collection, Track } from '../types/music';
  export let track: Track;
  export let collections: Collection[];
  export let onclose: () => void;
  export let onfavorite: (track: Track) => Promise<void> | void;
  export let onremove: (track: Track) => Promise<void> | void;
  export let onedit: (track: Track, values: { title: string; artist: string; album: string }) => Promise<void> | void;
  export let onplaylist: (track: Track, playlist: Collection) => Promise<void> | void;
  let mode: 'menu' | 'edit' | 'playlist' | 'remove' = 'menu';
  let title = track.title; let artist = track.artist; let album = track.album; let busy = false; let error = '';
  async function run(action: () => Promise<void> | void) { busy = true; error = ''; try { await action(); onclose(); } catch (reason) { error = reason instanceof Error ? reason.message : 'Não foi possível concluir a ação.'; } finally { busy = false; } }
</script>

<div class="modal-backdrop" role="presentation" onclick={(event) => { if (event.target === event.currentTarget) onclose(); }}>
  <div class="action-sheet" role="dialog" aria-modal="true" aria-labelledby="track-actions-title">
    <header><div><small>OPÇÕES DA MÚSICA</small><h2 id="track-actions-title">{track.title}</h2></div><button onclick={onclose} aria-label="Fechar">×</button></header>
    {#if mode === 'menu'}
      <button class="sheet-action" onclick={() => mode = 'edit'}><span>✎</span><div><b>Editar metadados</b><small>Título, artista e álbum no Rigbyfy</small></div></button>
      <button class="sheet-action" onclick={() => mode = 'playlist'}><span>＋</span><div><b>Adicionar a uma playlist</b><small>Escolha uma coleção</small></div></button>
      <button class="sheet-action" onclick={() => run(() => onfavorite(track))}><span>♥</span><div><b>{track.favorite ? 'Remover dos favoritos' : 'Adicionar aos favoritos'}</b><small>Atualiza sua biblioteca</small></div></button>
      <button class="sheet-action danger" onclick={() => mode = 'remove'}><span>×</span><div><b>Remover música</b><small>O arquivo original não será apagado</small></div></button>
    {:else if mode === 'edit'}
      <form onsubmit={(event) => { event.preventDefault(); void run(() => onedit(track, { title, artist, album })); }}>
        <label>Título<input bind:value={title} required maxlength="200" /></label>
        <label>Artista<input bind:value={artist} maxlength="200" /></label>
        <label>Álbum<input bind:value={album} maxlength="200" /></label>
        <div class="sheet-buttons"><button type="button" onclick={() => mode = 'menu'}>Voltar</button><button class="primary" disabled={busy}>Salvar</button></div>
      </form>
    {:else if mode === 'playlist'}
      <div class="playlist-picker">{#each collections.filter((item) => item.type === 'playlist') as playlist (playlist.id)}<button disabled={busy} onclick={() => run(() => onplaylist(track, playlist))}><span style={`--h:${playlist.hue}`}>♫</span><div><b>{playlist.title}</b><small>{playlist.subtitle}</small></div></button>{/each}</div>
      <div class="sheet-buttons"><button onclick={() => mode = 'menu'}>Voltar</button></div>
    {:else}
      <p class="confirm-copy">Remover <b>{track.title}</b> da biblioteca? O arquivo de áudio continuará no celular.</p>
      <div class="sheet-buttons"><button onclick={() => mode = 'menu'}>Cancelar</button><button class="danger-button" disabled={busy} onclick={() => run(() => onremove(track))}>Remover</button></div>
    {/if}
    {#if error}<p class="sheet-error" role="alert">{error}</p>{/if}
  </div>
</div>
