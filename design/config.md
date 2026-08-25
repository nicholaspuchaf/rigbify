# Tela de configurações

Referência visual: `config.jpeg`

## Objetivo

Reunir preferências do aplicativo, biblioteca, reprodução, aparência, armazenamento, privacidade e informações técnicas de maneira pesquisável e organizada.

## Direção visual

- Fundo quase preto e cabeçalho cinza-escuro ligeiramente destacado.
- Ícones simples em branco, sem caixas ao redor.
- Títulos brancos e descrições em cinza.
- Verde-lima em switches, seleção e ações positivas.
- Separação por espaçamento e títulos de seção, evitando excesso de divisórias.

## Estrutura

### Cabeçalho

- Botão voltar à esquerda.
- Título central `Configurações`.
- Busca à direita para filtrar opções.
- Cabeçalho fixo durante a rolagem.

### Resumo do aplicativo

- Substituir área de conta/premium por um resumo local do Rigbyfy.
- Mostrar quantidade de músicas, espaço utilizado e última atualização da biblioteca.
- Ação principal: `Atualizar biblioteca`.

### Grupos de configuração

#### Biblioteca

- Pastas monitoradas.
- Verificar novas músicas automaticamente.
- Metadados e capas.
- Remover referências de arquivos ausentes.

#### Reprodução

- Reprodução sem pausas.
- Autoplay da fila.
- Normalização de volume.
- Comportamento ao desconectar fones.
- Retomar última música ao abrir.

#### Áudio

- Qualidade e decodificação.
- Equalizador, quando disponível.
- Saída de áudio.
- Crossfade futuro.

#### Aparência

- Tema escuro, sistema ou alto contraste.
- Cor de destaque.
- Densidade das listas.
- Reduzir animações.

#### Notificações e tela bloqueada

- Habilitar controles multimídia.
- Exibir capa na tela bloqueada.
- Explicação e atalho para permissões do sistema quando necessário.

#### Armazenamento

- Espaço usado por capas e cache.
- Limpar cache de capas.
- Local do banco e diagnóstico sem revelar caminhos sensíveis.
- Exportar/importar playlists e configurações.

#### Privacidade

- Informar claramente que os dados permanecem no dispositivo.
- Gerenciar logs de diagnóstico.
- Excluir histórico de reprodução.

#### Sobre e suporte

- Versão do aplicativo e do núcleo nativo.
- Licenças de código aberto.
- Relatar problema e exportar diagnóstico com confirmação.

### Player e navegação

- Se houver música ativa, manter o miniplayer acima da navegação.
- A tela precisa reservar espaço inferior para não ocultar o último item.
- `Configurações` fica selecionado na navegação inferior.

## Padrão de item

- Ícone à esquerda.
- Área central com título e resumo do valor atual.
- Chevron para páginas internas, switch para opções booleanas ou valor à direita para seleção.
- Toda a linha é clicável, não somente o ícone ou texto.
- Opções perigosas usam confirmação e ficam em seção separada.

## Estados

- Busca ativa: mostrar apenas opções correspondentes e seus grupos.
- Sem resultado: mensagem com botão para limpar busca.
- Permissão negada: descrição específica e ação para abrir configurações do sistema.
- Operação em andamento: bloquear somente o item afetado.
- Sucesso ou erro: feedback curto, acessível e não intrusivo.

## Componentes Svelte sugeridos

- `SettingsHeader.svelte`
- `LibrarySummary.svelte`
- `SettingsSection.svelte`
- `SettingsRow.svelte`
- `SettingsSwitch.svelte`
- `SettingsSelect.svelte`
- `StorageUsage.svelte`
- `ConfirmationDialog.svelte`

## Regras arquiteturais

- A tela consome um serviço tipado de configurações.
- Preferências não devem ser espalhadas em componentes ou chaves literais.
- Permissões são verificadas pela camada nativa.
- Alterações sensíveis devem retornar sucesso ou erro antes de atualizar definitivamente a interface.

## Acessibilidade

- Switches devem anunciar nome, estado e descrição.
- Cabeçalhos de grupo usam hierarquia semântica.
- Valores atuais não podem depender apenas de cor.
- Confirmações devolvem foco ao item que abriu o diálogo.

## Critérios de aceite do protótipo

- Cabeçalho, resumo e todos os grupos principais visíveis.
- Busca filtra opções simuladas.
- Switches, seleções e diálogos possuem estados funcionais.
- Tela permanece utilizável com texto ampliado e entre 320 e 430 px.
