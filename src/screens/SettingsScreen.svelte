<script lang="ts">
  import Icon from '../components/Icon.svelte'; import SettingRow from '../components/SettingRow.svelte';
  export let onback: () => void; let query = ''; let autoplay = true; let lockCover = true; let reduceMotion = false;
  const matches = (value: string) => value.toLocaleLowerCase('pt-BR').includes(query.toLocaleLowerCase('pt-BR'));
</script>
<div class="screen settings-screen"><header class="settings-header"><button onclick={onback} aria-label="Voltar"><Icon name="back" /></button><h1>Configurações</h1><span></span></header>
  <label class="settings-search"><Icon name="search" size={19} /><span class="sr-only">Buscar configurações</span><input bind:value={query} placeholder="Buscar configurações" /></label>
  <section class="library-summary"><div><span class="eyebrow">BIBLIOTECA LOCAL</span><h2>6 músicas no aparelho</h2><p>24 MB · Atualizada agora</p></div><button>Atualizar</button></section>
  {#if matches('biblioteca pastas metadados')}<section class="settings-group"><h2>Biblioteca</h2><SettingRow icon="library" title="Pastas monitoradas" description="Escolha onde procurar músicas" /><SettingRow icon="music" title="Metadados e capas" description="Preferências de leitura e imagens" /></section>{/if}
  {#if matches('reprodução autoplay retomar')}<section class="settings-group"><h2>Reprodução</h2><SettingRow icon="play" title="Autoplay da fila" description="Continue ouvindo músicas relacionadas" enabled={autoplay} ontoggle={() => autoplay = !autoplay} /><SettingRow icon="play" title="Retomar última música" description="Continue de onde você parou" /></section>{/if}
  {#if matches('notificações tela bloqueada capa')}<section class="settings-group"><h2>Notificações e tela bloqueada</h2><SettingRow icon="device" title="Exibir capa" description="Mostrar a arte na tela bloqueada" enabled={lockCover} ontoggle={() => lockCover = !lockCover} /><SettingRow icon="device" title="Controles multimídia" description="Gerenciar permissões do sistema" /></section>{/if}
  {#if matches('aparência animações tema')}<section class="settings-group"><h2>Aparência</h2><SettingRow icon="settings" title="Tema" description="Escuro" /><SettingRow icon="settings" title="Reduzir animações" description="Use transições mais discretas" enabled={reduceMotion} ontoggle={() => reduceMotion = !reduceMotion} /></section>{/if}
  {#if matches('armazenamento privacidade sobre')}<section class="settings-group"><h2>Armazenamento e privacidade</h2><SettingRow icon="library" title="Uso de armazenamento" description="24 MB em músicas e capas" /><SettingRow icon="settings" title="Privacidade" description="Seus dados permanecem neste dispositivo" /><SettingRow icon="settings" title="Sobre o Rigbyfy" description="Versão 0.1.0" /></section>{/if}
</div>
