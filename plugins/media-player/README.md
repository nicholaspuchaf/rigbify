# Player nativo do Rigbyfy

Plugin Tauri responsável pela reprodução Android com Media3. O `PlaybackService` mantém o
`ExoPlayer` e a `MediaSession` fora da WebView, permitindo reprodução em segundo plano,
notificação multimídia, tela bloqueada e controles externos.

O plugin recebe uma fila normalizada e expõe `set_queue`, `play`, `pause`, `seek_to`,
`skip_next`, `skip_previous`, `stop` e `get_state`. No Android 13 ou posterior, a permissão de
notificações é solicitada no primeiro play. O áudio continua sendo controlado pelo serviço mesmo
se o usuário não autorizar notificações, sujeito às regras de foreground service do Android.
