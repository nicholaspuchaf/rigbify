# ISSUE-0002 — Tela branca pisca na inicialização

- Status: em validação
- Prioridade: média
- Área provável: Android / WebView
- Registrada em: 2026-08-26

## Relato

Uma tela branca é exibida brevemente antes da interface escura do Rigbyfy ao iniciar o aplicativo.

## Resultado esperado

A transição de inicialização deve manter fundo escuro coerente com a identidade visual, sem flash branco.

## Análise inicial

Hipótese: o tema/`windowBackground` da Activity ou a inicialização da WebView usa uma cor padrão antes do CSS Svelte ser aplicado. Confirmar em aparelho real e nas configurações Android geradas antes de alterar o tema.

## Implementação

- `index.html` define fundo, cor e `color-scheme` escuros antes de carregar os módulos da interface.
- `tauri.conf.json` define `backgroundColor` escuro para a janela, sem editar o projeto Android gerado.
- Falta validar em inicialização a frio e retorno do plano de fundo em aparelho Android.

## Critérios de aceite

- [ ] Não há flash branco em inicialização a frio e retorno do plano de fundo.
- [ ] Fundo inicial é escuro e acessível.
- [ ] Validação manual em aparelho Android.
