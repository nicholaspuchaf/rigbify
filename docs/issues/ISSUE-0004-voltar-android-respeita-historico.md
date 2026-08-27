# ISSUE-0004 — Botão Voltar do Android não respeita a página anterior

- Status: em validação
- Prioridade: alta
- Área provável: navegação
- Registrada em: 2026-08-26

## Relato

Ao abrir uma playlist pela Biblioteca e usar o botão Voltar do Android, o aplicativo não retorna ao estado/tela anterior correto.

## Resultado esperado

O botão Voltar deve percorrer o histórico interno do Rigbyfy e sair do aplicativo somente quando a Home for a única tela restante.

## Análise inicial

Fato: `App.svelte` mantém `historyDepth` e usa `window.history`. O tratamento atual precisa ser validado contra rotas aninhadas, mudanças de tela por substituição e retorno do Android/WebView.

## Implementação

Uma matriz exaustiva de retorno foi centralizada em `src/services/navigation.ts`. Ela exige que todo destino `View` declare seu retorno e mantém a Home como único destino que delega a saída ao sistema. O `App.svelte` usa a matriz como fallback quando não há entrada de histórico para retornar.

## Critérios de aceite

- [ ] Biblioteca → Playlist → Voltar retorna à Biblioteca.
- [ ] Playlist → Adicionar músicas/Editar → Voltar retorna à Playlist.
- [ ] Player, Downloads e Configurações respeitam a origem.
- [ ] Home → Voltar permite o comportamento nativo de saída.
- [ ] Testes automatizados cobrem todas as telas atuais e uma matriz de telas futuras simuladas, para que novos destinos precisem declarar sua estratégia de retorno.
