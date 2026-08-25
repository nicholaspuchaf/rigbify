<script lang="ts">import type { Collection } from '../types/music'; import Artwork from './Artwork.svelte'; export let item: Collection; export let onopen: (item: Collection) => void = () => {};</script>
<button class="collection-card" onclick={() => onopen(item)}><Artwork hue={item.hue} label={`Capa de ${item.title}`} /><span><b>{item.title}</b><small>{item.subtitle}</small></span></button>
