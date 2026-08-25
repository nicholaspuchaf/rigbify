# Pasta `design`

## Escopo

Esta pasta guarda referências visuais fornecidas para o produto e especificações textuais derivadas delas. Ela descreve intenção de UX, hierarquia, componentes, estados e ações antes da implementação em Svelte. Não contém código executável nem é empacotada automaticamente no aplicativo.

As imagens são inspiração, não contratos pixel-perfect. A implementação deve preservar identidade escura, hierarquia e localização geral das ações, mas pode adaptar espaçamento, tipografia e componentes para acessibilidade, consistência e diferentes tamanhos de tela.

## Dependências e consumidores

- Não possui dependência de runtime.
- É consumida por quem implementa telas em `src/screens` e componentes em `src/components`.
- Deve ser lida em conjunto com [`../src/README.md`](../src/README.md) e [`../AGENTS.md`](../AGENTS.md).
- Os documentos Markdown podem mencionar entidades e casos de uso que dependem dos serviços TypeScript e do núcleo Rust, mas não devem definir detalhes incompatíveis com esses contratos.

## Inventário detalhado

| Referência | Especificação | Responsabilidade |
| --- | --- | --- |
| `home.jpeg` | `home.md` | Home, saudação, chips, recomendações, seções e mini player |
| `search.jpeg` | `search.md` | Busca, campo, gêneros/atalhos, resultados e estados vazios |
| `myPlace.jpeg` | `myPlace.md` | Biblioteca do usuário, filtros, listas e ações das músicas |
| `playerMusica.jpeg` | `playerMusica.md` | Player completo, capa, progresso, controles e ações |
| `config.jpeg` | `config.md` | Configurações, agrupamento e linhas de preferência |
| `playlist_home.jpeg` | `playlist_home.md` | Detalhe de playlist, resumo, ações e faixas |
| `playlist_adicionar_musica.jpeg` | `playlist_adicionar_musica.md` | Busca, seleção e adição em lote de músicas |
| `playlist_ordenar.jpeg` | `playlist_ordenar.md` | Escolha do critério de ordenação e ordem personalizada |
| `edit_playlist.jpeg` | `edit_playlist.md` | Edição de nome, descrição e capa da playlist |

Cada `.jpeg` é a evidência visual original. O `.md` de mesmo nome é a interpretação implementável: ele deve explicar composição, botões, comportamento, estados e critérios de aceitação. Em conflito entre referência visual e comportamento já validado pelo produto, registre a decisão no Markdown em vez de esconder a divergência.

## Fluxo para uma tela

1. Identifique a referência e leia seu `.md` completo.
2. Compare com componentes e tokens existentes no frontend.
3. Liste estados além da imagem estática: carregando, vazio, erro, desabilitado, confirmação e sucesso.
4. Reuse componentes existentes; crie um novo apenas quando representar um conceito recorrente.
5. Encaminhe efeitos para serviços, nunca diretamente do markup para APIs nativas.
6. Verifique navegação por teclado, nomes acessíveis, contraste, áreas de toque e textos longos.
7. Atualize a especificação se a implementação aprovada mudar o comportamento.

## Padrão para novas referências

- Use nomes descritivos em `snake_case` para novos pares; nomes históricos existentes são preservados.
- Adicione a imagem original sem editar ou sobrescrever silenciosamente a referência anterior.
- Crie um Markdown de mesmo nome com: objetivo, estrutura visual, ações, estados, dados necessários, acessibilidade, responsividade e critérios de aceitação.
- Informe se uma ação é visual, local, persistida em SQLite ou nativa de plataforma.
- Não coloque segredos, dados pessoais reais, músicas protegidas ou capturas com informações sensíveis.
- Não copie CSS, código Svelte ou detalhes frágeis de implementação para esta pasta.

## Limites

- `design/` especifica o produto; `src/` implementa a experiência.
- Decisões de schema e regras de negócio pertencem a `src-tauri/`.
- Decisões específicas de Media3/Android pertencem a `plugins/`.
- Ativos realmente servidos ao app pertencem a `public/` ou a uma futura pasta de assets do frontend, não aqui.

