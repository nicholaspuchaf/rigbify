<script lang="ts">import Icon from './Icon.svelte'; export let hue = 75; export let large = false; export let label = '';</script>
<span class:large class="artwork" style={`--art-hue:${hue}`} role={label ? 'img' : undefined} aria-label={label || undefined} aria-hidden={label ? undefined : 'true'}><Icon name="music" size={large ? 64 : 22} /><i></i></span>
