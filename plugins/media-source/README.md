# Plugin Tauri `media-source`

Plugin de aquisição de músicas locais. Ele abre o seletor Android, mantém acesso às URIs escolhidas e extrai metadados; também permite selecionar a pasta de um bundle com `manifest.json` e áudios relativos. Não salva músicas no SQLite e não reproduz áudio.

## Contrato

A API Rust registra os comandos declarados em `src/commands.rs` e troca os DTOs de `src/models.rs` com a implementação mobile. O retorno representa a seleção do usuário e inclui URI/origem, nome, artista, álbum, duração, capa quando disponível e informações técnicas. Cancelar o seletor é um resultado recuperável, não um crash.

No Android, `MediaSourcePlugin.kt` usa `ACTION_OPEN_DOCUMENT` para áudios individuais e `ACTION_OPEN_DOCUMENT_TREE` para bundles. A seleção da pasta do bundle exige `manifest.json` na raiz e conserva uma permissão persistível de leitura para as URIs de seus áudios; o manifesto só pode resolver caminhos relativos abaixo dessa raiz. Para não bloquear a Activity, as URIs dos itens são construídas com `DocumentsContract` a partir da árvore autorizada, sem listar a pasta para cada faixa. `MediaMetadataMapper.kt` consulta `ContentResolver` e `MediaMetadataRetriever`, normaliza campos ausentes e produz o payload consumido pelo Rust.

## Estrutura

- `Cargo.toml`/`Cargo.lock`: crate e resolução das dependências.
- `build.rs`: geração das permissões Tauri.
- `src/lib.rs`: setup e API pública.
- `src/commands.rs`: superfície de comandos.
- `src/models.rs`: contratos serializáveis.
- `src/error.rs`: erro tipado do plugin.
- `src/mobile.rs`: bridge mobile.
- `src/desktop.rs`: fallback de plataforma não implementada.
- `permissions/`: conjunto padrão, comandos gerados e schema.
- `android/build.gradle.kts`: módulo Android, SDK e dependências.
- `android/src/main/AndroidManifest.xml`: declaração nativa.
- `android/src/main/java/MediaSourcePlugin.kt`: seletor e lifecycle do resultado.
- `android/src/main/java/MediaMetadataMapper.kt`: leitura/conversão de metadados.
- `android/src/test/java/MediaMetadataMapperTest.kt`: testes unitários do mapper.

## Bibliotecas

Tauri 2 no Rust/Android, Serde para DTOs, AndroidX Activity KTX 1.10.1 e Core KTX 1.16.0. O módulo compila com SDK 36, suporta a partir do SDK 21 e usa JVM target 1.8.

## Regras

- Preserve a permissão persistível; não converta `content://` em caminhos de filesystem frágeis.
- Não leia arquivos que o usuário não selecionou nem copie áudio implicitamente.
- Mantenha extração fora de responsabilidades de catálogo/playlist.
- Trate metadados ausentes, arquivos inválidos, formatos diferentes e permissão revogada.
- Uma mudança de DTO deve ser sincronizada com o núcleo Rust e os tipos/services do frontend.
- Atualize permissões e seus artefatos gerados pelo processo oficial, sem editar somente o schema gerado.
- Cubra conversões puras com JUnit e valide seleção real em aparelho.

Leia também [`../README.md`](../README.md) e [`../../src-tauri/README.md`](../../src-tauri/README.md).
