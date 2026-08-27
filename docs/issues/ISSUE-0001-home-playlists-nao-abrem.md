# ISSUE-0001 — Playlists não abrem a partir da Home

- Status: aberta
- Prioridade: alta
- Área provável: navegação / playlists
- Registrada em: 2026-08-26

## Relato

Selecionar uma playlist exibida na Home não abre sua tela de detalhes.

## Resultado esperado

Ao tocar em uma playlist na Home, o aplicativo deve abrir a mesma tela de playlist usada pela Biblioteca, preservando o histórico para o botão Voltar.

## Passos para reproduzir

1. Ter ao menos uma playlist disponível.
2. Abrir Início.
3. Tocar em uma coleção do tipo playlist.

## Análise inicial

Fato: `HomeScreen` recebe `oncollection={openPlaylist}`. A função atual de abertura só prossegue se o ID estiver na lista carregada de playlists; é necessário validar a origem/ID das coleções renderizadas na Home e o carregamento assíncrono da biblioteca.

## Critérios de aceite

- [ ] Toda playlist visível na Home abre seus detalhes.
- [ ] O botão Voltar retorna à Home.
- [ ] Cobertura automatizada do fluxo Home → Playlist → Voltar.
