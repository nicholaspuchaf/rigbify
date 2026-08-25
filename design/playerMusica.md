# Player em tela cheia

Referência visual: `playerMusica.jpeg`

## Objetivo

Concentrar capa, informações e controles da reprodução atual, mantendo as ações principais confortáveis para uso com uma mão.

## Direção visual

- Tela imersiva e escura.
- Fundo derivado das cores da capa, com gradiente escuro e desfoque intenso.
- Capa quadrada como principal elemento visual; não usar vídeo no protótipo.
- Branco para controles primários, cinza para indisponíveis e verde-lima para estados ativos.
- Controles grandes, espaçados e com alto contraste.

## Estrutura

### Barra superior

- Botão para recolher o player à esquerda.
- Contexto central em duas linhas: `Tocando da playlist` e nome da playlist/fila.
- Menu de mais opções à direita.
- Respeitar a área segura superior.

### Área da capa

- Capa quadrada centralizada, usando quase toda a largura útil.
- Cantos entre 12 e 20 px e sombra suave.
- Se não existir capa, gerar composição com gradiente e ícone musical.
- Em telas baixas, reduzir a capa antes de reduzir controles.

### Informações

- Título em destaque, uma ou duas linhas.
- Artista e álbum em texto secundário.
- Botão de favorito à direita; ativo em verde-lima.
- Tocar no artista ou álbum navega para o respectivo detalhe futuramente.

### Progresso

- Slider horizontal com área de toque maior que a linha visível.
- Tempo atual à esquerda e duração à direita.
- Enquanto arrasta, mostrar prévia do tempo e só confirmar a posição ao soltar quando necessário.
- Progresso deve acompanhar eventos do player nativo, não um temporizador independente da interface.

### Controles principais

- Linha com embaralhar, anterior, play/pausa, próxima e repetir.
- Play/pausa é circular, maior e central.
- Embaralhar e repetir ativos usam verde-lima e indicador adicional para não depender apenas de cor.
- Repetição alterna entre desligada, fila e uma música.
- Botões anterior/próxima ficam visualmente desabilitados quando não houver ação possível.

### Controles secundários

- Saída de áudio/dispositivo.
- Compartilhar somente metadados, nunca o arquivo sem confirmação explícita.
- Abrir fila.
- Temporizador para parar reprodução, opcional.

### Fila ou detalhes

- Card inferior ou bottom sheet com próxima música e acesso à fila.
- A folha pode ser arrastada para cima sem desmontar o player.

## Estados do player

- `idle`: nenhum conteúdo; não abrir esta tela.
- `loading`: play substituído por indicador de carregamento.
- `playing`: botão mostra pausa.
- `paused`: botão mostra play.
- `buffering`: manter estado atual e mostrar progresso de carregamento.
- `error`: explicar o problema e oferecer tentar novamente ou remover arquivo ausente.

O player nativo é a fonte de verdade. A tela envia intenções e reage aos eventos retornados por ele.

## Componentes Svelte sugeridos

- `NowPlayingHeader.svelte`
- `AlbumArtwork.svelte`
- `TrackMetadata.svelte`
- `PlaybackProgress.svelte`
- `PlaybackControls.svelte`
- `SecondaryControls.svelte`
- `QueueSheet.svelte`
- `PlayerBackground.svelte`

## Integração nativa esperada

- Ações: play, pause, seek, next, previous, shuffle, repeat e stop.
- Eventos: estado, faixa, posição, duração, erro, fila e rota de áudio.
- O mesmo estado alimenta o player completo, miniplayer, notificação e tela bloqueada.
- Comandos devem ser idempotentes e tolerar mudança rápida de faixa.

## Acessibilidade

- Botões com labels que reflitam a ação atual, como `Pausar Monster`.
- Slider anuncia tempo atual e duração.
- Ordem de foco acompanha a hierarquia visual.
- Fundo derivado da capa deve sempre receber camada escura suficiente para contraste.

## Critérios de aceite do protótipo

- Layout se adapta a telas baixas e altas sem esconder controles.
- Play, pausa, anterior, próxima, favorito, embaralhar e repetir mudam visualmente.
- Slider pode ser manipulado por toque e teclado.
- Estados loading, paused, playing e error podem ser simulados.
