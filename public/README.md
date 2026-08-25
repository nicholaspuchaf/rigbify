# Pasta `public`

## Escopo

`public/` contém arquivos estáticos que o Vite copia para a raiz do bundle sem transformação. Use-a somente quando o recurso precisa conservar nome e URL pública. Código TypeScript/Svelte e recursos que devem passar pelo pipeline de imports não pertencem aqui.

## Conteúdo atual

```text
public/
├── README.md
└── sw.js
```

### `sw.js`

Service Worker registrado por `src/main.ts` apenas em produção. Ele:

- usa o cache versionado `rigbyfy-v2`;
- pré-carrega `/`, `/index.html`, `/manifest.webmanifest` e `/icon.svg` durante `install`;
- remove caches Rigbyfy antigos no `activate`;
- intercepta as requisições `GET` vistas pelo escopo do worker;
- tenta a rede primeiro e atualiza o cache quando recebe uma resposta válida;
- usa a resposta em cache quando a rede falha.

Esse cache atende o shell web. Ele não armazena a biblioteca musical, URIs Android, metadados SQLite nem estado do player. No Tauri/Android, a persistência de domínio continua sendo responsabilidade do núcleo nativo.

## Dependências

- Vite copia os arquivos desta pasta no build.
- `src/main.ts` depende da URL `/sw.js` e da API `navigator.serviceWorker`.
- `sw.js` depende das APIs padrão Service Worker, Fetch e Cache Storage.
- Os caminhos pré-carregados dependem dos arquivos raiz `index.html`, `manifest.webmanifest` e `icon.svg`.

## Regras para alterações

- Leia [`../src/README.md`](../src/README.md) ao alterar comportamento consumido pelo frontend.
- Ao mudar os recursos ou a política de cache, incremente o nome do cache para invalidar versões antigas.
- O handler deve falhar de maneira previsível quando um recurso não estiver em cache.
- Não cacheie arquivos de áudio, `content:// URI`, respostas privadas ou dados pessoais sem uma decisão arquitetural e limites explícitos.
- Não inclua tokens, configuração pessoal ou segredos; tudo aqui é publicamente acessível no bundle web.
- Prefira imports do Vite para ativos com hash e otimização. Use `public/` quando URL estável for requisito.
- Teste o modo produção; o Service Worker não é registrado pelo fluxo normal de desenvolvimento.
- Verifique cuidadosamente mudanças offline para não servir HTML/API obsoletos como se fossem dados atuais.

## Como validar

```bash
bun run build
bun run preview
```

No navegador, confirme em DevTools que o worker foi registrado, que os quatro recursos do shell estão no cache correto e que uma atualização remove a versão anterior.
