# Classificar músicas da playlist

Referência visual: `playlist_ordenar.jpeg`

## Objetivo

Escolher como as músicas serão apresentadas e reproduzidas na playlist, preservando uma ordem manual independente das classificações automáticas.

## Apresentação

- Bottom sheet sobre a tela de detalhe escurecida.
- Cantos superiores arredondados e indicador de arraste.
- Título `Classificar por`.
- Opções em linhas amplas com check na opção ativa.
- Verde-lima para seleção; branco e cinza para as demais.
- A folha fecha ao selecionar, arrastar para baixo, tocar fora ou pressionar voltar.

## Critérios

- `Ordem personalizada`: usa `playlist_tracks.position`.
- `Título`: ordem alfabética localizada.
- `Artista`: artista e depois título.
- `Álbum`: álbum, número do disco, número da faixa e título.
- `Adicionadas recentemente`: `playlist_tracks.added_at` decrescente.
- Pode haver alternância crescente/decrescente para critérios aplicáveis.

Valores ausentes devem ser enviados ao fim da lista e ordenados de forma determinística pelo título/id.

## Comportamento

- Selecionar um critério atualiza e persiste a preferência da playlist.
- A lista muda imediatamente sem reescrever posições manuais.
- Voltar para `Ordem personalizada` recupera exatamente a última sequência manual.
- O player deve usar a ordem atualmente exibida ao iniciar a playlist.
- Alterar classificação durante reprodução não deve trocar abruptamente a faixa atual; a fila seguinte é reconstruída preservando a faixa e posição.
- O critério não afeta outras playlists.

## Relação com edição manual

- Arrastar músicas na tela de edição força a playlist para `Ordem personalizada` após confirmação.
- Classificações automáticas não alteram o campo `position`.
- Novas músicas sempre recebem uma posição manual no final, mesmo quando a visualização está ordenada por outro critério.

## Estados e erros

- Critério ativo apresenta check e descrição acessível.
- Durante persistência, bloquear cliques repetidos brevemente.
- Em erro, restaurar a seleção anterior e manter a folha aberta com mensagem.
- Playlist vazia ainda pode salvar um critério para uso futuro.

## Modelo esperado

- `sort_mode`: `custom`, `title`, `artist`, `album` ou `recently_added`.
- `sort_direction`: `ascending` ou `descending`.
- Valores desconhecidos de versões futuras devem cair com segurança em `custom`.
- Ordenação de texto deve seguir uma estratégia consistente no Rust/SQLite e na interface.

## Componentes Svelte sugeridos

- `PlaylistSortSheet.svelte`
- `BottomSheet.svelte` reutilizável.
- `SelectableActionRow.svelte`.

## Acessibilidade

- Usar semântica de diálogo e radiogroup.
- Cada opção expõe `aria-checked`.
- Foco inicial na opção ativa.
- Ao fechar, devolver foco ao botão `Classificar`.
- Permitir fechamento por Escape em desktop.

## Critérios de aceite

- Todas as cinco opções produzem ordem determinística.
- Preferência persiste após reiniciar o aplicativo.
- Ordem manual nunca é perdida ao alternar critérios.
- Fila do player corresponde à ordem visível ao iniciar reprodução.

