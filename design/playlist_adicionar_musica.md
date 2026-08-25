# Adicionar músicas à playlist

Referência visual: `playlist_adicionar_musica.jpeg`

## Objetivo

Permitir encontrar músicas da biblioteca local e adicioná-las rapidamente à playlist aberta, individualmente ou em lote.

## Direção visual

- Tela escura em continuidade com a biblioteca.
- Cabeçalho simples e fixo.
- Chips horizontais abaixo do título.
- Lista densa com capa, metadados e botão circular de inclusão.
- Campo de busca fixo próximo à base ou no cabeçalho quando o teclado estiver aberto.
- Verde-lima indica seleção ou inclusão concluída.

## Cabeçalho

- Botão `Voltar` à esquerda.
- Título `Adicionar à playlist`.
- Opcionalmente mostrar o nome da playlist em uma segunda linha.
- A navegação para trás preserva inclusões já confirmadas.

## Filtros

Chips roláveis horizontalmente:

- `Sugeridas`: músicas ainda ausentes da playlist, priorizando artistas e álbuns relacionados ao conteúdo atual.
- `Músicas`: toda a biblioteca.
- `Favoritas`.
- `Adicionadas recentemente`.
- `Artistas` e `Álbuns`, se houver agrupamento disponível.

Não incluir episódios ou conteúdo remoto enquanto o produto for exclusivamente musical e local.

## Busca

- Campo com placeholder `O que você gostaria de adicionar?`.
- Busca por título, artista e álbum, sem diferenciar maiúsculas ou acentos quando possível.
- Debounce curto somente se a biblioteca for grande.
- Limpar busca deve restaurar o filtro atual.
- Teclado não pode esconder o item selecionado ou a ação de concluir.

## Lista de resultados

Cada linha contém:

- Capa ou fallback.
- Título.
- Artista e álbum como informação secundária.
- Indicador de arquivo indisponível quando aplicável.
- Botão `Adicionar` com ícone de mais.
- Após adicionar, o botão muda para check e fica desabilitado ou oferece desfazer.

Tocar na área principal pode reproduzir uma prévia usando a fila atual, sem adicionar automaticamente. O botão lateral é a única ação de inclusão para evitar acidentes.

## Inclusão individual e em lote

- Inclusão individual deve persistir imediatamente e atualizar a interface de forma otimista com rollback em erro.
- Pressionar `Selecionar várias` ativa checkboxes e uma barra `Adicionar N músicas`.
- Itens já existentes na playlist aparecem marcados.
- Duplicatas não devem ser criadas.
- A ordem inicial das novas músicas segue a sequência de seleção ou o final da playlist.

## Sugestões locais

Na primeira versão, sugestões podem considerar:

- Mesmo artista das músicas existentes.
- Mesmo álbum ou gênero.
- Favoritas ainda não adicionadas.
- Mais tocadas e adicionadas recentemente.

Não depender de internet nem enviar dados da biblioteca para serviços externos.

## Estados

- Biblioteca vazia: mensagem e ação para importar músicas.
- Nenhum resultado: informar termo pesquisado e oferecer limpar filtros.
- Todas já adicionadas: mensagem de conclusão.
- Arquivo ausente: permitir visualizar, mas impedir inclusão até resolver a disponibilidade.
- Erro ao adicionar: restaurar botão de mais e mostrar mensagem contextual.

## Componentes Svelte sugeridos

- `AddTracksHeader.svelte`
- `PlaylistSourceChips.svelte`
- `TrackSearchField.svelte`
- `AddTrackRow.svelte`
- `MultiSelectBar.svelte`
- Reutilizar `Artwork.svelte` e metadados do `TrackRow`.

## Persistência e comandos

- Adicionar uma ou várias associações em transação.
- Calcular posições sem colisão.
- Comando unitário idempotente para inclusão individual.
- Comando em lote retorna adicionadas, ignoradas e erros por música.
- Nenhuma operação copia ou move o arquivo de áudio.

## Acessibilidade

- Botão anuncia `Adicionar [título] à [playlist]` ou `Já adicionada`.
- Chips expõem estado selecionado.
- Seleção múltipla anuncia a quantidade.
- Resultados devem manter ordem de foco previsível.

## Critérios de aceite

- Busca e filtros operam sobre músicas reais.
- Inclusões persistem depois de reabrir a playlist.
- Duplicatas são impedidas.
- Inclusão múltipla é atômica ou informa falhas individualmente.

