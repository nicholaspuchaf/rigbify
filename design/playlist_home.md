# Detalhe da playlist

Referência visual: `playlist_home.jpeg`

## Objetivo

Apresentar uma playlist, seus metadados e músicas, oferecendo reprodução imediata e acesso às ações de manutenção. Esta é a tela central do domínio de playlists.

## Direção visual

- Fundo quase preto na área da lista.
- Cabeçalho com gradiente escuro derivado da capa da playlist.
- Verde-lima do Rigbyfy substitui o verde da referência nos botões ativos.
- Capa quadrada grande, centralizada, com cantos discretamente arredondados.
- Texto principal branco e informações secundárias em cinza.
- Miniplayer e navegação permanecem fixos na base, sem cobrir músicas.

## Cabeçalho e capa

- Botão `Voltar` no canto superior esquerdo, respeitando a área segura.
- Capa principal pode ser:
  - Imagem escolhida pelo usuário.
  - Mosaico das capas das primeiras quatro músicas.
  - Gradiente determinístico quando não houver capas.
- O mosaico deve ser regenerado quando a composição da playlist mudar, mas uma capa personalizada sempre tem prioridade.
- Em rolagem, o cabeçalho pode reduzir para uma barra compacta com nome e voltar.

## Informações da playlist

- Nome em destaque, como `All good`.
- Descrição opcional abaixo do nome.
- Autor ou indicação `Playlist local`.
- Quantidade de músicas e duração total calculada.
- Indicador de disponibilidade quando existirem arquivos ausentes.
- Nome, descrição e capa podem ser editados sem alterar músicas.

## Ações principais

- `Adicionar`: abre a tela para buscar e incluir músicas.
- `Editar`: abre a edição manual da sequência e remoção de itens.
- `Classificar`: abre a folha de critérios de ordenação.
- `Nome e detalhes`: edita nome, descrição e capa.
- Menu de mais opções:
  - Renomear.
  - Duplicar playlist.
  - Limpar músicas.
  - Excluir playlist.
- Download não será exibido enquanto não existir gerenciamento real de arquivos offline.
- Compartilhamento futuro deve compartilhar uma lista de metadados, não arquivos, sem confirmação explícita.

## Reprodução

- Botão circular principal toca a playlist desde o início ou continua sua última posição.
- Botão de embaralhar ativa/desativa shuffle e inicia reprodução quando necessário.
- Ao tocar uma música, a fila nativa recebe as músicas da playlist naquela ordem.
- Próxima e anterior respeitam a ordem efetiva, incluindo shuffle e repetição.
- Arquivos ausentes devem ser ignorados com aviso, sem interromper toda a fila.
- O player nativo continua sendo a fonte de verdade.

## Lista de músicas

Cada linha contém:

- Capa quadrada ou fallback.
- Título.
- Artista e, quando útil, álbum.
- Indicador da faixa que está tocando.
- Menu de três pontos.

O menu da música oferece:

- Tocar agora.
- Tocar a seguir.
- Adicionar à fila.
- Remover desta playlist.
- Editar metadados.
- Adicionar a outra playlist.
- Alternar favorito.
- Remover da biblioteca, com confirmação separada.

Remover da playlist nunca remove a música da biblioteca. Remover da biblioteca deve explicar que o arquivo original permanece no celular.

## Estados

- Playlist vazia: capa fallback, mensagem e botão `Adicionar músicas`.
- Carregando: skeleton de cabeçalho e linhas.
- Música ausente: linha atenuada, ícone de alerta e ação para localizar/remover.
- Playlist inexistente: mensagem com retorno à biblioteca.
- Erro de persistência: manter alterações locais pendentes somente até informar o usuário e oferecer nova tentativa.

## Modelo e persistência esperados

- `playlists`: id, nome, descrição, capa, regra de ordenação e timestamps.
- `playlist_tracks`: playlist, música, posição manual e data de inclusão.
- A duração total é derivada das músicas, não armazenada como fonte de verdade.
- A mesma música aparece no máximo uma vez por playlist na primeira versão.
- Exclusão da playlist remove apenas associações, nunca músicas ou arquivos.

## Componentes Svelte sugeridos

- `PlaylistHeader.svelte`
- `PlaylistArtwork.svelte`
- `PlaylistMetadata.svelte`
- `PlaylistActions.svelte`
- `PlaylistTrackList.svelte`
- `TrackRow.svelte`
- `TrackActions.svelte`
- `MiniPlayer.svelte`

## Acessibilidade

- Nome acessível em todos os botões de ícone.
- Duração total deve ser texto, não somente ícone.
- Indicar reprodução e shuffle além da cor.
- Alvos de toque com no mínimo 44 × 44 px.
- Menu de ações deve reter foco e devolvê-lo ao botão que o abriu.

## Critérios de aceite

- Exibe capa, nome, autor, quantidade, duração e músicas reais.
- Tocar e embaralhar criam a fila correta no Media3.
- Adicionar, editar, ordenar, remover e excluir persistem após reiniciar o app.
- Playlist vazia e arquivos ausentes possuem tratamento explícito.

