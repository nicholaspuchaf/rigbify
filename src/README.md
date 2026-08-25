# Pasta `src`: frontend Svelte

## Escopo

Esta pasta implementa apresentação, navegação, interação e adaptação da interface para a ponte Tauri. Ela pode coordenar casos de uso do ponto de vista da UI, mas não deve conter SQL, detalhes de Kotlin/Media3 nem regras de domínio que precisem permanecer corretas fora da WebView.

## Tecnologias e dependências

- Svelte 5 para componentes e reatividade.
- TypeScript estrito para contratos e segurança de tipos.
- APIs `@tauri-apps/api` para detectar o runtime e executar `invoke`.
- Vitest para testes unitários de serviços e mapeadores.
- Vite para desenvolvimento, HMR e bundle.
- CSS global próprio; não há framework externo de componentes ou estilos.

Dependências descendentes: `services` conhece a ponte Tauri e os mocks; screens dependem de services, types e components; components devem depender apenas de tipos e props sempre que possível.

```text
screens ─────► services ─────► @tauri-apps/api ─────► Rust/plugins
   │             │
   ├────────────► types
   └────────────► components ─► types
```

## Estrutura e responsabilidades

### Arquivos raiz

- `main.ts`: ponto de entrada. Monta `App` no elemento raiz e registra `/sw.js` somente em produção.
- `App.svelte`: composição principal e coordenador da navegação. Mantém tela atual, faixa ativa, origem da fila, atualização da biblioteca e abertura dos fluxos de playlist/player.
- `style.css`: tokens visuais globais, reset, base tipográfica, cores, superfícies, largura mobile e utilitários compartilhados. Evite colocar aqui estilo exclusivo de uma única tela.
- `vite-env.d.ts`: tipos de ambiente fornecidos pelo Vite.

### `components/`

Componentes reutilizáveis e predominantemente apresentacionais:

- `Artwork.svelte`: representação de capa e fallback visual.
- `Avatar.svelte`: avatar circular usado no cabeçalho.
- `BottomNavigation.svelte`: navegação entre áreas principais.
- `ChipBar.svelte`: filtros/atalhos horizontais selecionáveis.
- `CollectionCard.svelte`: card de álbum, playlist ou coleção.
- `Icon.svelte`: catálogo SVG centralizado; novos ícones devem entrar aqui em vez de SVG duplicado.
- `MiniPlayer.svelte`: resumo persistente da faixa e controles essenciais.
- `ScreenHeader.svelte`: cabeçalho reutilizável, incluindo retorno e ações.
- `SectionHeading.svelte`: título e ação opcional de seção.
- `SettingRow.svelte`: linha consistente de configuração.
- `TrackActions.svelte`: menu compartilhado de três pontos para editar, adicionar a playlist, favoritar e remover.
- `TrackRow.svelte`: linha de música reutilizada em biblioteca, busca e playlists.

Regra: componentes recebem dados por props e comunicam intenções por callbacks. Não devem chamar SQLite ou plugins diretamente. Se uma variação servir somente a uma tela, prefira slot/prop estreita a duplicar o componente inteiro.

### `screens/`

Telas coordenam componentes e casos de uso:

- `HomeScreen.svelte`: saudação, filtros, coleções, recomendações e acesso à reprodução.
- `SearchScreen.svelte`: termo de busca, resultados e descoberta visual.
- `LibraryScreen.svelte`: importação, filtros da biblioteca, ações de faixa e entrada para playlists.
- `PlaylistScreen.svelte`: detalhe, ações administrativas, ordenação, reordenação, remoção e reprodução da playlist.
- `PlaylistAddScreen.svelte`: busca, filtros e inclusão imediata de músicas, com progresso, erro contextual e conclusão do fluxo.
- `PlaylistEditScreen.svelte`: nome, descrição e referência de capa.
- `PlayerScreen.svelte`: faixa ativa, progresso, controles nativos, fila e menu compartilhado.
- `SettingsScreen.svelte`: grupos e preferências visuais atuais.

Screens podem manter estado efêmero de formulário e carregamento. Estado persistente deve vir dos services/backend. Toda operação assíncrona deve representar carregamento, erro e atualização posterior de forma consistente.

### `services/`

Fronteira de efeitos e contratos do frontend:

- `libraryService.ts`: lista/importa músicas, altera favorito/metadados, remove do catálogo e oferece fallback web.
- `playerService.ts`: converte músicas em itens da fila e expõe play, pause, seek, anterior, próxima, stop e leitura do estado; no navegador usa estado mockado.
- `playlistService.ts`: CRUD, detalhes, adição/remoção, ordenação e reordenação de playlists, incluindo fallback em memória no navegador.
- `trackMapper.ts`: função pura que converte registros da biblioteca para o modelo visual `Track`.
- `*.test.ts`: testes unitários adjacentes aos respectivos serviços/mapeadores.

Somente services devem conhecer nomes de comandos Tauri e diferenças entre navegador e aplicativo. Preserve os payloads em `camelCase` na interface e faça qualquer adaptação de contrato num único lugar.

### `types/`

- `music.ts`: modelos visuais comuns de faixa, coleção e tela.
- `library.ts`: registros persistidos/importados e pedidos de atualização do catálogo.
- `player.ts`: fila e estado serializado do player.
- `playlist.ts`: resumo, detalhe, ordenação e requests de playlist.

Tipos de domínio compartilhados não devem ser redefinidos dentro de componentes. Prefira uniões literais para estados/opções fechadas e campos opcionais somente quando a ausência tiver significado real.

### `data/`

- `mock.ts`: conteúdo demonstrativo usado por telas e fallbacks web. Não é fonte de verdade em Android e não deve conter dados pessoais ou arquivos de áudio reais.

## Fluxos principais

### Carregar e importar

`App/LibraryScreen` solicita `libraryService` → Tauri chama Rust → plugin Android abre seletor e extrai metadados → Rust persiste → resposta tipada é mapeada para `Track` → a UI recarrega a lista.

### Executar ação de faixa

`TrackRow` ou `PlayerScreen` abre `TrackActions` → screen identifica a intenção → service executa comando → backend altera SQLite → screen atualiza/recarrega o estado. O mesmo componente garante as mesmas ações em todos os contextos.

### Reproduzir

A screen entrega faixa e contexto ao `playerService` → o serviço monta uma fila serializável → plugin Android inicia Media3 → a UI consulta o estado nativo periodicamente. Nunca assuma que o estado local da tela é mais atual que o player nativo.

### Playlist

Screens usam somente `playlistService`. A versão retornada no detalhe acompanha mutações para detectar concorrência. Depois de adicionar, remover, reordenar ou editar, use a resposta oficial ou recarregue; não mantenha uma cópia divergente indefinidamente.

## Padrão de implementação

- Leia a especificação correspondente em [`../design/README.md`](../design/README.md).
- Reuse `Icon`, `TrackRow`, `TrackActions`, headers, cards e navegação antes de criar equivalentes.
- Mantenha funções e handlers pequenos; extraia transformação pura para módulo testável.
- Use HTML semântico, foco visível, `aria-label` quando necessário e alvo de toque confortável.
- Evite `any`, promises ignoradas, acesso direto ao DOM e estado global implícito.
- Não coloque detalhes de plataforma em componentes. Adicione uma operação tipada ao service.
- Preserve o fallback web ao criar uma API Tauri, ou apresente indisponibilidade explícita se simulação não for segura.
- Mensagens de erro ao usuário devem ser úteis; detalhes técnicos ficam no limite apropriado e não devem revelar caminhos pessoais.
- CSS de um componente permanece no próprio `.svelte`; tokens e fundamentos compartilhados ficam em `style.css`.
- Atualize tipos antes dos consumidores quando um contrato mudar e mantenha Rust/Kotlin sincronizados.

## Testes e validação

Adicione teste unitário para mapeamentos, seleção/ordenação, fallback e tratamento de respostas. Para comportamento visual crítico, introduza teste de componente quando a infraestrutura correspondente existir. Fluxos nativos exigem também validação do backend/plugin.

```bash
bun run lint
bun run check
bun run test
bun run build
# ou todos:
bun run validate
```

Uma mudança em `src/` não está concluída se quebrar o navegador, o runtime Tauri, acessibilidade básica ou a documentação de sua tela/contrato.
