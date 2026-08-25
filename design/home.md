# Tela inicial

Referência visual: `home.jpeg`

## Objetivo

Ser o ponto de entrada do Rigbyfy, oferecendo acesso imediato às músicas recentes, atalhos da biblioteca e sugestões baseadas exclusivamente no conteúdo local do aparelho.

## Direção visual

- Fundo quase preto (`#0B0B0D`) em toda a tela.
- Superfícies elevadas em cinza-escuro (`#1A1A1E` e `#242428`).
- Verde-lima da marca (`#D7FF3F`) como destaque para ações ativas e reprodução.
- Texto principal branco suave (`#F5F5F3`) e secundário cinza (`#929298`).
- Cantos arredondados entre 10 e 16 px.
- Interface densa, mas com separação vertical clara entre seções.

## Estrutura

### Cabeçalho

- Respeitar a área segura superior do aparelho.
- Avatar circular à esquerda, abrindo Configurações.
- Filtros em chips ao lado ou abaixo: `Tudo`, `Músicas` e `Playlists`.
- Chip selecionado usa fundo verde-lima e texto escuro.
- Cabeçalho pode permanecer fixo ou ganhar fundo sólido durante a rolagem.

### Atalhos rápidos

- Grade de duas colunas com até oito itens usados recentemente.
- Cada item contém uma miniatura quadrada à esquerda e nome à direita.
- Tipos possíveis: músicas favoritas, adicionadas recentemente, playlist e álbum.
- Pressionar o item abre seu detalhe; pressionar e segurar abre ações contextuais.
- Em telas muito estreitas, preservar duas colunas e reduzir tipografia/espaçamento.

### Seção “Feito para você”

- Carrossel horizontal de capas grandes.
- O conteúdo deve vir da biblioteca local: mais ouvidas, esquecidas, adicionadas recentemente e mistura aleatória.
- Cada cartão contém capa, título e descrição curta.
- A rolagem horizontal deve indicar parcialmente o próximo cartão.

### Destaque

- Card horizontal amplo para retomar a fila ou continuar uma música interrompida.
- Capa ou composição visual ocupando a maior parte do cartão.
- Exibir título, artista, progresso e botão circular de reprodução.
- Não usar vídeo ou publicidade no protótipo.

### Recentes

- Lista ou carrossel horizontal com capas quadradas.
- Cabeçalho com título `Adicionadas recentemente` e ação `Ver tudo`.
- Mostrar título e artista, limitados a duas linhas no total.

### Player compacto e navegação

- O miniplayer fica fixo imediatamente acima da navegação inferior.
- A navegação inferior possui `Início`, `Buscar`, `Biblioteca` e `Configurações`.
- O item ativo usa verde-lima; os demais usam cinza.

## Estados

- Biblioteca vazia: ilustração simples, mensagem de boas-vindas e botão `Adicionar músicas`.
- Carregando: skeletons discretos nas grades e capas.
- Erro ao ler biblioteca: mensagem curta com ação `Tentar novamente`.
- Sem capa: arte gerada usando gradiente e ícone musical.

## Componentes Svelte sugeridos

- `AppHeader.svelte`
- `FilterChips.svelte`
- `QuickAccessGrid.svelte`
- `MusicCarousel.svelte`
- `ContinueListeningCard.svelte`
- `TrackCard.svelte`
- `MiniPlayer.svelte`
- `BottomNavigation.svelte`

## Comportamento responsivo

- Mobile: conteúdo em uma coluna com grades e carrosséis horizontais.
- Tablet: ampliar atalhos para três ou quatro colunas.
- Desktop: limitar conteúdo central e transformar a navegação inferior em barra lateral futuramente.

## Acessibilidade

- Chips devem expor estado selecionado.
- Capas decorativas devem ter texto alternativo vazio; capas informativas usam título e artista.
- Todos os cartões e botões precisam funcionar por teclado.
- Alvos de toque com no mínimo 44 × 44 px.

## Critérios de aceite do protótipo

- Cabeçalho, chips, atalhos, carrossel, recentes, miniplayer e navegação visíveis.
- Conteúdo rolável sem ficar escondido atrás das barras fixas.
- Tela vazia disponível para teste.
- Nenhuma dependência direta de SQLite ou do player nativo dentro da tela.
