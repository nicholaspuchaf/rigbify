# ISSUE-0003 — Ordenação por título, artista e álbum não funciona na playlist

- Status: em validação
- Prioridade: alta
- Área provável: playlists
- Registrada em: 2026-08-26

## Relato

Na tela de uma playlist, as opções de classificar por título, artista ou álbum não produzem a ordenação esperada.

## Resultado esperado

Cada modo deve reordenar de forma determinística, respeitar acentos/case, persistir a preferência e atualizar a fila em reprodução sem perder a faixa atual.

## Análise inicial

Fato: a interface expõe os modos `title`, `artist` e `album` e delega a `playlistService.setSort`. Investigar a implementação Rust/SQLite do sort e o retorno usado para renderização.

## Implementação

`SqlitePlaylistRepository` passou a ordenar as faixas no domínio após carregar posição e data de adição. A chave textual normaliza case e diacríticos, trata metadados vazios por último e usa desempates estáveis. A preferência de sort continua persistida na playlist; a fila ativa é reconstruída pela UI após a mudança.

## Critérios de aceite

- [ ] Título, artista e álbum ordenam corretamente em ordem ascendente.
- [ ] Ordem personalizada e recentes continuam funcionando.
- [ ] A ordem persiste ao reabrir a playlist.
- [ ] Testes unitários cobrem valores vazios, acentos e empates.
