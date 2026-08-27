# Backlog local de issues

Este diretório é o registro local de problemas encontrados no Rigbyfy enquanto o GitHub Issues não está conectado. Ele é versionado e deve ser tratado como a fonte de verdade para bugs, regressões, melhorias e dívidas técnicas ainda abertas.

## Fluxo

1. Registre todo relato em `ISSUES.md` e crie um arquivo a partir de `TEMPLATE.md`.
2. Use o próximo identificador sequencial: `ISSUE-0001-resumo-curto.md`.
3. Preserve evidências úteis: passos, versão, logs filtrados e screenshots sem dados privados.
4. Classifique prioridade e área provável, mas deixe claro quando forem hipóteses.
5. Não corrija automaticamente uma issue apenas por registrá-la; a implementação requer solicitação ou aprovação explícita.
6. Ao entregar uma correção, atualize a issue com arquivos alterados, validações executadas e o resultado. Mova o status no índice para `resolvida` somente após validação proporcional ao risco.

## Status e prioridade

- Status: `aberta`, `em análise`, `em andamento`, `bloqueada`, `resolvida`, `não reproduzida`.
- Prioridade: `crítica`, `alta`, `média`, `baixa`.

## Regras para agentes

Um agente de triagem deve transformar relatos informais em registros objetivos, sem inventar evidências. Ele pode consultar logs e o código para sugerir a área provável, mas deve distinguir fato de hipótese. Nunca inclua tokens, chaves, caminhos pessoais completos ou conteúdo privado da biblioteca nos arquivos de issue.

Quando o GitHub Issues estiver disponível, estes arquivos podem ser migrados preservando o identificador local no corpo da issue remota.
