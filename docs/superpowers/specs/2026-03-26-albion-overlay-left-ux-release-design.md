# Albion Overlay Left UX + Release Design

Data: 2026-03-26

## Objetivo

Evoluir a overlay do Albion para um formato mais adequado ao uso real dentro do jogo:

- corrigir a versão exibida nos artefatos e releases
- estabilizar o pipeline de build/release
- abrir a janela no lado esquerdo na primeira execução
- lembrar a ultima posicao depois disso
- redesenhar a interface para um formato lateral esquerdo com `lista curta + detalhe expansivel`

## Problemas atuais

1. A release `v0.1.1` foi publicada com artefatos marcados como `0.1.0` porque a versao do app ainda esta fixa em `package.json`, `src-tauri/Cargo.toml` e `src-tauri/tauri.conf.json`.
2. O pipeline de release ainda tem warning de compatibilidade futura no GitHub Actions por conta das actions rodando em Node 20.
3. A UI atual funciona, mas ainda parece um dashboard amplo. Isso compete demais com o market do jogo, que fica no centro da tela.
4. A janela nao nasce orientada para o fluxo real de uso: overlay esquerda, market centro, inventario direita.

## Abordagem escolhida

Opcao 2: `Overlay lateral dedicada`

Essa abordagem corrige versao/build e redesenha a app especificamente para o uso no lado esquerdo da tela, mantendo a app enxuta e operacional sem virar um painel pesado.

## Janela

### Posicionamento

- Na primeira abertura, a janela deve aparecer no lado esquerdo da tela.
- Depois disso, a app deve lembrar a ultima posicao conhecida.
- O comportamento nao deve prender a janela para sempre; o usuario pode reposicionar, e essa posicao passa a ser a nova referencia.

### Formato

- A janela deve ser mais estreita e alta do que a versao atual.
- O objetivo e reduzir interferencia visual no centro da tela.
- A largura ideal da primeira versao deve priorizar leitura rapida sem invadir o market.

## Layout escolhido

Baseado na opcao visual `B. Equilibrada`.

### Estrutura

1. Topo compacto
   - nome da app
   - status dos dados
   - acao de atualizar
   - indicador da hotkey

2. Barra de controles compacta
   - servidor
   - cidade atual
   - premium
   - modo de compra
   - transporte
   - orcamento

3. Lista curta de oportunidades
   - parte superior do corpo
   - foco em leitura e selecao rapida
   - poucos itens visiveis com bom contraste

4. Detalhe operacional expansivel
   - parte inferior
   - mostra materiais, taxas, capital e lucro total
   - serve como area de execucao do item selecionado

## UX da lista

Cada item da lista deve mostrar apenas o necessario:

- icone do item
- nome
- lucro por unidade
- lucro total com o orcamento
- destino de venda
- confianca

O layout da lista deve priorizar:

- linhas compactas
- boa separacao visual entre itens
- destaque claro do item selecionado

## UX do detalhe

O detalhe deve favorecer execucao, nao exploracao longa.

### Informacoes principais

- lucro por unidade
- lucro total
- custo de craft
- venda
- quantidade maxima
- capital usado
- capital restante
- taxa de compra
- taxa de venda
- impacto total de taxas

### Materiais

- icones visiveis
- quantidade
- cidade de compra
- preco unitario
- custo total por material

## Release e versionamento

### Versao

O numero da versao deve ficar alinhado entre:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

O objetivo e fazer com que a tag, a release e os bundles publiquem a mesma versao.

### GitHub Actions

Atualizar o workflow para reduzir risco futuro com runtime de actions:

- revisar `actions/checkout`
- revisar `actions/setup-node`
- manter pipeline de CI/CD funcional para Windows e Linux

## Testes e validacao

### Validacao tecnica

- `npm run build`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

### Smoke manual

Verificar:

- primeira abertura no lado esquerdo
- reabertura mantendo a ultima posicao
- layout lateral legivel com market no centro
- selecao de item atualizando o detalhe
- versao correta no app e nos artefatos

## Novidades sugeridas para backlog

1. Modo `compacto/jogo`
   - densidade maior, menos texto, mais leitura rapida

2. Ordenacao alternavel
   - lucro total
   - lucro por unidade
   - retorno sobre capital

3. Favoritos
   - fixar certos itens ou familias no topo

4. Lista de compra
   - consolidar materiais do item selecionado em ordem de compra

5. Custos por rota
   - trocar penalidade fixa por tabela entre cidades

6. Transparencia configuravel
   - para ajustar convivencia com a interface do jogo
