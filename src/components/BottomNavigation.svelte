<script lang="ts">import type { View } from '../types/music'; import Icon from './Icon.svelte'; export let active: View; export let navigate: (view: View) => void;
  const items = [{id:'home',label:'Início',icon:'home'},{id:'search',label:'Buscar',icon:'search'},{id:'library',label:'Biblioteca',icon:'library'},{id:'settings',label:'Configurações',icon:'settings'}] as const;
</script>
<nav class="bottom-nav" aria-label="Navegação principal">{#each items as item (item.id)}<button class:active={active === item.id} onclick={() => navigate(item.id)} aria-current={active === item.id ? 'page' : undefined}><Icon name={item.icon} size={22} /><span>{item.label}</span></button>{/each}</nav>
