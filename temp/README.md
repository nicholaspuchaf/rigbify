# Importação de playlists CSV

Esta pasta contém exports CSV do Spotify e uma ferramenta local para criar um bundle de importação do Rigbyfy. O bundle preserva o vínculo entre playlist, metadados e arquivos de áudio sem acessar o banco SQLite do telefone diretamente.

Os CSVs, o bundle gerado e `.venv` são locais e ficam ignorados pelo Git. O script, seus testes e o lockfile do `uv` são versionados.

Você pode guardar a chave da API em `.env` (arquivo ignorado) usando [`.env.example`](.env.example) como modelo:

```bash
cp .env.example .env
# edite .env e preencha YOUTUBE_API_KEY
```

O script lê essa variável sem exibi-la e sem substituir uma variável já exportada no shell.

## Fluxo

1. Gere o bundle e a fila de fontes:

   ```bash
   cd ~/Documents/rigbyfy/temp
   uv run rigbyfy_import.py prepare
   ```

2. Há dois jeitos de preencher URLs:

   - Preencha diretamente `Rigbyfy-import/sources.csv` com URLs HTTPS de conteúdo que você tem autorização para baixar; ou
   - Use a YouTube Data API para gerar candidatos e revise-os manualmente. Crie uma chave de API no seu projeto Google, mantenha-a fora do Git e execute, por exemplo:

     ```bash
     export YOUTUBE_API_KEY="sua-chave-local"
     uv run rigbyfy_import.py discover \
       --playlist "Rock brabo" \
       --authorize "Rock brabo" \
       --limit 10 \
       --interactive
     ```

     No modo `--interactive`, o terminal mostra os candidatos um a um (URL, título, canal, duração, pontuação e alertas). Digite `1`, `2` ou `3` para escolher; `s` pula a faixa e `q` encerra sem perder as escolhas anteriores. A URL escolhida é aplicada imediatamente em `sources.csv`, mas nenhum download é iniciado. O arquivo `Rigbyfy-import/review-<id-da-playlist>.csv` também é criado como registro.

     Sem `--interactive`, nenhum URL é aplicado. Revise o CSV, marque a linha escolhida com `approved=sim` e copie seu `candidate_url` para `approved_url`. Então aplique apenas as escolhas explícitas:

     ```bash
     uv run rigbyfy_import.py apply-review \
       --review Rigbyfy-import/review-<id-da-playlist>.csv
     ```

     A descoberta é limitada a dez faixas pendentes por execução por padrão, para uso gradual e controle de quota. Ao rodar novamente, faixas que já possuem URL em `sources.csv` são puladas e o script segue para as próximas. A pontuação só serve para priorizar revisão: rejeite candidatos incorretos, versões ao vivo, covers e remixes.

     Para lotes previamente autorizados, `--auto-select` grava automaticamente o candidato de maior pontuação. Use-o somente quando aceitar essa decisão automática; `--interactive` continua sendo a opção recomendada para máxima conferência.

3. Baixe uma playlist por vez. A confirmação repete o nome da playlist e vale apenas para aquela execução:

   ```bash
   uv run rigbyfy_import.py download \
     --playlist "Rock brabo" \
     --authorize "Rock brabo" \
     --limit 1
   uv run rigbyfy_import.py verify
   ```

4. Se o teste estiver correto, retome a mesma playlist ou escolha outra nos próximos dias. Faixas já baixadas são ignoradas automaticamente:

   ```bash
   uv run rigbyfy_import.py download \
     --playlist "Rock brabo" \
     --authorize "Rock brabo"
   uv run rigbyfy_import.py verify
   ```

5. Quando quiser transferir o bundle atualizado ao Android:

   ```bash
   uv run rigbyfy_import.py push
   ```

O bundle é enviado para `/sdcard/Download/Rigbyfy-import`.

## Estrutura gerada

```text
Rigbyfy-import/
├── manifest.json  # playlists, metadados e referência de cada arquivo
├── sources.csv    # fila revisável de URLs autorizadas
└── audio/         # arquivos baixados
```

`manifest.json` é escrito de forma atômica após cada download. A operação `push` exige `adb` disponível no PATH e um aparelho com depuração USB autorizada. Para tentar novamente uma faixa já baixada, passe `--retry` explicitamente.

Se executar `prepare` novamente, as URLs já preenchidas em `sources.csv` são preservadas. CSVs que não tenham o cabeçalho de exportação do Spotify são ignorados com um aviso.

## Limitação atual do aplicativo

O Rigbyfy ainda não possui a tela nativa que lê esse bundle e recria as playlists automaticamente. O bundle e o envio ADB estão prontos; o próximo passo no aplicativo é implementar o importador de `manifest.json`, que registrará os áudios na biblioteca e criará as playlists no SQLite por meio das APIs normais, sem escrever no banco diretamente.
