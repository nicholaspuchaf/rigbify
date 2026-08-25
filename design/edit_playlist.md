# Editar sequência da playlist

Referência visual: `edit_playlist.jpeg`

## Objetivo

Permitir reorganizar e remover músicas da playlist com segurança, oferecendo alterações em lote antes de salvar.

## Direção visual

- Fundo preto e cabeçalho escuro fixo.
- Lista ocupa toda a área disponível.
- Cada linha possui remoção à esquerda, capa e metadados no centro e alça de arraste à direita.
- Ações neutras em cinza; confirmação em verde-lima; remoção em vermelho suave quando ativa.
- Barra inferior fixa com ação para selecionar múltiplas músicas.

## Cabeçalho

- Botão `Voltar` à esquerda.
- Título central `Editar playlist`.
- Botão `Salvar` à direita.
- `Salvar` fica desabilitado até existir alteração válida.
- Ao voltar com mudanças pendentes, perguntar se deseja descartar.

## Linha editável

Cada item contém:

- Botão circular de menos, com label `Remover [título]`.
- Capa quadrada.
- Título e artista; álbum pode aparecer em telas maiores.
- Indicador de indisponibilidade.
- Alça de arraste com área de toque mínima de 44 px.

Tocar na capa não deve iniciar áudio durante o modo de edição, evitando conflito com arraste.

## Reordenação

- Pressionar e arrastar pela alça move a música.
- A lista acompanha o dedo e mantém auto-scroll nas bordas.
- Animação curta mostra a nova posição.
- A alteração permanece local até `Salvar`.
- Salvar grava todas as posições em uma única transação.
- Posições devem ser normalizadas para uma sequência sem lacunas.
- Reordenar define o critério da playlist como `Ordem personalizada`.

## Remoção

- Tocar no menos marca a música para remoção e permite desfazer.
- Remoção só é persistida ao salvar.
- Texto deve deixar claro: remove da playlist, não da biblioteca nem do celular.
- Seleção múltipla permite marcar várias e usar `Remover N músicas`.
- Não abrir confirmação individual repetitiva; uma confirmação em lote é suficiente.

## Inclusão durante edição

- Botão inferior `Selecionar` ou `Adicionar músicas` abre a tela de adição.
- Ao retornar, novas músicas aparecem no final e são parte das mudanças pendentes.
- Duplicatas continuam proibidas.

## Concorrência e reprodução

- Se a playlist estiver tocando, salvar reconstrói a fila preservando a faixa e sua posição quando ela continuar presente.
- Se a faixa atual for removida, o player segue para a próxima disponível.
- Alterações externas detectadas durante a edição devem gerar conflito explícito, não sobrescrever silenciosamente.
- Usar `updated_at` ou versão da playlist para controle otimista.

## Estados

- Sem alterações: salvar desabilitado.
- Salvando: bloquear nova edição e mostrar progresso discreto.
- Erro: preservar mudanças locais e oferecer tentar novamente.
- Lista vazia após remoções: permitir salvar e mostrar como ficará o estado vazio.
- Arquivo ausente: permitir remover e reordenar normalmente.

## Persistência esperada

- Operação de atualização recebe a sequência completa de IDs e a versão esperada.
- Validar que todas as músicas pertencem à playlist e não há IDs repetidos.
- Executar reordenação e remoções na mesma transação SQLite.
- Atualizar `playlists.updated_at` apenas após sucesso.
- Testar rollback em erro no meio da operação.

## Componentes Svelte sugeridos

- `EditPlaylistHeader.svelte`
- `SortableTrackList.svelte`
- `SortableTrackRow.svelte`
- `MultiSelectBar.svelte`
- `UnsavedChangesDialog.svelte`

## Acessibilidade

- Além de arrastar, oferecer `Mover para cima` e `Mover para baixo` no menu ou teclado.
- Anunciar nova posição após mover.
- Foco não pode desaparecer quando uma linha é removida.
- Botões de menos e alças precisam de nomes acessíveis específicos.

## Critérios de aceite

- Reordenação e remoções persistem atomicamente.
- Voltar protege mudanças não salvas.
- Remover da playlist nunca apaga a música ou arquivo.
- Fila ativa é atualizada sem reiniciar a faixa mantida.
- Existe alternativa acessível ao gesto de arrastar.
