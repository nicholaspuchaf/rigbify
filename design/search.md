# Tela de busca

Referência visual: `search.jpeg`

## Objetivo

Permitir localizar rapidamente músicas, artistas, álbuns e playlists armazenados no aparelho e oferecer atalhos de exploração da biblioteca.

## Direção visual

- Fundo quase preto e cabeçalho integrado ao conteúdo.
- Campo de busca claro ou cinza muito claro para produzir contraste imediato.
- Cards de categorias coloridos, mantendo a paleta geral escura.
- Verde-lima reservado ao foco, seleção e ações principais.

## Estrutura

### Cabeçalho

- Avatar circular à esquerda.
- Título grande `Buscar`.
- Ação opcional à direita para filtros avançados; não usar câmera no Rigbyfy.

### Campo de busca

- Campo largo, com ícone de lupa, placeholder `O que você quer ouvir?` e botão para limpar.
- Fica abaixo do título e pode permanecer fixo durante a rolagem.
- Ao receber foco, abre teclado e troca o conteúdo de exploração por resultados.
- Aplicar debounce curto somente se a busca exigir acesso nativo; busca em memória pode ser imediata.

### Exploração inicial

- Seção `Explore sua biblioteca`.
- Carrossel horizontal opcional com coleções automáticas, como mais ouvidas, adicionadas hoje e sem capa.
- Grade de duas colunas para categorias: `Músicas`, `Artistas`, `Álbuns`, `Playlists`, `Pastas` e `Formatos`.
- Cards usam cores profundas e uma imagem/capa inclinada parcialmente para fora do limite.
- As cores devem ser variadas, mas com contraste adequado para texto branco.

### Resultados

- Agrupar por `Músicas`, `Artistas`, `Álbuns` e `Playlists`.
- Mostrar no máximo alguns resultados por grupo, com `Ver tudo`.
- Música: capa, título, artista e menu contextual.
- Artista: imagem circular ou monograma.
- Álbum/playlist: capa quadrada.
- Destacar apenas correspondências quando isso não prejudicar a leitura.

### Player e navegação

- Manter miniplayer e navegação inferior fixos.
- Quando o teclado estiver aberto, a navegação pode desaparecer, mas o resultado não deve ficar obstruído.

## Estados

- Inicial: categorias e coleções locais.
- Digitando: resultados atualizados e botão de limpar.
- Sem resultado: `Nenhuma música encontrada` e sugestões para revisar a busca.
- Biblioteca vazia: botão `Adicionar músicas`.
- Permissão indisponível: explicar como liberar acesso aos arquivos.

## Componentes Svelte sugeridos

- `SearchHeader.svelte`
- `SearchField.svelte`
- `DiscoveryCarousel.svelte`
- `CategoryGrid.svelte`
- `CategoryCard.svelte`
- `SearchResults.svelte`
- `TrackRow.svelte`
- `EmptyState.svelte`

## Regras de busca

- Ignorar diferenças entre maiúsculas/minúsculas e acentos.
- Buscar por título, artista, álbum, playlist e nome do arquivo.
- Não realizar requisições externas.
- Não salvar consultas contendo caminhos completos ou outros dados sensíveis em logs.

## Acessibilidade

- Campo com label programática, não apenas placeholder.
- Informar quantidade de resultados sem interromper a digitação.
- Ordem de foco: cabeçalho, busca, filtros e resultados.
- Cards coloridos precisam manter contraste mínimo AA.

## Critérios de aceite do protótipo

- Estados inicial, com resultados e sem resultados navegáveis.
- Busca filtra dados simulados de mais de uma categoria.
- Campo e botão de limpar funcionam por teclado.
- Grade se adapta sem cortes entre 320 e 430 px.
