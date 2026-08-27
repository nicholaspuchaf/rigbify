# ISSUE-0005 — Biblioteca não lista nem abre álbuns e artistas

- Status: em validação
- Prioridade: alta
- Área provável: biblioteca
- Registrada em: 2026-08-26

## Relato

As abas Álbums e Artistas da Biblioteca não mostram dados. Não existe fluxo para abrir um álbum/artista e listar suas músicas.

## Resultado esperado

- Álbums: listar álbuns agrupados pelos metadados, abrir um álbum e mostrar suas faixas.
- Artistas: listar artistas agrupados, abrir um artista e mostrar suas faixas/álbuns.

## Análise inicial

Fato: `LibraryScreen` filtra `collections`, que atualmente representa playlists; não há agregação de faixas por álbum ou artista nem tela de detalhes para esses grupos.

## Implementação

`libraryCollections.ts` agrupa as faixas reais por álbum e artista, em ordem `pt-BR`, e oferece rótulos previsíveis para metadados ausentes. `LibraryCollectionScreen` abre o grupo e apresenta somente as faixas correspondentes, com reprodução, favoritos e ações compartilhadas.

## Critérios de aceite

- [ ] Álbuns e artistas aparecem a partir da biblioteca local real.
- [ ] Grupos sem valor de metadado recebem tratamento consistente.
- [ ] Abrir álbum/artista mostra somente suas faixas e mantém mini-player/navegação.
- [ ] Agregação e ordenação possuem testes unitários.
