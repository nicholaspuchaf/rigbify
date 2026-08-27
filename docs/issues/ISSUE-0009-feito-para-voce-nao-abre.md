# ISSUE-0009 — Cards de Feito para você não abrem a playlist

- Status: resolvida
- Prioridade: alta
- Área provável: Home / navegação
- Registrada em: 2026-08-27

## Relato

Os cards da seção Feito para você não respondem à seleção e não abrem a playlist associada.

## Resultado esperado

Selecionar um card deve chamar o mesmo fluxo de abertura de playlist usado nos atalhos da Home.

## Evidências

- `HomeScreen.svelte` renderiza os cards como `article`, sem callback de abertura.

## Critérios de aceite

- [x] Todo card é acessível por teclado e possui nome acessível.
- [x] Selecionar o card chama a intenção de abrir sua coleção.
- [x] Há teste proporcional ao comportamento.

## Resolução

Os cards foram convertidos em botões com nome acessível e chamam `oncollection`, a mesma intenção usada pelos atalhos da Home. A ordenação de mais tocadas foi extraída para `homeRecommendations.ts` e coberta por teste unitário; lint, tipos, testes e build web passaram.
