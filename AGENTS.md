# AGENTS.md — Diretrizes do Rigbyfy

Este documento define as regras para qualquer pessoa ou agente que trabalhe neste repositório. Leia-o antes de alterar código. As regras valem para todo o projeto, salvo quando um `AGENTS.md` mais específico existir em uma subpasta.

## 1. Visão do produto

Rigbyfy é um player de músicas locais, offline e multiplataforma para Android, iOS e desktop.

Princípios do produto:

- A biblioteca e os arquivos pertencem ao usuário e permanecem no dispositivo.
- A reprodução deve funcionar em segundo plano e com a tela bloqueada.
- O aplicativo deve funcionar sem internet depois de instalado.
- A experiência deve ser rápida, acessível e consistente.
- O aplicativo não oferece catálogo nem download de conteúdo protegido.

Leia também o `README.md` antes de implementar novas funcionalidades. Ele contém arquitetura, roadmap e instruções de ambiente.

## 2. Stack oficial

- Interface: Svelte 5 com TypeScript
- Build web: Vite
- Pacotes e scripts JavaScript: Bun
- Aplicação: Tauri 2
- Núcleo compartilhado: Rust
- Persistência: SQLite
- Android: Kotlin com Media3/ExoPlayer
- iOS: Swift com AVFoundation

Não adicione outro framework de interface, banco de dados ou mecanismo de reprodução sem justificar a necessidade e documentar a decisão.

## 3. Arquitetura e responsabilidades

Cada camada deve ter responsabilidade clara:

- Svelte cuida da apresentação, interação e estado visual.
- Stores e serviços TypeScript coordenam casos de uso da interface.
- Tauri commands e events formam a fronteira entre interface e código nativo.
- Rust cuida de regras compartilhadas, biblioteca, arquivos, metadados e persistência.
- Kotlin e Swift cuidam somente das integrações específicas das plataformas.
- SQLite guarda metadados, playlists, favoritos, histórico, fila e configurações.
- Arquivos de áudio permanecem no sistema de arquivos; não devem ser copiados para `localStorage`.

Evite colocar regras de negócio em componentes Svelte, comandos Tauri ou controladores nativos. Esses pontos devem validar entradas e delegar o trabalho para serviços focados.

## 4. Organização de pastas desejada

À medida que o projeto crescer, mantenha esta estrutura:

```text
src/
  components/       componentes reutilizáveis
  features/         funcionalidades organizadas por domínio
  routes/           telas e navegação
  stores/           estado compartilhado
  services/         acesso à ponte Tauri e serviços externos
  types/            tipos TypeScript compartilhados
  utils/            funções puras e genéricas
  assets/           imagens e recursos da interface

src-tauri/src/
  commands/         adaptadores dos comandos Tauri
  domain/           entidades e regras de negócio
  services/         casos de uso
  repositories/     contratos e implementações de persistência
  database/         conexão, migrations e modelos SQLite
  media/            contratos do player
  platform/         integrações específicas quando necessárias
  errors/           erros tipados do domínio

tests/              testes de integração e fixtures
docs/               decisões arquiteturais e documentação técnica
```

Organize funcionalidades completas por domínio quando isso reduzir dependências cruzadas. Não crie pastas genéricas cheias de arquivos sem relação entre si.

## 5. Princípios obrigatórios de código

### Baixo acoplamento

- Dependa de interfaces e contratos, não de implementações concretas.
- Não permita que componentes conheçam detalhes de SQLite, Kotlin, Swift ou caminhos internos.
- Encapsule APIs do Tauri em serviços TypeScript tipados.
- Mantenha o domínio Rust independente da interface e, quando possível, do Tauri.

### Alta coesão e escopo limitado

- Cada módulo, componente e função deve ter uma responsabilidade principal.
- Prefira funções pequenas, previsíveis e fáceis de testar.
- Variáveis devem existir no menor escopo necessário.
- Evite estado global; use stores apenas para estado realmente compartilhado.
- Divida arquivos quando misturarem responsabilidades ou se tornarem difíceis de navegar.

### Não repetir código

- Extraia comportamento repetido para funções, componentes ou serviços reutilizáveis.
- Não crie abstrações prematuras para uma única ocorrência simples.
- Antes de criar uma nova utilidade, procure uma implementação existente.
- Uma abstração deve representar um conceito real do domínio, não apenas reduzir linhas.

### Simplicidade

- Escolha a solução mais simples que preserve os requisitos e a arquitetura.
- Evite dependências para tarefas pequenas que possam ser implementadas com segurança.
- Não antecipe funcionalidades sem requisito atual.
- Remova código morto, flags expiradas e comentários desatualizados.

## 6. Padrões por linguagem

### TypeScript e Svelte

- TypeScript estrito é obrigatório; não use `any` sem justificativa documentada.
- Prefira tipos de domínio explícitos e uniões discriminadas.
- Componentes devem receber dados por props e emitir intenções, não acessar persistência diretamente.
- Preserve a reatividade idiomática do Svelte 5.
- Todo elemento interativo deve ser acessível por teclado e possuir nome acessível.
- Não ignore erros de promises.
- Escape e valide dados vindos de arquivos antes de exibi-los.

### Rust

- Código deve passar por `cargo fmt` e `cargo clippy` sem novos avisos.
- Não use `unwrap()` ou `expect()` em fluxos recuperáveis de produção.
- Modele falhas com erros tipados e mensagens úteis.
- Separe domínio, persistência e adaptadores Tauri.
- Evite `unsafe`; se indispensável, documente invariantes e cubra com testes.
- Operações pesadas e de I/O não devem bloquear a thread da interface.

### Kotlin e Swift

- Mantenha APIs do plugin pequenas, versionadas e equivalentes entre plataformas.
- Isole Media3 e AVFoundation atrás de um contrato comum.
- Respeite o ciclo de vida, Audio Focus/Audio Session e interrupções do sistema.
- Não replique regras de biblioteca ou playlists nas implementações nativas.
- Eventos enviados ao Svelte devem possuir payload tipado e estável.

## 7. Contratos entre camadas

- Comandos Tauri devem aceitar e retornar estruturas serializáveis e tipadas.
- Valide todas as entradas no limite nativo; nunca confie somente na interface.
- Use nomes consistentes para comandos, eventos e payloads.
- Mudanças incompatíveis em contratos exigem migração e documentação.
- O player deve expor uma máquina de estados clara, como `idle`, `loading`, `playing`, `paused`, `stopped` e `error`.
- Evite eventos excessivos; envie somente mudanças de estado relevantes.

## 8. Persistência e migrations

- Toda mudança no schema SQLite exige migration incremental.
- Migrations aplicadas nunca devem ser reescritas; crie uma nova.
- Migrations devem ser idempotentes ou executadas de forma transacional.
- Nunca armazene áudio binário no SQLite ou `localStorage` sem uma decisão arquitetural explícita.
- Caminhos de arquivos devem ser normalizados e validados.
- Exclusão de registros e arquivos deve ser segura e, quando possível, recuperável.
- Teste upgrades partindo de bancos de versões anteriores.

## 9. Reprodução e plataformas

- Reprodução em segundo plano deve usar Media3 no Android e AVFoundation no iOS.
- Não trate `<audio>` na WebView como player final de produção.
- Estado da interface deve refletir o player nativo, que é a fonte de verdade da reprodução.
- Trate chamadas, perda de foco, fones desconectados, Bluetooth, economia de bateria e encerramento do processo.
- Notificação e tela bloqueada devem mostrar dados coerentes com a faixa atual.
- Código específico de plataforma não deve vazar para componentes Svelte.

## 10. Testes

Toda correção de bug deve incluir um teste que falhe antes da correção, quando tecnicamente possível. Toda funcionalidade nova deve incluir testes proporcionais ao risco.

Pirâmide de testes:

- Testes unitários para funções, domínio, stores e casos de uso.
- Testes de componente para comportamento visual e acessibilidade.
- Testes de integração para comandos Tauri, repositories e SQLite.
- Testes ponta a ponta para fluxos críticos: importar, tocar, pausar, trocar faixa e recuperar estado.
- Testes em aparelho real para áudio em segundo plano, tela bloqueada, Bluetooth e interrupções.

Os testes devem ser determinísticos, independentes e rápidos. Não dependa de ordem de execução, horário real, rede ou biblioteca pessoal do desenvolvedor. Use fixtures pequenas e legalmente distribuíveis.

Antes de concluir uma mudança, execute o que for aplicável:

```bash
bun run lint
bun run check
bun run test
bun run build
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --no-default-features --lib -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib
```

Se algum comando ainda não existir, crie-o ao introduzir a respectiva ferramenta. Se não puder executar um teste por limitação do ambiente, documente claramente o que ficou pendente.

## 11. Segurança e privacidade

- Solicite apenas permissões indispensáveis e no momento do uso.
- Mantenha capacidades e escopos do Tauri no princípio do menor privilégio.
- Nunca registre caminhos pessoais completos, tokens ou conteúdo da biblioteca em logs de produção.
- Não execute comandos de shell construídos com dados do usuário.
- Valide extensões, MIME types, caminhos e tamanhos antes de processar arquivos.
- Dependências novas devem ter manutenção ativa, licença compatível e necessidade clara.
- Não adicione telemetria, analytics ou comunicação de rede sem consentimento e documentação.

## 12. Desempenho

- Listas grandes devem usar paginação, virtualização ou carregamento incremental.
- Leitura de metadados e capas deve ocorrer fora da thread da interface.
- Capas devem ser redimensionadas e armazenadas em cache; não carregue imagens enormes na memória.
- Evite renderizações e eventos de progresso desnecessariamente frequentes.
- Meça antes de otimizar e registre o cenário usado na medição.

## 13. Documentação de mudanças

Toda mudança relevante deve atualizar pelo menos um destes locais:

- `README.md` para uso, instalação, arquitetura ou recursos visíveis.
- `CHANGELOG.md` para alterações entregues ao usuário, quando criado.
- `docs/adr/` para decisões arquiteturais importantes.
- Comentários no código somente para explicar intenção, restrições ou decisões não óbvias.

Não use comentários para repetir o que o código já diz. APIs públicas e contratos entre camadas devem ser documentados.

Uma decisão arquitetural deve registrar contexto, opções consideradas, decisão e consequências.

## 14. Commits e revisões

- Faça mudanças pequenas e focadas.
- Não misture refatoração ampla com funcionalidade sem necessidade.
- Use mensagens de commit claras, preferencialmente no padrão Conventional Commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:` e `chore:`.
- Não inclua arquivos gerados, segredos, builds ou configurações pessoais.
- Preserve mudanças existentes de outros colaboradores.
- Revise o diff completo antes de concluir.

## 15. Dependências

Antes de adicionar uma dependência:

1. Confirme que o projeto realmente precisa dela.
2. Verifique manutenção, licença, tamanho e suporte mobile.
3. Prefira plugins oficiais do Tauri quando adequados.
4. Confirme suporte a Android e iOS; suporte apenas a desktop não é suficiente.
5. Registre dependências que impactem a arquitetura no README ou em um ADR.

Não atualize versões principais sem ler as notas de migração e executar a suíte relevante.

Use Bun para instalar dependências e executar scripts. Não use APIs exclusivas como `Bun.file`, `Bun.serve`, `bun:sqlite` ou `bun:test`; o projeto deve continuar compatível com o ecossistema Node para permitir uma migração futura de gerenciador. O único lockfile JavaScript aceito é `bun.lock`.

Os hooks do Lefthook são obrigatórios. O `pre-commit` executa verificações rápidas nos arquivos alterados e o `pre-push` executa a validação completa. Não use `--no-verify` sem uma exceção explícita e documentada.

## 16. Critérios de conclusão

Uma tarefa só está concluída quando:

- O comportamento solicitado está implementado sem quebrar fluxos existentes.
- A solução respeita as fronteiras arquiteturais.
- Código duplicado e código morto foram evitados.
- Tipos, tratamento de erros e estados vazios foram considerados.
- Testes relevantes foram criados e executados.
- Formatação, análise estática e build passam.
- Acessibilidade e comportamento mobile foram verificados.
- README, roadmap, changelog ou ADR foram atualizados quando necessário.
- Limitações e validações não executadas foram informadas claramente.

## 17. Regras para agentes automatizados

- Inspecione o repositório e este arquivo antes de editar.
- Não altere a stack ou a arquitetura silenciosamente.
- Não apague nem sobrescreva trabalho alheio para facilitar uma mudança.
- Não faça operações destrutivas sem autorização explícita.
- Prefira alterações pequenas, verificáveis e reversíveis.
- Corrija a causa do problema, não apenas o sintoma.
- Não declare sucesso sem evidência de build, teste ou inspeção apropriada.
- Informe comandos não executados e o motivo.
- Ao descobrir uma regra duradoura importante, proponha ou faça uma atualização neste documento.
