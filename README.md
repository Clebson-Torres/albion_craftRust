# Albion Crafting Overlay

Overlay desktop em Rust + Tauri para consultar oportunidades de crafting no Albion Online sem sair do jogo.

Versao atual do app: `0.1.1`

## O que a V2 faz

- mostra um top de oportunidades para `bags` e `capes`
- permite trocar entre `west`, `east` e `europe`
- compara venda entre cidades e `Black Market`
- calcula custo de craft a partir dos materiais suportados
- considera modo de compra, transporte, premium e orcamento
- exibe lucro por unidade, lucro total, capital usado, taxas e nivel de confianca
- mostra icones do item e dos materiais para facilitar a compra
- atualiza os dados em background
- reutiliza o ultimo ranking valido quando a API falha
- abre com foco em overlay lateral esquerda para uso ao lado do market
- lembra a ultima posicao da janela depois que voce mover a app

## Como rodar

```powershell
npm install
npm run tauri dev
```

Hotkey atual:

- `Ctrl+Shift+A` para alternar a visibilidade da janela

## Layout pensado para o jogo

- primeira abertura no lado esquerdo da tela
- lista curta de oportunidades na parte superior
- detalhe operacional na parte inferior
- centro da tela livre para a loja
- lado direito livre para o inventario

## Build local

```powershell
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
```

## Fonte de dados

O projeto usa a API publica do Albion Online Data Project:

- [Albion Online Data API](https://www.albion-online-data.com/api/)

Os icones sao carregados a partir do render oficial do Albion:

- [Albion Render Service](https://render.albiononline.com/)

## Limitacoes atuais

- o catalogo inicial cobre apenas `T4` a `T6` de bags e capes
- as receitas suportadas ainda sao curadas manualmente
- a API nao fornece um ranking direto de "mais vendidos", entao a confianca usa heuristicas
- a cidade atual ainda e informada manualmente pelo jogador
- ainda nao existe tabela real de rotas por cidade, apenas penalidade de transporte configuravel
- a app nao tenta ler nada do cliente do jogo

## CI e releases

O repositorio inclui workflows de GitHub Actions para:

- validacao em `ubuntu-latest` e `windows-latest`
- build do frontend
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- release com bundles Windows (`.msi` e instalador `.exe`/NSIS) e Linux (`.AppImage` e `.deb`)

Os metadados de versao do app ficam alinhados entre:

- `package.json`
- `Cargo.toml`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

## Proximos passos naturais

- ampliar catalogo e receitas suportadas
- adicionar tabela de custo por rota entre cidades
- incluir filtros por tier/encantamento
- adicionar ordenacao alternavel por lucro unitario, lucro total e retorno sobre capital
