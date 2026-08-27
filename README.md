# Rigbyfy

Rigbyfy é um player de músicas locais, mobile-first e offline, construído com Svelte 5, Tauri 2, Rust, SQLite e, no Android, Kotlin com Media3/ExoPlayer. O aplicativo importa arquivos escolhidos pelo usuário sem copiar o áudio para o banco, mantém catálogo e playlists locais e delega a reprodução ao sistema operacional.

> Estado atual: a interface, biblioteca local, favoritos, edição de metadados, playlists e integração inicial do player Android estão implementados. O frontend e o núcleo Rust foram validados por lint, tipos, testes e build. A instalação Android completa ainda precisa ser revalidada em aparelho após as últimas alterações; iOS e reprodução desktop ainda não foram implementados.

## Funcionalidades atuais

### Biblioteca local

- Seleção de vários arquivos pelo seletor nativo do Android (`ACTION_OPEN_DOCUMENT`).
- Permissão persistente para continuar acessando as `content:// URI` escolhidas.
- Extração de título, artista, álbum, duração, capa, MIME type, tamanho, número da faixa/disco, ano e gênero com `MediaMetadataRetriever`.
- Catálogo persistente em SQLite, com deduplicação pela origem e URI.
- Busca por título, artista e álbum.
- Marcação e remoção de favoritos.
- Edição local de título, artista e álbum sem alterar o arquivo original.
- Remoção da música do catálogo sem apagar o arquivo do aparelho.
- Importação de bundle por seletor de pasta Android: lê o `manifest.json` escolhido, registra somente áudios presentes e recria/atualiza playlists sem duplicar relações.
- Mocks equivalentes no navegador para desenvolver a interface sem o runtime Tauri.

### Playlists

- Criar, listar, abrir, renomear, duplicar, limpar e excluir playlists.
- Editar descrição e capa de referência da playlist.
- Adicionar uma música pelo menu de ações ou várias músicas por uma tela pesquisável.
- Filtrar rapidamente as favoritas durante a seleção.
- Impedir a duplicação da mesma música dentro de uma playlist.
- Remover faixas e reorganizá-las por controles acessíveis de mover para cima/baixo.
- Ordenar por ordem personalizada, título, artista, álbum ou adição recente.
- Preservar posições personalizadas no banco mesmo ao visualizar outra ordenação.
- Calcular quantidade e duração total a partir das faixas existentes.
- Usar transações SQLite e versão otimista para evitar gravações concorrentes silenciosas.
- Iniciar a fila respeitando a ordem visível ou embaralhá-la.
- Reconstruir a playlist ativa ao abrir a tela do player.

### Reprodução Android

- Fila nativa em Media3/ExoPlayer.
- Play, pause, seek, anterior, próxima e stop.
- `MediaSessionService` para reprodução em segundo plano.
- Serviço em primeiro plano e notificação multimídia com metadados da faixa.
- Solicitação da permissão de notificação no Android quando necessária.
- Consulta do estado nativo pelo frontend, atualmente a cada segundo.
- Ações de música compartilhadas entre listas e player: editar metadados, adicionar a playlist, favoritar/desfavoritar e remover do aplicativo.

### Interface

- Telas Home, Busca, Biblioteca, detalhe de álbum/artista, Playlist, Adicionar músicas, Editar playlist, Player e Configurações.
- Navegação inferior, mini player, cards, linhas de música e menu de ações reutilizáveis.
- Home com recomendações locais navegáveis, músicas adicionadas recentemente e seção Mais tocadas baseada no contador persistido da biblioteca.
- Em **Recentes** na Biblioteca, músicas e playlists são ordenadas no SQLite pela última reprodução; o app não reordena a biblioteca inteira na memória.
- Histórico interno integrado ao botão/gesto Voltar do Android; o app só é encerrado quando a Home é a raiz ativa.
- Layout escuro, mobile-first, inspirado nas referências de [`design/`](design/README.md), incluindo fundo inicial escuro antes do carregamento do Svelte.
- Service Worker simples para o shell web em builds de produção.
- Ícone de produção de fones/equalizador em PNG, com formatos desktop, iOS e Android regenerados.

### Downloads autorizados

- Acesso pelo banner da Home ou pelo atalho “Baixar mídia autorizada” na seção Buscar.
- Consulta de URLs HTTPS do YouTube/YouTube Music antes do download.
- Metadados, thumbnail e formatos disponíveis com seleção de áudio/vídeo, original, MP3, M4A, Opus e MP4.
- Progresso por eventos, pós-processamento FFmpeg, cancelamento por `downloadId` e registro automático na biblioteca.
- Contratos com `jobId`, `downloadId` e `source.kind` preparados para playlists futuras; a primeira versão aceita uma mídia por job.
- Confirmação explícita de que o usuário tem autorização para baixar o conteúdo.
- Ferramenta local em [`temp/README.md`](temp/README.md) para preparar CSVs do Spotify, gerar candidatos via YouTube Data API para revisão explícita e baixar uma playlist por execução antes de enviá-la ao aparelho via ADB. Ela não escreve no banco do app nem importa o bundle automaticamente.

## Limitações conhecidas

- A implementação nativa do player existe apenas para Android; desktop retorna indisponibilidade e iOS ainda não possui implementação Swift.
- O build APK completo depois das mudanças mais recentes foi interrompido antes da conclusão e deve ser repetido em aparelho real.
- Bluetooth, chamadas, perda de foco, economia de bateria e restauração após encerramento do processo ainda exigem testes completos em dispositivos reais.
- Reordenação de playlist usa botões; gesto de arrastar ainda não existe.
- Capas extraídas ou recebidas no bundle são persistidas como referência e renderizadas nas faixas e no player. Capas de bundles são copiadas com limite de 10 MiB ao cache privado e mantêm uma URI de arquivo local, utilizada pela interface e pelo player nativo; referências indisponíveis usam um fallback visual.
- Histórico, fila persistente entre reinícios, downloads de playlists e compartilhamento `ACTION_SEND` são evoluções planejadas.
- O downloader aceita somente mídia individual e salva no armazenamento privado do app; exportação para Downloads/MediaStore ainda não está exposta.
- A política CSP está desabilitada em `tauri.conf.json` durante esta fase e deve ser endurecida antes de distribuição.

## Stack

| Camada | Tecnologia | Responsabilidade |
| --- | --- | --- |
| Interface | Svelte 5 + TypeScript | Telas, componentes e estado visual |
| Build web | Vite 7 | Dev server, HMR e bundle |
| Pacotes JS | Bun | Instalação, scripts e `bun.lock` |
| Shell do app | Tauri 2 | Janela, mobile build e ponte JS/Rust/plugins |
| Núcleo | Rust | Casos de uso, contratos, validação e acesso aos repositories |
| Persistência | SQLite via `rusqlite` | Músicas, favoritos e playlists |
| Android | Kotlin + Media3 1.7.1 + yt-dlp Android 0.18.1 | Arquivos, player, Python, yt-dlp e FFmpeg |
| Qualidade | ESLint, svelte-check, Vitest, Clippy e Lefthook | Análise, testes, build e hooks Git |

## Arquitetura

```text
Svelte screens/components
        │
        ▼
TypeScript services (contratos tipados e fallback web)
        │ invoke
        ▼
Tauri commands em Rust ───────► SQLite repositories
        │
        ├──────────────────────► plugin media-source ─► Kotlin/Android
        ├──────────────────────► plugin media-player ─► Media3/ExoPlayer
        └──────────────────────► plugin media-downloader ─► yt-dlp/Python/FFmpeg
```

O player nativo é a fonte de verdade da reprodução. O SQLite é a fonte de verdade do catálogo e das playlists. Componentes não acessam SQLite, Kotlin ou comandos Tauri diretamente: telas chamam serviços TypeScript, comandos validam/adaptam a entrada e os módulos Rust executam as regras.

### Padrões adotados

- **Arquitetura em camadas:** apresentação, adaptadores TypeScript/Tauri, domínio/casos de uso, persistência e plataforma nativa.
- **Repository:** traits Rust isolam regras de biblioteca e playlist dos detalhes do SQLite.
- **Service/Application Service:** módulos de biblioteca e playlist coordenam validação, repositories e transações.
- **Adapter/Bridge:** serviços TypeScript e plugins Tauri convertem contratos entre WebView, Rust e Kotlin.
- **Plugin:** integrações Android de seleção/metadados e reprodução vivem em plugins independentes.
- **DTOs tipados:** os limites entre camadas usam estruturas serializáveis explícitas.
- **Dependency Inversion:** regras Rust dependem de contratos, permitindo testes com repositories em memória.
- **Single Source of Truth:** SQLite governa catálogo; Media3 governa reprodução.
- **Optimistic Concurrency:** playlists carregam uma versão e rejeitam atualização baseada em estado antigo.
- **Unit of Work/Transaction:** mutações relacionadas de playlist são atômicas.
- **Migrations incrementais:** schema evolui por SQL numerado e nunca pela reescrita de uma migration aplicada.
- **Progressive enhancement:** no navegador, serviços usam mocks; no Tauri, usam integrações nativas.
- **Composição de componentes:** elementos visuais compartilhados evitam repetição entre telas.

## Como cada área foi implementada

### Importação e metadados

`libraryService.ts` chama o comando Rust de importação. O núcleo abre o plugin `media-source`, que inicia o seletor Android, preserva permissões das URIs e mapeia metadados nativos. O resultado volta ao Rust, é normalizado e persistido por `SqliteTrackRepository`. No navegador, o mesmo serviço fornece dados mockados para manter o desenvolvimento visual funcional.

### Importação de bundle transferido por ADB

Na Biblioteca, **Importar bundle** abre o seletor de pastas do Android. Selecione a pasta que contém `manifest.json` e `audio/` — no bundle enviado por ADB, normalmente `Musics/rigify/imports`. O Storage Access Framework concede acesso persistente somente à pasta selecionada; o plugin valida os caminhos relativos declarados no manifesto antes de devolver URIs e metadados locais. O command Rust `import_bundle` é o coordenador: persiste as faixas de modo idempotente pelo par origem/URI e adiciona os IDs resultantes às playlists descritas no manifesto. O bundle continua no armazenamento compartilhado e não é copiado para SQLite.

### Favoritos, edição e remoção

O componente `TrackActions` centraliza as opções. A tela envia a intenção ao serviço; o comando Rust busca e atualiza o registro via repository. Edição altera somente o catálogo. Remoção exclui relações e registro local, não o áudio físico escolhido pelo usuário.

### Playlists

`playlistService.ts` expõe uma API única para as telas. No backend, `commands/playlists.rs` traduz requests e `media/playlist_library.rs` aplica regras sobre o contrato `PlaylistRepository`. `SqlitePlaylistRepository` executa consultas e transações nas tabelas criadas pelas migrations `0002` e `0003`. A ordem customizada é armazenada separadamente da ordenação de visualização.

### Player Android

`playerService.ts` monta a fila e chama o plugin `media-player`. No Android, `PlaybackService` mantém ExoPlayer e MediaSession fora do ciclo de vida da WebView; `MediaPlayerPlugin` expõe comandos Tauri e converte o estado para payloads estáveis. A UI consulta esse estado e atualiza player completo e mini player.

### Downloads

`DownloadScreen.svelte` consulta `downloadService` para obter metadados e formatos antes de baixar. Commands Rust validam URL, origem, qualidade e formato, geram IDs e delegam ao plugin `media-downloader`. O Kotlin mantém cada processo yt-dlp por `downloadId`, emite progresso, executa FFmpeg quando necessário, limpa temporários em cancelamento/falha e retorna a URI final. `register_completed_download` reutiliza `MediaLibrary::import` e registra a origem externa no SQLite.

### Interface e navegação

`App.svelte` é o coordenador da navegação atual e mantém o estado necessário entre telas. Screens compõem componentes genéricos; serviços encapsulam efeitos externos e tipos mantêm os contratos explícitos. Esta navegação é intencionalmente leve enquanto o conjunto de rotas permanece pequeno.

## Estrutura fiel do repositório

Arquivos gerados de build (`node_modules`, `target`, `dist` e artefatos Gradle) são omitidos. A pasta `src-tauri/gen/android` aparece porque é necessária ao build, mas seu conteúdo é gerado pelo Tauri e não deve ser tratado como fonte primária.

```text
rigbyfy/
├── .gitignore
├── AGENTS.md
├── README.md
├── package.json, bun.lock
├── eslint.config.js, lefthook.yml
├── vite.config.ts, tsconfig.json
├── index.html, icon.svg, manifest.webmanifest
├── design/
│   ├── README.md
│   ├── config.{jpeg,md}
│   ├── edit_playlist.{jpeg,md}
│   ├── home.{jpeg,md}
│   ├── myPlace.{jpeg,md}
│   ├── playerMusica.{jpeg,md}
│   ├── playlist_adicionar_musica.{jpeg,md}
│   ├── playlist_home.{jpeg,md}
│   ├── playlist_ordenar.{jpeg,md}
│   └── search.{jpeg,md}
├── plugins/
│   ├── README.md
│   ├── permissions/default.toml
│   ├── media-source/       # seletor e metadados locais
│   ├── media-player/       # reprodução nativa Android
│   └── media-downloader/   # yt-dlp/Python/FFmpeg
├── docs/
│   └── issues/             # backlog local versionado: índice, template e registros
├── public/
│   ├── README.md
│   └── sw.js
├── temp/                    # ferramenta versionada; CSVs e bundles locais são ignorados
│   ├── README.md
│   ├── pyproject.toml
│   ├── rigbyfy_import.py
│   ├── test_rigbyfy_import.py
│   └── uv.lock
├── src/
│   ├── README.md
│   ├── App.svelte, main.ts, style.css, vite-env.d.ts
│   ├── components/         # 12 componentes compartilhados
│   ├── data/mock.ts
│   ├── screens/            # telas, incluindo DownloadScreen
│   ├── services/           # biblioteca, player, playlists, downloads e testes
│   └── types/              # library, music, player, playlist e download
└── src-tauri/
    ├── README.md
    ├── Cargo.toml, Cargo.lock, build.rs, tauri.conf.json
    ├── capabilities/default.json
    ├── icons/icon.svg
    ├── gen/android/        # projeto Android gerado
    └── src/
        ├── main.rs, lib.rs
        ├── commands/{mod.rs,media.rs,playlists.rs}
        ├── database/
        │   ├── mod.rs
        │   ├── sqlite_track_repository.rs
        │   ├── sqlite_playlist_repository.rs
        │   └── migrations/{0001_tracks.sql,0002_track_actions.sql,0003_playlist_details.sql,0004_download_origins.sql}
        ├── errors/mod.rs
        └── media/
            ├── README.md, mod.rs, models.rs
            ├── library.rs, repository.rs
            └── playlist.rs, playlist_library.rs, playlist_repository.rs
```

Documentação detalhada por área:

- [`design/README.md`](design/README.md): inventário e processo das referências visuais.
- [`plugins/README.md`](plugins/README.md): contratos e implementação dos plugins nativos.
- [`public/README.md`](public/README.md): ativos públicos e Service Worker.
- [`src/README.md`](src/README.md): frontend, telas, componentes, serviços e tipos.
- [`src-tauri/README.md`](src-tauri/README.md): núcleo Rust, SQLite, comandos e build mobile.
- [`src-tauri/src/media/README.md`](src-tauri/src/media/README.md): plano e decisões específicas do domínio de mídia.

## Preparação do ambiente

### Base

Instale Bun, Rust/Cargo e as dependências de sistema descritas nos [pré-requisitos do Tauri 2](https://v2.tauri.app/start/prerequisites/).

```bash
bun install
```

O `prepare` instala os hooks do Lefthook. Bun é o gerenciador oficial, mas o código evita APIs exclusivas do runtime para preservar uma futura migração.

### Android

Instale Android Studio, Android SDK, Platform Tools, Command-line Tools, NDK e um JDK compatível com o Gradle usado. Configure no `~/.zshrc`, ajustando os caminhos locais:

```bash
export JAVA_HOME="/caminho/para/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.13846066"
export ANDROID_NDK_HOME="$NDK_HOME"
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
```

Depois recarregue o shell e confira:

```bash
source ~/.zshrc
java -version
adb devices
bun run tauri android init
bun run tauri android dev
```

Use um JDK suportado pelo Gradle do projeto; versões muito novas podem causar `Unsupported class file major version`.

## Executar

```bash
bun run dev                    # interface no navegador
bun run tauri dev              # shell desktop; player nativo não implementado
bun run tauri android dev      # aparelho/emulador Android
bun run tauri android build    # APK/AAB de distribuição
```

No Android, habilite depuração USB, autorize o computador e confirme o dispositivo com `adb devices`. A primeira compilação baixa dependências e compila Rust/Gradle, portanto é consideravelmente mais lenta; builds seguintes reutilizam caches.

## Scripts e qualidade

| Comando | Executa |
| --- | --- |
| `bun run dev` | Vite em desenvolvimento |
| `bun run build` | Bundle de produção com Vite |
| `bun run preview` | Preview do bundle web |
| `bun run lint` | ESLint em TS/Svelte |
| `bun run lint:fix` | Correções automáticas seguras |
| `bun run check` | Diagnósticos Svelte/TypeScript |
| `bun run test` | Vitest uma vez |
| `bun run test:watch` | Vitest em modo observação |
| `bun run validate` | lint, check, testes e build web |
| `bun run validate:rust` | fmt, Clippy e testes do núcleo Rust |
| `bun run tauri ...` | CLI Tauri via Bun |

Validação nativa específica:

```bash
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --no-default-features --lib -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib
cargo check --manifest-path src-tauri/Cargo.toml --target aarch64-linux-android
```

Os testes atuais cobrem mapeamento e comportamento dos serviços TypeScript, regras Rust de biblioteca/playlists e mapeadores Kotlin. Integrações reais de Android precisam também de build Gradle e testes em aparelho. O Lefthook executa verificações rápidas antes do commit e validações completas antes do push.

## Banco e migrations

As migrations são aplicadas em ordem ao abrir o banco:

1. `0001_tracks.sql`: catálogo e metadados das músicas.
2. `0002_track_actions.sql`: favoritos e estrutura inicial de playlists/relações.
3. `0003_playlist_details.sql`: descrição, capa, ordenação, posições e versão.
4. `0004_download_origins.sql`: URL da página e ID externo para músicas obtidas por download.
5. `0005_recent_playback.sql`: última reprodução de playlists e índices para ordenação recente de músicas e playlists.

Nunca altere uma migration já aplicada. Crie o próximo arquivo numerado, faça a mudança de forma transacional e teste tanto banco vazio quanto upgrade.

## Privacidade e segurança

- Áudio não é salvo no SQLite nem em `localStorage`.
- O arquivo original permanece sob controle do usuário e do Android Storage Access Framework.
- Remover do Rigbyfy não apaga o arquivo físico.
- Capacidades Tauri são explícitas para os dois plugins.
- Não há telemetria nem envio da biblioteca para servidor.
- Downloads futuros devem entrar como uma nova origem atrás dos contratos existentes, com consentimento, validação e respeito a direitos autorais.

## Fluxo para contribuir

1. Leia [`AGENTS.md`](AGENTS.md), este README e o README da pasta que será alterada.
2. Confirme a camada responsável antes de escrever código.
3. Implemente contratos e testes proporcionais ao risco.
4. Atualize documentação e migrations quando aplicável.
5. Execute `bun run validate` e as validações Rust/Android relevantes.
6. Revise o diff e não inclua artefatos gerados, configurações pessoais ou segredos.
