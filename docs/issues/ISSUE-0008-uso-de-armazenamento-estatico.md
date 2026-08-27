# ISSUE-0008 — Uso de armazenamento em Configurações é estático

- Status: resolvida
- Prioridade: média
- Área provável: interface / biblioteca
- Registrada em: 2026-08-27

## Relato

A seção Uso de armazenamento exibe valores fixos e não informa o volume efetivo das músicas da biblioteca.

## Resultado esperado

Configurações deve mostrar a quantidade de músicas e a soma dos tamanhos conhecidos, atualizada a partir da biblioteca local.

## Evidências

- `SettingsScreen.svelte` contém o texto fixo `24 MB em músicas e capas`.

## Critérios de aceite

- [x] Quantidade e bytes das músicas vêm do catálogo atual.
- [x] Faixas sem tamanho conhecido são identificadas sem estimativa enganosa.
- [x] Há teste para formatação e agregação.

## Resolução

`storageUsage.ts` calcula a quantidade de músicas e a soma de `fileSize` retornado pelo catálogo; entradas sem tamanho são explicitadas, sem estimativa. `SettingsScreen` carrega o resumo pelo serviço da biblioteca. Coberto por `storageUsage.test.ts` e pela validação web completa.
