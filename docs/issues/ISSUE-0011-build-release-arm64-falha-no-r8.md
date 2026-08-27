# ISSUE-0011 — Build release ARM64 falha no R8 por classes XZ ausentes

- Status: em andamento
- Prioridade: alta
- Área provável: downloads / Android
- Registrada em: 2026-08-27

## Relato

A build Android release para ARM64 falha na tarefa de minificação `:app:minifyUniversalReleaseWithR8`.

## Resultado esperado

O APK release ARM64 deve ser gerado para distribuição.

## Passos para reproduzir

1. Executar `bun run tauri android build --target aarch64 --apk`.
2. Aguardar a etapa R8 da variante release.

## Evidências

- O R8 informa classes ausentes no pacote `org.tukaani.xz`.
- O yt-dlp Android 0.18.1 traz `commons-compress:1.12`.
- O POM dessa versão declara `org.tukaani:xz:1.5` como dependência opcional.

## Análise inicial

Fato confirmado: a variante debug não executa essa minificação; por isso compila. A variante release analisa referências do `commons-compress` e falha porque XZ não entra transitivamente no classpath Android. Avisos de APIs depreciadas e arquivos `consumer-rules.pro` ausentes são independentes da falha.

## Critérios de aceite

- [ ] O build release ARM64 conclui a etapa R8.
- [ ] A dependência necessária fica declarada no módulo que usa yt-dlp Android.
- [ ] A build release é validada e documentada.

## Resolução

Pendente de validação da build release.
