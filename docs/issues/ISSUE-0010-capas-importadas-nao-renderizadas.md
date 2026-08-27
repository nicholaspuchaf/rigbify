# ISSUE-0010 — Capas importadas não são renderizadas

- Status: em validação
- Prioridade: média
- Área provável: interface
- Registrada em: 2026-08-27

## Relato

Após importar um bundle com arquivos de capa, as músicas aparecem no catálogo, mas a interface mostra somente a capa gráfica de fallback.

## Resultado esperado

Faixas com `artworkUri` válido exibem a imagem correspondente; faixas sem imagem, ou cuja leitura falhar, mantêm o fallback visual.

## Passos para reproduzir

1. Enviar um bundle com `artwork/` e referências `artwork` no manifesto.
2. Importar a pasta do bundle no Android.
3. Abrir a biblioteca ou o player.

## Ambiente

- Plataforma/dispositivo: Android
- Origem dos dados: bundle local importado via seletor de pasta

## Evidências

- O bundle transferido contém 542 imagens de capa e o manifesto faz referência a elas.
- O plugin Android converte a referência do manifesto em `artworkUri` e o command Rust a persiste em SQLite.
- `trackMapper` mantém `artworkUri`, mas `Artwork.svelte` não recebia nem renderizava essa propriedade.

## Análise inicial

Fato confirmado: o primeiro ajuste conectou a URI à apresentação, mas as capas do bundle usam `content://` e a WebView não consegue carregá-las diretamente. As capas precisam ser materializadas no cache privado. A tentativa intermediária de convertê-las em `asset://` foi incompatível com o Media3 e não resolveu a renderização na WebView; a URI persistida deve permanecer `file://`. A leitura direta de `file://` também é bloqueada pela WebView, embora o arquivo exista e o `<img>` tente carregá-lo.

## Critérios de aceite

- [x] Faixas com URI de capa passam essa URI aos componentes visuais de faixa e player.
- [x] Capas de bundle são materializadas no cache privado e preservam uma URI `file://` utilizável pelo player.
- [x] A interface recebe somente uma URL `data:` gerada por comando nativo a partir de uma capa autorizada do cache privado.
- [x] Falhas de carregamento mantêm o fallback acessível.
- [ ] A renderização é validada no aparelho Android após a instalação da build.
- [x] Documentação atualizada quando aplicável.

## Resolução

Causa: `Artwork.svelte` recebia somente a tonalidade visual e não renderizava a URI de capa já presente no modelo `Track`. Depois de conectada, a URI `content://` do Storage Access Framework continuou inacessível diretamente pela WebView.

Alterações: `Artwork.svelte` passou a renderizar imagem com fallback no evento de erro; os consumidores de faixas (lista, grade, player, miniplayer e edição/adição de playlist) agora passam `artworkUri`. O plugin Android copia capas de bundle para o cache privado com limite de 10 MiB e preserva a URI `file://` desses arquivos, que também é a forma aceita pelo Media3 para a notificação e a tela bloqueada. Para a WebView, `load_artwork_data_uri` aceita somente arquivos canônicos dentro de `cache/artwork/`, limita a leitura a 10 MiB e retorna uma URL `data:` com o MIME detectado; o frontend mantém essa representação em cache por URI.

Validações: `bun run validate` (lint, tipos, 17 testes e build web), `cargo fmt --check`, `cargo test --no-default-features --lib` (13 testes) e build Android de depuração concluídos com sucesso. A validação visual no Android continua pendente.
