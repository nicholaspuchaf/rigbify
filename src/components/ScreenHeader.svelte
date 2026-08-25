<script lang="ts">
  import Avatar from './Avatar.svelte'; import Icon from './Icon.svelte';
  export let title: string; export let onsettings: () => void; export let action: 'search' | 'plus' | null = null; export let onaction: (() => void) | undefined = undefined;
</script>
<header class="screen-header"><div class="title-row"><Avatar onclick={onsettings} /><h1>{title}</h1><span class="header-spacer"></span>{#if action}<button class="icon-button" onclick={onaction} aria-label={action === 'search' ? 'Buscar' : 'Adicionar'}><Icon name={action} size={28} /></button>{/if}</div></header>
