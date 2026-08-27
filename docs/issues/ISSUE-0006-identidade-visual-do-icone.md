# ISSUE-0006 — Substituir e preparar alternativas do ícone do app

- Status: em validação
- Prioridade: média
- Área provável: design / Android
- Registrada em: 2026-08-26

## Relato

O ícone atual do aplicativo não representa bem a identidade visual desejada.

## Evidências

- `design/ChatGPT Image 26 de ago. de 2026, 15_43_03.png`: proposta de nota musical neon.
- `design/ChatGPT Image 26 de ago. de 2026, 15_43_11.png`: três alternativas reunidas em uma imagem: vinil (esquerda), play/equalizador (centro) e fones/equalizador (direita).

## Resultado esperado

Separar as três alternativas da imagem de referência em assets individuais para avaliação, escolher uma direção e gerar o conjunto de ícones Android necessário, incluindo variações adaptativas.

## Implementação

As alternativas foram preparadas em `src-tauri/icons/candidates/`: `vinyl.png`, `play-equalizer.png` e `headphones-equalizer.png`. A opção fones/equalizador foi aprovada, definida como ícone de bundle e regenerada para desktop, iOS e Android. Falta validar no launcher claro e escuro em aparelho.

## Critérios de aceite

- [x] Três alternativas são separadas sem perder proporção ou fundo.
- [x] Uma opção é aprovada explicitamente antes de substituir o ícone de produção.
- [x] Ícones adaptativos Android incluem foreground/background e resolução adequada.
- [ ] Ícone validado em tema claro/escuro e na grade de aplicativos.
