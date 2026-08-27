<script lang="ts">
  import { afterUpdate } from 'svelte';
  import Icon from './Icon.svelte';
  import { resolveArtworkSource } from '../services/artworkService';

  export let hue = 75;
  export let src: string | undefined = undefined;
  export let large = false;
  export let label = '';

  let failedSource: string | undefined;
  let imageSource: string | undefined;
  let requestedSource: string | undefined;

  function resolveArtwork() {
    if (src === requestedSource) return;
    requestedSource = src;
    failedSource = undefined;
    imageSource = undefined;
    if (!src) return;

    const source = src;
    void resolveArtworkSource(source).then((resolved) => {
      if (requestedSource === source) imageSource = resolved;
    });
  }

  afterUpdate(resolveArtwork);
  $: showImage = Boolean(imageSource) && imageSource !== failedSource;
</script>

<span class:large class="artwork" style={`--art-hue:${hue}`} role={label ? 'img' : undefined} aria-label={label || undefined} aria-hidden={label ? undefined : 'true'}>
  {#if showImage}
    <img src={imageSource} alt="" onerror={() => failedSource = imageSource} />
  {:else}
    <Icon name="music" size={large ? 64 : 22} />
    <i></i>
  {/if}
</span>
