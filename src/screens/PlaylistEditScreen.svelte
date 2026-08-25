<script lang="ts">
  import type { Track } from '../types/music'; import Icon from '../components/Icon.svelte'; import Artwork from '../components/Artwork.svelte';
  export let name:string; export let initialTracks:Track[]; export let onback:()=>void; export let onsave:(ids:string[])=>Promise<void>; export let onadd:()=>void;
  let items=[...initialTracks]; let busy=false; $: changed=items.map(i=>i.id).join(',')!==initialTracks.map(i=>i.id).join(',');
  function move(index:number, direction:number){const target=index+direction;if(target<0||target>=items.length)return;items=[...items];[items[index],items[target]]=[items[target],items[index]];}
</script>
<div class="screen playlist-edit-screen"><header class="edit-playlist-header"><button onclick={()=>{if(!changed||confirm('Descartar alterações?'))onback();}} aria-label="Voltar"><Icon name="back" /></button><div><h1>Editar playlist</h1><small>{name}</small></div><button disabled={!changed||busy} onclick={async()=>{busy=true;await onsave(items.map(i=>i.id));busy=false;}}>Salvar</button></header>
  <div>{#each items as track,index (track.id)}<div class="sortable-track"><button class="remove-from-playlist" onclick={()=>items=items.filter(i=>i.id!==track.id)} aria-label={`Remover ${track.title}`}>−</button><Artwork hue={track.hue} label="" /><span><b>{track.title}</b><small>{track.artist}</small></span><div><button onclick={()=>move(index,-1)} disabled={index===0} aria-label={`Mover ${track.title} para cima`}>↑</button><button onclick={()=>move(index,1)} disabled={index===items.length-1} aria-label={`Mover ${track.title} para baixo`}>↓</button></div></div>{/each}</div><button class="edit-add-button" onclick={onadd}>＋ Adicionar músicas</button>
</div>
