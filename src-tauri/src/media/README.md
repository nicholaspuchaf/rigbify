# Núcleo de mídia do Rigbyfy

Esta pasta contém os contratos e regras relacionados à biblioteca e à reprodução de mídia. O código daqui não deve depender diretamente da interface Svelte nem assumir que toda música veio de um arquivo escolhido no Android.

## Objetivo inicial

Implementar um fluxo funcional completo:

```text
Selecionar música no Android
        ↓
Persistir acesso ao arquivo
        ↓
Extrair e normalizar metadados
        ↓
Salvar registro no SQLite e capa no cache
        ↓
Exibir na biblioteca Svelte
        ↓
Reproduzir com Media3/ExoPlayer
```

A primeira entrega deve permitir:

- Selecionar um ou vários arquivos de áudio.
- Manter acesso aos arquivos depois de reiniciar o aparelho.
- Ler título, artista, álbum, duração e capa incorporada.
- Salvar os metadados no SQLite.
- Salvar uma versão otimizada da capa no cache do aplicativo.
- Exibir músicas reais na biblioteca.
- Tocar, pausar e acompanhar o progresso.
- Restaurar biblioteca, fila e posição depois de fechar o aplicativo.

## Localização das responsabilidades

```text
src-tauri/src/
  media/
    README.md           este documento
    models.rs           entidades e tipos compartilhados
    source.rs           contratos das origens de mídia
    metadata.rs         extração e normalização de metadados
    library.rs          casos de uso da biblioteca
    player.rs           contrato independente de plataforma

  commands/
    media.rs            adaptadores entre Svelte e o núcleo

  database/
    migrations/         evolução incremental do SQLite
    repositories/       persistência das entidades de mídia

  platform/
    android/            ponte para o plugin Kotlin
    ios/                ponte futura para o plugin Swift

plugins/
  media-player/
    android/             Kotlin, Media3 e Storage Access Framework
    ios/                 Swift e AVFoundation no futuro
```

Essa é a estrutura desejada. As pastas e arquivos devem ser criados conforme cada responsabilidade começar a ser implementada, evitando arquivos vazios ou abstrações sem uso.

## Modelo da música

O registro persistido deve possuir, no mínimo:

| Campo | Tipo sugerido | Descrição |
| --- | --- | --- |
| `id` | UUID/string | Identificador interno estável |
| `source_type` | enum/string | Origem: local, importada ou download remoto |
| `source_uri` | string | URI Android, caminho privado ou identificador da origem |
| `title` | string | Nome da música normalizado |
| `artist` | string opcional | Artista principal |
| `album` | string opcional | Nome do álbum |
| `duration_ms` | inteiro | Duração em milissegundos |
| `artwork_uri` | string opcional | URI/caminho da capa ou banner otimizado |
| `mime_type` | string opcional | Tipo do arquivo, como `audio/mpeg` |
| `file_size` | inteiro opcional | Tamanho em bytes, quando conhecido |
| `track_number` | inteiro opcional | Número da faixa |
| `disc_number` | inteiro opcional | Número do disco |
| `year` | inteiro opcional | Ano de lançamento |
| `genre` | string opcional | Gênero informado nos metadados |
| `added_at` | timestamp | Data de inclusão na biblioteca |
| `updated_at` | timestamp | Última atualização dos metadados |
| `last_played_at` | timestamp opcional | Última reprodução |
| `play_count` | inteiro | Quantidade de reproduções concluídas |
| `favorite` | boolean | Estado de favorito |
| `availability` | enum/string | Disponível, ausente, baixando ou com erro |

`title`, `artist`, `album` e `artwork_uri` devem ser independentes da origem. A interface não deve precisar saber se esses dados vieram de uma tag ID3, do MediaStore, de um download ou de edição manual.

## Capa e banner da música

O termo técnico usado no código será `artwork`, abrangendo capa quadrada e imagem exibida no player. A interface pode apresentar essa imagem como capa ou banner conforme o layout.

Fluxo recomendado:

1. Procurar arte incorporada nos metadados do arquivo.
2. Validar formato, dimensões e tamanho.
3. Corrigir orientação quando necessário.
4. Criar uma versão quadrada otimizada para listas e player.
5. Salvar o arquivo derivado no cache privado do Rigbyfy.
6. Guardar apenas seu URI/caminho em `artwork_uri`.
7. Gerar um gradiente determinístico quando não houver imagem.

Não salvar imagens grandes como BLOB no SQLite. Também não manter bitmaps completos em memória nas listas. O cache deve possuir limites e poder ser reconstruído.

Uma implementação futura poderá guardar variantes:

- Miniatura para listas.
- Capa média para o player.
- Banner desfocado ou paleta de cores derivada da capa.

## Origem das músicas

O domínio deve trabalhar com uma abstração semelhante a:

```text
MediaSource
  ├── AndroidDocumentSource
  ├── AppPrivateFileSource
  └── RemoteDownloadSource (futuro)
```

Cada origem resolve um item para algo que o player consiga abrir, mas nenhuma tela Svelte deve acessar seus detalhes diretamente.

Tipos planejados:

- `local_document`: arquivo externo escolhido pelo Storage Access Framework.
- `app_file`: arquivo copiado para o armazenamento privado do Rigbyfy.
- `remote_download`: arquivo obtido legalmente de uma URL e salvo pelo gerenciador de downloads futuro.

O player recebe uma referência resolvida e os metadados normalizados. Ele não decide como baixar, importar ou persistir a música.

## Importação local no Android

Usar `ACTION_OPEN_DOCUMENT` para arquivos individuais e, futuramente, `ACTION_OPEN_DOCUMENT_TREE` para pastas permitidas pelo sistema.

Requisitos:

- Aceitar apenas MIME types e extensões suportadas.
- Solicitar acesso de leitura persistente à URI.
- Nunca depender de caminhos absolutos derivados de `content://`.
- Detectar músicas duplicadas por URI e, quando necessário, por hash parcial ou metadados.
- Tratar arquivo removido, movido ou permissão revogada.
- Processar múltiplos arquivos fora da thread da interface.
- Retornar progresso e erros individualizados ao Svelte.

Por padrão, a importação referencia o arquivo original e não cria uma cópia. Uma opção explícita poderá copiar o áudio para o armazenamento privado.

## Download de músicas da internet — evolução futura

O projeto será preparado para baixar músicas da internet futuramente, mas somente de fontes autorizadas, conteúdo próprio, domínio público ou serviços cujos termos permitam download.

Essa funcionalidade não deve exigir mudanças no modelo central nem nos componentes da interface. Para isso:

- Não usar a URI local como identificador da música.
- Separar `id` interno, `source_uri` e `source_type`.
- Manter metadados normalizados fora da implementação de download.
- Criar um contrato `MediaSource` para resolver a origem.
- Criar um contrato `DownloadManager` separado do player.
- Representar estados `queued`, `downloading`, `completed`, `failed` e `cancelled`.
- Persistir progresso, tamanho total, checksum e erro do download em tabela própria.
- Baixar para arquivo temporário e mover de forma atômica somente após validação.
- Validar HTTPS, MIME type, tamanho, espaço disponível e checksum.
- Nunca executar ou interpretar conteúdo baixado como código.
- Permitir pausar, retomar, cancelar e excluir o arquivo baixado.
- Aplicar limites de rede e opção de baixar somente por Wi-Fi.

O player só deve receber arquivos completos e validados. Streaming remoto, caso seja desejado no futuro, será uma capacidade separada de download offline.

## Persistência

O SQLite será a fonte de verdade da biblioteca. Arquivos de áudio e imagens ficam no sistema de arquivos.

Tabelas inicialmente previstas:

- `tracks`
- `artists`
- `albums`
- `playlists`
- `playlist_tracks`
- `playback_history`
- `playback_state`
- `artwork_cache`
- `media_permissions`
- `downloads` no futuro

Toda alteração de schema exige migration incremental. Uma migration já aplicada nunca deve ser editada.

## Player Android

O plugin Kotlin usará:

- Media3/ExoPlayer para reprodução.
- `MediaSessionService` para segundo plano.
- `MediaSession` para tela bloqueada, Bluetooth e controles externos.
- Notificação multimídia gerenciada pelo estado da sessão.
- `MediaController` como entrada dos comandos da interface.

Comandos mínimos:

- `setQueue`
- `play`
- `pause`
- `seekTo`
- `skipNext`
- `skipPrevious`
- `setShuffle`
- `setRepeatMode`
- `stop`

Eventos mínimos:

- Mudança de estado.
- Mudança de faixa.
- Posição e duração.
- Mudança da fila.
- Erro de reprodução.
- Mudança de rota de áudio.

O player nativo é a fonte de verdade da reprodução. Svelte envia intenções e renderiza o estado recebido.

## API para o Svelte

Os componentes não chamam `invoke` diretamente. Um serviço TypeScript tipado deve encapsular os comandos Tauri e expor operações de domínio:

```text
libraryService.importTracks()
libraryService.listTracks()
libraryService.updateTrackMetadata()
libraryService.removeTrack()

playerService.play(trackId)
playerService.pause()
playerService.seek(positionMs)
playerService.subscribe(listener)
```

Isso permite trocar dados simulados pelos serviços reais sem acoplar telas a Kotlin, Rust ou SQLite.

## Ordem de implementação

### Etapa 1 — Contratos e banco

- Definir entidades e erros Rust.
- Configurar SQLite e migrations.
- Criar repository e testes unitários.
- Criar modelos TypeScript equivalentes.

### Etapa 2 — Importação Android

- Criar plugin Tauri Kotlin.
- Abrir seletor de arquivos.
- Persistir permissões das URIs.
- Extrair metadados e artwork.
- Salvar e listar músicas reais.

### Etapa 3 — Reprodução básica

- Integrar Media3/ExoPlayer.
- Tocar e pausar uma música selecionada.
- Sincronizar faixa, estado, posição e duração com Svelte.
- Ligar player completo e miniplayer aos serviços reais.

### Etapa 4 — Segundo plano

- Implementar `MediaSessionService`.
- Configurar notificação, tela bloqueada, fones e Bluetooth.
- Tratar Audio Focus, chamadas e desconexão de fones.
- Restaurar fila e posição.

### Etapa 5 — Biblioteca completa

- Importação múltipla e por pasta.
- Busca, favoritos, playlists e histórico.
- Edição de metadados sem modificar o arquivo original por padrão.
- Detecção de arquivos ausentes e duplicados.

### Etapa futura — Downloads autorizados

- Implementar `RemoteDownloadSource` e `DownloadManager`.
- Criar tabela e fila de downloads.
- Adicionar segurança, retomada, integridade e políticas de rede.
- Reutilizar os mesmos modelos, repository, biblioteca e player.

## Testes obrigatórios

- Normalização de metadados ausentes ou inválidos.
- Duração e tamanho com limites válidos.
- Inserção, atualização e deduplicação no SQLite.
- Migration entre versões do banco.
- URI válida, revogada, movida e ausente.
- Arquivo sem capa, capa corrompida e imagem muito grande.
- Reinício do aplicativo e do aparelho.
- Play, pause, seek, anterior, próxima e fim da fila.
- Segundo plano, tela bloqueada, Bluetooth, chamada e desconexão de fone em aparelho real.
- Download interrompido, retomado, corrompido e sem espaço quando essa etapa existir.

## Critério da primeira entrega

A primeira entrega estará concluída quando uma música escolhida pelo seletor Android:

1. Aparecer na biblioteca com nome, artista, álbum, duração e capa.
2. Continuar cadastrada depois de fechar e abrir o aplicativo.
3. Puder ser reproduzida, pausada e reposicionada.
4. Exibir o mesmo estado no player completo e no miniplayer.
5. Gerar um erro compreensível se o arquivo deixar de estar disponível.
