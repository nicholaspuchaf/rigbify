# ISSUE-0007 — Ordenação de playlists não funciona

- Status: em validação
- Prioridade: média
- Área provável: biblioteca / playlists
- Registrada em: 2026-08-26

## Relato

Não é possível ordenar playlists na Biblioteca.

## Resultado esperado

O usuário deve poder escolher uma ordenação para a lista de playlists, no mínimo nome e data de criação/atualização, com critério visível e persistente.

## Análise inicial

Fato: o botão de ordenação da `LibraryScreen` está apenas visual; não possui callback nem estado para mudar a ordem. Diferenciar esta ordenação da ordenação de faixas dentro de uma playlist.

## Implementação

O botão abre uma folha de ações acessível com Recentes, Nome A–Z e Nome Z–A. A ordenação é determinística, com desempate por atualização/criação, e a preferência visual persiste sob `rigbyfy.playlist-sort`.

## Critérios de aceite

- [ ] O botão abre opções de classificação de playlists.
- [ ] A lista muda conforme o critério escolhido.
- [ ] A preferência persiste ao reiniciar o app.
- [ ] Testes cobrem critérios, ordem ascendente/descendente e empates.
