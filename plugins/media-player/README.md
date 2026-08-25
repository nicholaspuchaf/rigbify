# Plugin Tauri `media-player`

Plugin de reprodução nativa Android. Recebe uma fila normalizada e mantém ExoPlayer/MediaSession fora da WebView para suportar segundo plano, notificação multimídia, tela bloqueada e controles externos. Não consulta SQLite nem decide a composição de uma playlist.

## Contrato

Expõe `set_queue`, `play`, `pause`, `seek_to`, `skip_next`, `skip_previous`, `stop` e `get_state`. O estado serializado informa faixa/índice atual, posição, duração, disponibilidade de navegação e estado de reprodução. Media3 é a fonte de verdade; a interface não deve simular sucesso antes de reconciliar o estado retornado.

No Android 13 ou posterior, a permissão de notificações é solicitada no primeiro play. O serviço continua sujeito às regras de foreground service do Android quando o usuário nega essa permissão.

## Estrutura

- `Cargo.toml`: crate e dependências.
- `build.rs`: geração das permissões Tauri.
- `src/lib.rs`: registro e API pública.
- `src/commands.rs`: operações expostas.
- `src/models.rs`: fila, requests e estado serializável.
- `src/error.rs`: falhas tipadas.
- `src/mobile.rs`: bridge para Kotlin.
- `src/desktop.rs`: indisponibilidade explícita em desktop.
- `permissions/`: default, comandos gerados e schema.
- `android/build.gradle.kts`: Media3 e dependências Android.
- `android/src/main/AndroidManifest.xml`: serviço e permissões de foreground/media.
- `android/src/main/java/PlaybackService.kt`: proprietário do ExoPlayer e MediaSession.
- `android/src/main/java/MediaPlayerPlugin.kt`: comandos, binding e permissão de notificação.
- `android/src/main/java/PlaybackModels.kt`: DTOs e mapper de estado.
- `android/src/test/java/PlaybackStateMapperTest.kt`: teste unitário do mapper.

## Bibliotecas

Media3 ExoPlayer/Session 1.7.1, Tauri 2 Android, AndroidX, AppCompat e Material; JUnit/Espresso para testes. O módulo usa compile SDK 36, min SDK 21 e JVM target 1.8. Há duas declarações atuais de Core KTX (1.16.0 e 1.9.0); Gradle resolve a versão efetiva, mas elas devem ser consolidadas numa mudança de manutenção validada.

## Regras

- `PlaybackService` continua proprietário do player; não prenda reprodução ao ciclo da Activity/WebView.
- Converta toda entrada em `MediaItem` validado e mantenha metadados coerentes com a notificação.
- Trate fila vazia, índice inválido, URI inacessível, erro de decoder e serviço desconectado.
- Preserve idempotência razoável em play/pause/stop e consistência nas trocas rápidas de fila.
- Integração de Audio Focus, noisy intent, Bluetooth, chamadas e restauração deve ser validada em aparelho real.
- Não adicione regras de favoritos/playlists nem persistência SQLite ao plugin.
- Mudanças de contrato exigem atualização sincronizada em Kotlin, Rust, TypeScript, permissions e testes.

Leia também [`../README.md`](../README.md), [`../../src/README.md`](../../src/README.md) e [`../../src-tauri/README.md`](../../src-tauri/README.md).
