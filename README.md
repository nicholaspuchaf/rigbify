# Rigbyfy

Player de músicas locais para Android, iOS e desktop, construído com Svelte, Tauri 2 e Rust. O usuário poderá importar as próprias músicas, organizar a biblioteca e ouvir tudo offline. Nenhum áudio será enviado para servidores.

## Objetivos

- Importar MP3, M4A, AAC, OGG, WAV e FLAC do aparelho
- Reproduzir offline, minimizado e com a tela desligada
- Mostrar capa, título, artista e controles na tela bloqueada
- Oferecer notificação com play, pausa, avançar e voltar
- Responder a fones de ouvido e Bluetooth
- Criar favoritos, playlists, histórico e fila persistente
- Entregar versões Android, iOS e desktop

## Tecnologias

| Camada | Tecnologia | Responsabilidade |
| --- | --- | --- |
| Interface | Svelte 5 + TypeScript | Telas, componentes e estado visual |
| Pacotes JS | Bun | Instalação, scripts e lockfile |
| Aplicativo | Tauri 2 | Builds mobile/desktop e comunicação entre camadas |
| Núcleo | Rust | Biblioteca, arquivos, metadados e regras |
| Banco | SQLite | Músicas, favoritos, playlists, histórico e configurações |
| Android | Kotlin + Media3/ExoPlayer | Reprodução, serviço e `MediaSession` |
| iOS | Swift + AVFoundation | Reprodução, sessão de áudio e controles remotos |

O Svelte roda na WebView, mas o áudio final não dependerá de `<audio>`. A reprodução será nativa para funcionar de forma confiável em segundo plano.

## Arquitetura

```text
Svelte (interface)
       │
       ▼
Tauri commands e events
       ├── Rust: SQLite, arquivos, metadados e regras
       └── Plugin de mídia
             ├── Android: Kotlin + Media3/ExoPlayer
             └── iOS: Swift + AVFoundation
```

### Android

- Media3/ExoPlayer para reprodução
- `MediaSession` para integração com o sistema
- `MediaStyle` para a notificação multimídia
- Foreground Service para segundo plano
- Audio Focus para chamadas e interrupções

### iOS

- `AVPlayer` e `AVAudioSession`
- `MPNowPlayingInfoCenter` para informações da faixa
- `MPRemoteCommandCenter` para controles externos
- Background Audio Capability para tela desligada

O Android pode ser desenvolvido no Linux. Compilar e testar para iOS exige macOS e Xcode.

## Armazenamento

- Áudios permanecem no sistema de arquivos do aparelho.
- SQLite armazena caminhos, metadados, favoritos, playlists e histórico.
- Preferências pequenas podem usar armazenamento chave-valor.
- Áudio não será salvo em `localStorage`, que é pequeno e inadequado para binários.

O plano técnico detalhado de importação, metadados, capas, player nativo e suporte futuro a downloads está em [`src-tauri/src/media/README.md`](src-tauri/src/media/README.md).

## Roadmap

### Fase 1 — Fundação

- [x] Definir stack e arquitetura
- [x] Criar interface mobile-first
- [x] Migrar a base para Svelte 5 + TypeScript
- [x] Criar estrutura Tauri 2/Rust
- [x] Criar protótipo navegável das cinco telas principais
- [x] Extrair componentes compartilhados de navegação, player e biblioteca
- [ ] Validar o frontend após instalar Bun e o Tauri após instalar as dependências nativas do sistema

### Fase 2 — Biblioteca local

- [ ] Configurar SQLite e migrations
- [ ] Criar seletor nativo de arquivos
- [ ] Ler metadados, capas e duração
- [ ] Implementar busca, favoritos, playlists e histórico
- [ ] Tratar arquivos removidos ou movidos

### Fase 3 — Player Android

- [ ] Criar plugin Tauri em Kotlin
- [ ] Integrar Media3/ExoPlayer
- [ ] Implementar Foreground Service e notificação
- [ ] Implementar tela bloqueada, Bluetooth e fones
- [ ] Sincronizar eventos do player com Svelte
- [ ] Testar chamadas e economia de bateria

### Fase 4 — Player iOS

- [ ] Criar implementação Swift do plugin
- [ ] Integrar AVFoundation e áudio em segundo plano
- [ ] Implementar Central de Controle e tela bloqueada

### Fase 5 — Produto

- [ ] Fila persistente e reprodução aleatória
- [ ] Temas, acessibilidade e testes automatizados
- [ ] Assinatura e publicação nas lojas

## Preparação

Instale Bun, Rust/Cargo e as [dependências oficiais do Tauri](https://v2.tauri.app/start/prerequisites/). Para Android, instale também Android Studio, JDK e Android SDK. O Bun será usado como gerenciador e executor, sem APIs exclusivas, preservando uma possível migração futura.

```bash
bun install
bun run dev             # interface no navegador
bun run tauri dev       # aplicativo desktop
bun run tauri android init
bun run tauri android dev
```

Em um Mac com Xcode:

```bash
bun run tauri ios init
bun run tauri ios dev
```

## Qualidade e hooks Git

O projeto usa ESLint, `svelte-check`, Vitest e Lefthook. `bun install` instala os hooks automaticamente.

```bash
bun run lint            # lint de TypeScript e Svelte
bun run lint:fix        # correções seguras do ESLint
bun run check           # tipos e diagnóstico Svelte
bun run test            # testes unitários
bun run validate        # lint, check, testes e build
bun run validate:rust   # format, Clippy e testes Rust
```

Antes de cada commit, o Lefthook verifica os arquivos alterados. Antes do push, executa a validação completa do frontend e, quando houver mudanças nativas, do Rust. Não ignore os hooks com `--no-verify`; corrija a falha ou documente uma exceção aprovada.

## Segurança e privacidade

- A biblioteca permanece no dispositivo.
- Permissões serão solicitadas somente quando necessárias.
- Os comandos Tauri terão escopo mínimo e caminhos serão validados no Rust.
- O app não fornecerá catálogo nem downloads de músicas protegidas.

## Estado atual

A fundação Svelte/Tauri e o design inicial estão criados. As especificações das telas principais ficam em `design/*.md` e serão a referência para os próximos componentes Svelte. A próxima entrega é implementar esse protótipo navegável, seguida pela biblioteca persistida em SQLite e pelo player Android nativo.
