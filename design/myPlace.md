# Tela da biblioteca

Referência visual: `myPlace.jpeg`

## Objetivo

Centralizar todo o conteúdo salvo no dispositivo e permitir filtrar, ordenar, criar playlists e acessar músicas rapidamente.

## Direção visual

- Fundo quase preto e conteúdo em lista vertical.
- Capas quadradas grandes, sem cartões externos desnecessários.
- Texto principal branco e metadados cinza.
- Verde-lima indica item em reprodução, conteúdo fixado e ações confirmadas.

## Estrutura

### Cabeçalho

- Avatar à esquerda e título `Sua biblioteca`.
- Botão de busca local e botão `Adicionar` à direita.
- O botão adicionar abre opções: importar músicas, criar playlist ou escolher pasta.
- Cabeçalho permanece compacto durante a rolagem.

### Filtros

- Chips horizontais: `Playlists`, `Músicas`, `Álbuns`, `Artistas` e `Pastas`.
- A lista deve permitir rolagem horizontal sem barra visível.
- Um filtro selecionado pode exibir botão de fechar para voltar a `Tudo`.

### Ordenação e visualização

- Linha com ordenação atual, inicialmente `Recentes`.
- Menu oferece: recentes, adicionadas recentemente, alfabética e mais tocadas.
- Botão à direita alterna entre lista e grade.

### Conteúdo

- Primeiros itens opcionais: `Músicas favoritas`, `Adicionadas recentemente` e playlists fixadas.
- Lista padrão com capa de aproximadamente 72 px, título e metadados.
- Playlist: quantidade de músicas.
- Álbum: artista e ano, quando disponível.
- Música: artista, álbum e duração.
- Item em reprodução usa título verde-lima e indicador animado discreto.
- Menu contextual oferece reproduzir em seguida, adicionar à playlist, editar metadados e localizar arquivo.

### Player e navegação

- Miniplayer fixo acima da navegação.
- `Biblioteca` fica destacado na navegação inferior.
- Reservar espaço inferior suficiente para o último item não ficar coberto.

## Estados

- Biblioteca vazia: instrução e botão `Escolher músicas`.
- Filtro vazio: mensagem específica e ação para limpar filtro.
- Arquivo ausente: item com alerta e ações para localizar ou remover referência.
- Atualizando biblioteca: indicador pequeno sem bloquear toda a tela.

## Componentes Svelte sugeridos

- `LibraryHeader.svelte`
- `LibraryFilters.svelte`
- `LibraryToolbar.svelte`
- `LibraryList.svelte`
- `LibraryGrid.svelte`
- `LibraryItem.svelte`
- `ContextMenu.svelte`
- `ImportMusicSheet.svelte`

## Regras de estado

- Filtro, ordenação e modo de visualização devem ser persistidos.
- A tela consome um serviço de biblioteca; não acessa SQLite diretamente.
- Listas grandes devem ser carregadas incrementalmente ou virtualizadas.
- Atualizações de favoritos ou metadados devem refletir sem recarregar toda a tela.

## Acessibilidade

- Informar semanticamente o filtro e a ordenação ativos.
- Menus contextuais precisam devolver o foco ao item de origem quando fechados.
- Títulos truncados devem continuar disponíveis como nome acessível.
- Ícones de ordenação e modo de exibição precisam de labels.

## Critérios de aceite do protótipo

- Filtros, ordenação e alternância lista/grade funcionam com dados simulados.
- Itens fixados e item em reprodução possuem estados visuais distintos.
- A tela suporta biblioteca vazia e arquivo ausente.
- O último item permanece acessível acima das barras fixas.
