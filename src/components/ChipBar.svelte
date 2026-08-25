<script lang="ts">export let items: string[]; export let selected: string; export let onselect: (item: string) => void;</script>
<div class="chip-bar" aria-label="Filtros">{#each items as item}<button class:active={selected === item} onclick={() => onselect(item)} aria-pressed={selected === item}>{item}</button>{/each}</div>
