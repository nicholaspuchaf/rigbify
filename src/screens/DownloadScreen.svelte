<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from '../components/Icon.svelte';
  import { downloadService } from '../services/downloadService';
  import type { DownloadMediaInfo, DownloadMediaKind, DownloadStatus, OutputFormat } from '../types/download';

  export let onback: () => void;
  export let onimported: () => Promise<void>;

  let url = '';
  let info: DownloadMediaInfo | null = null;
  let mediaKind: DownloadMediaKind = 'audio';
  let outputFormat: OutputFormat = 'original';
  let quality = 'best';
  let formatId = '';
  let status: DownloadStatus | null = null;
  let loading = false;
  let registering = false;
  let error = '';
  let authorized = false;
  let registered = false;

  $: visibleFormats = info?.formats.filter((format) => mediaKind === 'audio' ? format.audioOnly : !format.audioOnly) ?? [];
  $: availableOutputs = mediaKind === 'audio' ? ['original', 'mp3', 'm4a', 'opus'] as OutputFormat[] : ['original', 'mp4'] as OutputFormat[];

  onMount(() => {
    let listener: { unregister(): Promise<void> } | null = null;
    void downloadService.onProgress(async (next) => {
      if (status && next.downloadId !== status.downloadId) return;
      status = next;
      if (next.status === 'completed' && !registered && !registering) {
        registering = true;
        try { await downloadService.registerCompleted(next.downloadId); registered = true; await onimported(); }
        catch (reason) { error = message(reason); }
        finally { registering = false; }
      }
    }).then((value) => listener = value).catch((reason) => error = message(reason));
    return () => { if (listener) void listener.unregister(); };
  });

  async function inspect() {
    loading = true; error = ''; info = null; status = null; registered = false;
    try { info = await downloadService.getMediaInfo(url.trim()); }
    catch (reason) { error = message(reason); }
    finally { loading = false; }
  }

  async function start() {
    if (!info || !authorized) return;
    loading = true; error = ''; registered = false;
    try {
      status = await downloadService.start({
        source: { kind: 'single', url: info.url }, outputFormat, mediaKind,
        quality: quality || undefined, formatId: formatId || undefined,
        title: info.title, artist: info.artist ?? info.channel ?? undefined, album: info.album ?? undefined,
        thumbnail: info.thumbnail ?? undefined, durationMs: info.durationMs ?? undefined, sourceId: info.id
      });
    } catch (reason) { error = message(reason); }
    finally { loading = false; }
  }

  async function cancel() {
    if (!status) return;
    try { status = await downloadService.cancel(status.downloadId); }
    catch (reason) { error = message(reason); }
  }

  function message(reason: unknown) { return reason instanceof Error ? reason.message : String(reason); }
  function formatDuration(ms: number | null) { if (!ms) return 'Duração desconhecida'; const seconds = Math.round(ms / 1000); return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, '0')}`; }
  function statusLabel(value: DownloadStatus | null) {
    if (!value) return '';
    return ({ download_started: 'Preparando', download_progress: 'Baixando', post_processing: 'Processando com FFmpeg', completed: registered ? 'Adicionada à biblioteca' : 'Registrando na biblioteca', failed: 'Falhou', cancelled: 'Cancelado' } as const)[value.status];
  }
</script>

<div class="screen download-screen">
  <header class="subpage-header"><button onclick={onback} aria-label="Voltar"><Icon name="back" /></button><div><h1>Baixar mídia</h1><small>YouTube e YouTube Music</small></div></header>
  <form class="download-url" onsubmit={(event) => { event.preventDefault(); void inspect(); }}>
    <label for="download-url">URL do conteúdo</label>
    <div><input id="download-url" bind:value={url} inputmode="url" placeholder="https://youtube.com/watch?v=…" required /><button disabled={loading}>{loading && !info ? 'Consultando…' : 'Analisar'}</button></div>
  </form>

  {#if error}<p class="download-error" role="alert">{error}</p>{/if}

  {#if info}
    <article class="download-preview">
      {#if info.thumbnail}<img src={info.thumbnail} alt={`Thumbnail de ${info.title}`} />{:else}<div class="download-thumbnail"><Icon name="music" size={40} /></div>{/if}
      <div><h2>{info.title}</h2><p>{info.artist ?? info.channel ?? 'Canal desconhecido'}</p><small>{formatDuration(info.durationMs)} · {info.formats.length} formatos detectados</small></div>
    </article>

    <div class="download-options">
      <label>Tipo<select bind:value={mediaKind} onchange={() => { outputFormat = 'original'; formatId = ''; }}><option value="audio">Áudio</option><option value="video">Vídeo</option></select></label>
      <label>Saída<select bind:value={outputFormat}>{#each availableOutputs as output (output)}<option value={output}>{output === 'original' ? 'Original (sem converter)' : output.toUpperCase()}</option>{/each}</select></label>
      {#if mediaKind === 'video'}<label>Qualidade<select bind:value={quality}>{#each ['best','2160','1440','1080','720','480','360'] as value (value)}<option value={value}>{value === 'best' ? 'Melhor disponível' : `${value}p`}</option>{/each}</select></label>{/if}
      <label>Stream<select bind:value={formatId}><option value="">Automático recomendado</option>{#each visibleFormats as format (format.id)}<option value={format.id}>{format.id} · {format.container ?? '?'} · {format.resolution ?? `${Math.round(format.bitrate ?? 0)} kbps`}</option>{/each}</select></label>
    </div>

    <p class="conversion-note">{outputFormat === 'original' ? 'O stream original será mantido, sem perda causada por transcodificação.' : outputFormat === 'mp4' ? 'Vídeo e áudio podem ser baixados separadamente e unidos com FFmpeg.' : `O áudio será baixado e convertido para ${outputFormat.toUpperCase()} com FFmpeg.`}</p>
    <label class="rights-confirm"><input type="checkbox" bind:checked={authorized} /><span>Confirmo que tenho autorização para baixar e armazenar este conteúdo.</span></label>

    {#if !status || ['failed','cancelled'].includes(status.status)}<button class="download-primary" disabled={!authorized || loading} onclick={start}>Iniciar download</button>{/if}
  {/if}

  {#if status}
    <section class="download-status" aria-live="polite">
      <div><b>{statusLabel(status)}</b><span>{Math.round(status.progress)}%</span></div>
      <progress max="100" value={status.progress}></progress>
      <small>{status.message ?? ''}{status.eta !== null ? ` · ${status.eta}s restantes` : ''}</small>
      {#if ['download_started','download_progress','post_processing'].includes(status.status)}<button onclick={cancel}>Cancelar download</button>{/if}
    </section>
  {/if}
</div>
