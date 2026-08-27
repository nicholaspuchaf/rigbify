# Pasta `src-tauri`: aplicação Tauri e núcleo Rust

## Escopo

Esta pasta contém o host Tauri, configuração/capabilities, núcleo de domínio, persistência SQLite e o projeto Android gerado. É a fronteira confiável do aplicativo: valida entradas da WebView, aplica regras da biblioteca/playlists, registra plugins e controla onde os dados locais são armazenados.

O áudio não é copiado para o banco. O catálogo guarda URI/origem e metadados; a reprodução recebe esses identificadores pelo plugin Android.

## Dependências

### Rust

- `tauri` 2: runtime, commands, estado e registro de plugins; habilitado pela feature `app`.
- `serde` e `serde_json`: contratos serializáveis.
- `rusqlite` 0.37 com SQLite `bundled`: banco local consistente entre ambientes.
- `thiserror` 2: erros tipados.
- `uuid` 1 com v4: identificadores de entidades.
- `unicode-normalization` 0.1: ordenação estável de texto com case e diacríticos normalizados.
- `tauri-plugin-media-source` e `tauri-plugin-media-player`: crates locais opcionais, usados pela feature `app`.

A feature padrão `app` liga o shell Tauri e plugins. Testes do núcleo usam `--no-default-features --lib` para evitar dependências gráficas/plataforma quando elas não são necessárias.

### Build e plataforma

- `build.rs` executa o build helper do Tauri.
- `tauri.conf.json` aponta o dev server Vite para a porta 1420, bundle web para `../dist`, janela 430×820, fundo inicial escuro e identificador Android `com.rigbyfy.app`.
- `capabilities/default.json` habilita `core:default`, `media-source:default` e `media-player:default` para a janela principal.
- O build Android usa o NDK configurado no ambiente e o projeto gerado em `gen/android`.

## Estrutura detalhada

```text
src-tauri/
├── README.md
├── Cargo.toml              # crate, feature app e dependências
├── Cargo.lock              # resolução Rust reproduzível
├── build.rs                # geração Tauri em build time
├── tauri.conf.json         # produto, frontend, janela e bundle
├── capabilities/
│   └── default.json        # permissões da janela/WebView
├── icons/
│   ├── icon.png            # fonte raster do ícone de produção
│   └── candidates/         # alternativas visuais, com fones/equalizador aprovada
├── gen/android/            # projeto Gradle gerado pelo Tauri
└── src/
    ├── main.rs             # binário mínimo que chama a biblioteca
    ├── lib.rs              # composição: estado, banco, plugins e commands
    ├── commands/
    │   ├── mod.rs
    │   ├── media.rs        # API Tauri da biblioteca e favoritos
    │   └── playlists.rs    # API Tauri de playlists
    ├── database/
    │   ├── mod.rs          # conexão, inicialização e migrations
    │   ├── sqlite_track_repository.rs
    │   ├── sqlite_playlist_repository.rs
    │   └── migrations/
    │       ├── 0001_tracks.sql
    │       ├── 0002_track_actions.sql
    │       └── 0003_playlist_details.sql
    ├── errors/
    │   └── mod.rs          # erros da aplicação e serialização ao frontend
    └── media/
        ├── README.md       # decisões e evolução do domínio de mídia
        ├── mod.rs          # exports do módulo
        ├── models.rs       # entidades/DTOs de músicas
        ├── library.rs      # casos de uso da biblioteca
        ├── repository.rs   # contrato TrackRepository
        ├── playlist.rs     # entidades, requests e ordenações
        ├── playlist_library.rs # regras/casos de uso de playlists
        └── playlist_repository.rs # contrato PlaylistRepository
```

## Arquivos de composição

### `src/main.rs`

Entry point do executável. Mantém o binário fino e delega a inicialização à função pública do crate, permitindo testar a biblioteca sem subir a aplicação.

### `src/lib.rs`

Composition root. Inicializa o banco e estado compartilhado, registra os plugins locais, expõe commands e executa o contexto Tauri. Dependências concretas devem ser conectadas aqui, não criadas aleatoriamente dentro das regras de negócio.

### `tauri.conf.json`

Contrato de build do produto. Mudanças de identificador, comandos beforeDev/beforeBuild, URL, janela ou segurança afetam todas as plataformas. A CSP está `null` atualmente e precisa de política explícita antes da distribuição pública.

### `capabilities/default.json`

Define capacidades acessíveis pela WebView. Siga menor privilégio: novos comandos/plugins só devem ser adicionados quando uma feature precisa deles e depois de revisar o impacto.

## `commands/`: adaptadores Tauri

- `media.rs` recebe requests do frontend para listar/importar, editar metadados, favoritar e remover. Inclui `import_bundle`, que coordena a leitura Android do `Rigbyfy-import`, upsert idempotente das faixas e associação às playlists sem expor SQLite à WebView.
- `playlists.rs` expõe criação, listagem, detalhe, edição, duplicação, limpeza, exclusão, adição, remoção, reordenação e ordenação.
- `mod.rs` organiza os módulos públicos.

Commands não devem conter SQL nem acumular regra de negócio. Eles convertem erros para respostas serializáveis, preservam nomes estáveis e delegam. Uma nova operação exige sincronização com `src/services`, capability quando aplicável e teste em uma camada inferior.

## `media/`: domínio e casos de uso

- `models.rs` representa música importada/persistida e alterações permitidas.
- `repository.rs` define o contrato de persistência de faixa; desacopla os casos de uso de SQLite.
- `library.rs` aplica regras da biblioteca, como normalização/deduplicação e mutações de catálogo.
- `playlist.rs` define playlist, resumo/detalhe, faixa posicionada, critérios de ordenação e requests.
- `playlist_repository.rs` descreve operações necessárias sem impor SQL.
- `playlist_library.rs` coordena validações e regras: nome válido, playlist/faixa existente, ausência de duplicatas, posições, versão otimista e mutações atômicas.

Esse módulo deve permanecer compilável/testável sem Tauri sempre que possível. Uma futura origem de músicas baixadas da internet deve implementar/adaptar os mesmos contratos, registrar sua origem e entregar uma URI reproduzível, sem acoplar download à UI ou ao player.

## `database/`: SQLite

### Inicialização

`mod.rs` abre a conexão local, habilita/configura o necessário e aplica migrations incorporadas em ordem. A conexão é compartilhada de forma controlada pelo estado da aplicação.

### Repositories

- `sqlite_track_repository.rs`: SQL do catálogo, busca/listagem, importação idempotente, favoritos, registro de reprodução, edição e remoção. A listagem recente é ordenada pelo índice de última reprodução.
- `sqlite_playlist_repository.rs`: SQL de playlists e relações, agregados, ordenações, posições e transações. A comparação de título, artista e álbum normaliza case e diacríticos antes de ordenar, mantendo campos vazios no final; a listagem recente usa a última reprodução persistida.

Somente implementações de repository devem conhecer tabelas e consultas. Retorne entidades do domínio, não rows/crates SQLite para a interface.

### Migrations

- `0001_tracks.sql`: tabela base de músicas e seus metadados/origem.
- `0002_track_actions.sql`: favorito, playlists e relacionamento música-playlist.
- `0003_playlist_details.sql`: descrição, capa, preferência de ordenação, posição customizada e versão de concorrência.
- `0005_recent_playback.sql`: última reprodução de playlist e índices para ordenação recente de faixas/playlists.

Regras obrigatórias:

1. Nunca reescreva uma migration que possa ter sido aplicada em aparelho.
2. Crie o próximo número monotônico e mantenha upgrade transacional.
3. Defina defaults/backfill para dados existentes.
4. Teste banco vazio e atualização a partir da versão anterior.
5. Preserve integridade referencial, índices úteis e exclusões intencionais.

## Playlists e consistência

O detalhe deriva contagem/duração das relações válidas. Inclusão impede a mesma faixa duas vezes. Alterações múltiplas são transacionais. `version` implementa concorrência otimista: um cliente com versão antiga deve recarregar em vez de sobrescrever silenciosamente uma edição mais nova. A posição customizada é persistida mesmo que o usuário escolha uma ordenação calculada para exibição.

## Integração com plugins

O núcleo depende dos plugins locais em [`../plugins/README.md`](../plugins/README.md):

- `media-source` devolve seleção e metadados; o núcleo decide persistência.
- `media-source` também abre o seletor de pastas Android para o bundle, exige que a pasta escolhida contenha `manifest.json`, resolve os caminhos relativos de áudio via Storage Access Framework, copia capas com tamanho limitado para o cache privado e valida que os caminhos permanecem dentro desse diretório antes de devolver URIs/metadados locais.
- `load_artwork_data_uri` aceita somente uma URI `file://` localizada no cache privado `artwork/`, limita a leitura a 10 MiB e devolve uma URL `data:` para a WebView. A URI de arquivo persistida não é substituída, pois é a entrada do Media3 para notificação e tela bloqueada.
- `media-player` recebe a fila preparada; o núcleo/plugin não consulta playlists por conta própria.

Não replique APIs Android em commands do domínio. Encapsule diferenças de plataforma no plugin e mantenha DTOs pequenos.

## `gen/android/`

Projeto Gradle criado/atualizado pelo Tauri para empacotar a biblioteca Rust e plugins Kotlin. Contém manifest agregado, Activities geradas, jniLibs e configuração de build.

- Não faça mudança duradoura diretamente em arquivos gerados quando houver fonte equivalente em `tauri.conf.json`, `capabilities` ou `plugins/*/android`.
- Não documente um arquivo gerado como API estável.
- Artefatos, caches, APKs e bibliotecas `.so` não devem entrar em commits.
- O aviso de `onBackPressed` no `WryActivity.kt` vem do código gerado e não impede por si só o build.

## Erros, segurança e privacidade

- `errors/mod.rs` centraliza falhas recuperáveis com mensagens úteis e serializáveis.
- Não use `unwrap`/`expect` em entrada, I/O, banco ou plugin no fluxo de produção.
- Valide IDs, texto, URIs e limites na fronteira nativa, mesmo que o frontend já valide.
- Não registre caminhos completos da biblioteca nem conteúdo do usuário.
- Remover um registro não deve apagar o arquivo original sem um caso de uso separado e confirmação explícita.
- Use queries parametrizadas; nunca concatene entrada do usuário em SQL.

## Padrão para novas features

1. Modele entidade/request e erro no domínio.
2. Acrescente ou evolua o trait do repository somente com a capacidade necessária.
3. Implemente e teste a regra contra um repository isolável.
4. Adicione migration incremental se houver mudança persistente.
5. Implemente SQLite com transação quando houver múltiplas gravações.
6. Exponha um command fino e DTO estável.
7. Ligue o command em `lib.rs`, service TypeScript e capability quando necessário.
8. Atualize este README, o README raiz e o guia da área consumidora.

Prefira baixo acoplamento, alta coesão, funções de escopo pequeno e dependência em traits. Não crie uma abstração genérica sem um conceito real do domínio.

## Testes e validação

```bash
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --no-default-features --lib -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib
cargo check --manifest-path src-tauri/Cargo.toml --target aarch64-linux-android
```

O núcleo contém testes unitários de biblioteca e playlists. Mudanças em SQL precisam de testes de repository/migration. Mudanças na ponte exigem compilação com a feature `app`; mudanças Android exigem Gradle e aparelho/emulador. Reprodução em segundo plano, notificação e tela bloqueada só são consideradas validadas após teste em aparelho real.

Estado de validação na criação deste documento: frontend completo, Clippy/testes do núcleo e `cargo check` ARM64 passaram; o último APK completo após as alterações de playlist foi interrompido e permanece pendente.
